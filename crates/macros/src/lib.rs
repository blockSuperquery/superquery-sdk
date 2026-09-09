//! Procedural macros for mappings.
//!
//! Two macros, both deliberately thin:
//!
//! - `#[handler]` marks a mapping entry point. It will generate the WASM
//!   export symbol, payload deserialisation, error conversion and the ABI
//!   version assertion.
//! - `#[derive(SuperQueryEntity)]` implements the store traits for a generated
//!   entity struct.
//!
//! The guiding rule is that a developer should be able to read the expansion
//! and recognise their own code. Macro magic that makes a failing mapping
//! impossible to debug is worse than a little boilerplate.
//!
//! Scaffold for Milestone 7: today both macros pass the item through
//! unchanged so projects compile end-to-end while the ABI is still settling.

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, ItemFn, parse_macro_input};

/// Mark an async function as a mapping handler.
///
/// ```ignore
/// #[handler]
/// pub async fn handle_transfer(event: EvmLog<Transfer>) -> Result<()> { .. }
/// ```
///
/// Milestone 7 adds the generated export: `sq_handle_<name>`, the payload
/// decode, and the panic-to-structured-error conversion.
#[proc_macro_attribute]
pub fn handler(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let function = parse_macro_input!(item as ItemFn);
    // TODO(milestone-7): emit `#[unsafe(no_mangle)] pub extern "C" fn sq_handle_..`
    // wrapping this body, plus the ABI version assertion.
    quote!(#function).into()
}

/// Implement the store traits for a generated entity struct.
///
/// Milestone 7 adds the `Entity` impl: `NAME`, `id()`, and the
/// [`superquery_types::Value`] lowering for each field.
#[proc_macro_derive(SuperQueryEntity, attributes(superquery))]
pub fn derive_entity(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let _name = &input.ident;
    // TODO(milestone-7): generate `impl superquery_sdk::store::Entity`.
    TokenStream::new()
}
