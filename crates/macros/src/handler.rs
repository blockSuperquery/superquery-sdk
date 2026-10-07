//! `#[handler]`.
//!
//! Leaves the function exactly as written and adds two items beside it:
//!
//! - `__sq_dispatch_<name>(payload: &[u8]) -> HandlerStatus`, which decodes
//!   the payload, runs the handler and reports errors to the host. Compiled
//!   on every target, so tests can call it directly.
//! - on `wasm32`, the `sq_handle_<name>(ptr, len) -> u32` export the node
//!   calls, which only moves the buffer and delegates to the dispatcher.

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::spanned::Spanned;
use syn::{FnArg, ItemFn, Safety};

pub(crate) fn expand(attr: TokenStream, function: ItemFn) -> syn::Result<TokenStream> {
    if !attr.is_empty() {
        return Err(syn::Error::new(
            attr.span(),
            "#[handler] takes no arguments; the manifest names the handler by function name",
        ));
    }

    let sig = &function.sig;
    if !sig.generics.params.is_empty() {
        return Err(syn::Error::new(
            sig.generics.span(),
            "a handler cannot be generic: the export needs one concrete input type",
        ));
    }
    if let Safety::Unsafe(token) = &sig.safety {
        return Err(syn::Error::new(
            token.span(),
            "a handler cannot be `unsafe`",
        ));
    }
    if let Some(abi) = &sig.abi {
        return Err(syn::Error::new(
            abi.span(),
            "write a plain Rust fn; #[handler] generates the extern export",
        ));
    }

    let mut inputs = sig.inputs.iter();
    let (Some(input), None) = (inputs.next(), inputs.next()) else {
        return Err(syn::Error::new(
            sig.inputs.span(),
            "a handler takes exactly one argument: the event, transaction or block",
        ));
    };
    let FnArg::Typed(input) = input else {
        return Err(syn::Error::new(
            input.span(),
            "a handler is a free function, not a method",
        ));
    };
    let input_ty = &input.ty;

    let name = &sig.ident;
    let dispatch = format_ident!("__sq_dispatch_{}", name);
    let export = format_ident!("sq_handle_{}", name);
    let call = if sig.asyncness.is_some() {
        quote!(#name(input))
    } else {
        quote!(::core::future::ready(#name(input)))
    };

    Ok(quote! {
        #function

        #[doc(hidden)]
        pub fn #dispatch(payload: &[u8]) -> ::superquery_sdk::__private::HandlerStatus {
            ::superquery_sdk::__private::dispatch(payload, |input: #input_ty| #call)
        }

        #[cfg(target_arch = "wasm32")]
        #[doc(hidden)]
        #[unsafe(no_mangle)]
        pub extern "C" fn #export(ptr: u32, len: u32) -> u32 {
            ::superquery_sdk::__private::run_export(ptr, len, #dispatch)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    fn expanded(function: ItemFn) -> String {
        expand(TokenStream::new(), function)
            .expect("expands")
            .to_string()
    }

    fn error(function: ItemFn) -> String {
        expand(TokenStream::new(), function)
            .expect_err("rejected")
            .to_string()
    }

    #[test]
    fn exports_the_symbol_the_runtime_expects_and_keeps_the_function() {
        let out = expanded(parse_quote! {
            pub async fn handle_transfer(event: EvmLog<Transfer>) -> Result<()> { Ok(()) }
        });
        assert!(out.contains("pub async fn handle_transfer"), "{out}");
        assert!(
            out.contains("extern \"C\" fn sq_handle_handle_transfer"),
            "{out}"
        );
        assert!(out.contains("unsafe (no_mangle)"), "{out}");
        assert!(out.contains("cfg (target_arch = \"wasm32\")"), "{out}");
        assert!(out.contains("fn __sq_dispatch_handle_transfer"), "{out}");
        assert!(out.contains("input : EvmLog < Transfer >"), "{out}");
    }

    #[test]
    fn a_sync_handler_is_wrapped_in_a_ready_future() {
        let out = expanded(parse_quote! {
            fn handle_block(block: EvmBlock) -> Result<()> { Ok(()) }
        });
        assert!(
            out.contains("future :: ready (handle_block (input))"),
            "{out}"
        );
    }

    #[test]
    fn rejects_signatures_the_runtime_cannot_call() {
        assert!(
            error(parse_quote! { async fn h() -> Result<()> { Ok(()) } }).contains("exactly one")
        );
        assert!(
            error(parse_quote! { async fn h(a: A, b: B) -> Result<()> { Ok(()) } })
                .contains("exactly one")
        );
        assert!(
            error(parse_quote! { async fn h<T>(a: T) -> Result<()> { Ok(()) } })
                .contains("generic")
        );
        assert!(
            error(parse_quote! { async fn h(&self) -> Result<()> { Ok(()) } })
                .contains("free function")
        );
        assert!(
            error(parse_quote! { unsafe fn h(a: A) -> Result<()> { Ok(()) } }).contains("unsafe")
        );
        assert!(
            error(parse_quote! { extern "C" fn h(a: A) -> Result<()> { Ok(()) } })
                .contains("plain Rust fn")
        );
    }

    #[test]
    fn rejects_arguments_to_the_attribute() {
        let err = expand(
            quote!(name = "x"),
            parse_quote! { fn h(a: A) -> Result<()> { Ok(()) } },
        )
        .unwrap_err()
        .to_string();
        assert!(err.contains("takes no arguments"), "{err}");
    }
}
