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
//! Each macro is a thin `proc_macro` shim over a `proc_macro2` expansion in
//! its own module, so the expansion is unit-testable without a compiler.

mod entity;
mod handler;

use proc_macro::TokenStream;
use syn::{DeriveInput, ItemFn, parse_macro_input};

/// Mark a function as a mapping handler.
///
/// ```ignore
/// #[handler]
/// pub async fn handle_transfer(event: EvmLog<Transfer>) -> Result<()> { .. }
/// ```
///
/// The function is left untouched. Beside it the macro adds the
/// `sq_handle_<name>` WASM export the node calls, which decodes the payload
/// with `HandlerInput`, runs the handler and reports an `Err` through
/// `sq_handler_error`. Sync and async functions are both accepted; the one
/// argument must implement `HandlerInput`.
#[proc_macro_attribute]
pub fn handler(attr: TokenStream, item: TokenStream) -> TokenStream {
    let function = parse_macro_input!(item as ItemFn);
    handler::expand(attr.into(), function)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Implement `superquery_sdk::store::Entity` for a generated entity struct.
///
/// ```ignore
/// #[derive(SuperQueryEntity)]
/// #[superquery(entity = "Transfer")]          // defaults to the struct name
/// pub struct Transfer {
///     pub id: String,                          // required: the primary key
///     #[superquery(rename = "blockNumber")]    // the schema's field name
///     pub block_number: BigInt,
/// }
/// ```
///
/// Every field type must implement `ToValue` and `FromValue`.
#[proc_macro_derive(SuperQueryEntity, attributes(superquery))]
pub fn derive_entity(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    entity::expand(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
