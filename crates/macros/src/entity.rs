//! `#[derive(SuperQueryEntity)]`.
//!
//! The expansion is one plain `impl Entity` a developer can read: a `set`
//! per field going out, a `from_value` per field coming back. Field names on
//! the wire are the schema's names, which codegen passes down with
//! `#[superquery(rename = "...")]` whenever the Rust identifier differs.

use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Fields, LitStr};

pub(crate) fn expand(input: DeriveInput) -> syn::Result<TokenStream> {
    let ident = &input.ident;
    let entity_name = struct_entity_name(&input)?;

    let Data::Struct(data) = &input.data else {
        return Err(syn::Error::new(
            ident.span(),
            "SuperQueryEntity can only be derived for a struct with named fields",
        ));
    };
    let Fields::Named(named) = &data.fields else {
        return Err(syn::Error::new(
            data.fields.span(),
            "SuperQueryEntity needs named fields, one per schema field",
        ));
    };
    if !input.generics.params.is_empty() {
        return Err(syn::Error::new(
            input.generics.span(),
            "an entity is a concrete schema type and cannot be generic",
        ));
    }

    let mut id_field = None;
    let mut lowering = Vec::new();
    let mut raising = Vec::new();

    for field in &named.named {
        let rust_name = field.ident.as_ref().expect("named fields have identifiers");
        let schema_name = field_schema_name(field)?;

        if schema_name == "id" {
            id_field = Some(rust_name);
        }

        lowering.push(quote_spanned! {field.span()=>
            entity.set(#schema_name, __sq::ToValue::to_value(&self.#rust_name));
        });
        raising.push(quote_spanned! {field.span()=>
            #rust_name: __sq::FromValue::from_value(
                entity.get(#schema_name).unwrap_or(&__sq::Value::Null),
            )
            .map_err(|err| __sq::Error::Entity {
                operation: "decode",
                entity: #entity_name.to_owned(),
                message: format!("field `{}`: {}", #schema_name, err),
            })?,
        });
    }

    let Some(id_field) = id_field else {
        return Err(syn::Error::new(
            named.span(),
            "an entity needs an `id` field: it is the store's primary key",
        ));
    };

    Ok(quote! {
        const _: () = {
            use ::superquery_sdk::__private as __sq;

            #[automatically_derived]
            impl ::superquery_sdk::store::Entity for #ident {
                const NAME: &'static str = #entity_name;

                fn id(&self) -> __sq::Result<__sq::EntityId> {
                    __sq::entity_id(#entity_name, &self.#id_field)
                }

                fn to_untyped(&self) -> __sq::UntypedEntity {
                    let mut entity = __sq::UntypedEntity::new();
                    #(#lowering)*
                    entity
                }

                fn from_untyped(entity: &__sq::UntypedEntity) -> __sq::Result<Self> {
                    Ok(Self { #(#raising)* })
                }
            }
        };
    })
}

/// `#[superquery(entity = "Name")]`, defaulting to the struct's own name.
fn struct_entity_name(input: &DeriveInput) -> syn::Result<String> {
    let mut name = None;
    for attr in input
        .attrs
        .iter()
        .filter(|a| a.path().is_ident("superquery"))
    {
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("entity") {
                name = Some(meta.value()?.parse::<LitStr>()?.value());
                Ok(())
            } else {
                Err(meta.error("expected `entity = \"Name\"`"))
            }
        })?;
    }
    Ok(name.unwrap_or_else(|| input.ident.to_string()))
}

/// `#[superquery(rename = "schemaName")]`, defaulting to the identifier with
/// any `r#` stripped.
fn field_schema_name(field: &syn::Field) -> syn::Result<String> {
    let mut name = None;
    for attr in field
        .attrs
        .iter()
        .filter(|a| a.path().is_ident("superquery"))
    {
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("rename") {
                name = Some(meta.value()?.parse::<LitStr>()?.value());
                Ok(())
            } else {
                Err(meta.error("expected `rename = \"schemaName\"`"))
            }
        })?;
    }
    let ident = field.ident.as_ref().expect("named fields have identifiers");
    Ok(name.unwrap_or_else(|| {
        let raw = ident.to_string();
        raw.strip_prefix("r#").map(str::to_owned).unwrap_or(raw)
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    fn expanded(input: DeriveInput) -> String {
        expand(input).expect("expands").to_string()
    }

    fn error(input: DeriveInput) -> String {
        expand(input).expect_err("rejected").to_string()
    }

    #[test]
    fn uses_schema_names_on_the_wire_and_rust_names_in_the_struct() {
        let out = expanded(parse_quote! {
            #[superquery(entity = "Transfer")]
            struct TransferEntity {
                id: String,
                #[superquery(rename = "blockNumber")]
                block_number: BigInt,
                r#type: String,
            }
        });
        assert!(
            out.contains(r#"const NAME : & 'static str = "Transfer""#),
            "{out}"
        );
        assert!(out.contains(r#"entity . set ("blockNumber""#), "{out}");
        assert!(out.contains("self . block_number"), "{out}");
        assert!(out.contains(r#"entity . set ("type""#), "{out}");
    }

    #[test]
    fn the_entity_name_defaults_to_the_struct_name() {
        let out = expanded(parse_quote! { struct Holder { id: String } });
        assert!(out.contains(r#""Holder""#), "{out}");
    }

    #[test]
    fn rejects_a_struct_without_an_id_field() {
        let err = error(parse_quote! { struct NoId { name: String } });
        assert!(err.contains("needs an `id` field"), "{err}");
    }

    #[test]
    fn rejects_enums_tuple_structs_and_generics() {
        assert!(error(parse_quote! { enum E { A } }).contains("struct with named fields"));
        assert!(error(parse_quote! { struct T(String); }).contains("named fields"));
        assert!(error(parse_quote! { struct G<T> { id: T } }).contains("cannot be generic"));
    }

    #[test]
    fn rejects_unknown_attribute_keys() {
        let err = error(parse_quote! {
            #[superquery(table = "x")]
            struct T { id: String }
        });
        assert!(err.contains("expected `entity"), "{err}");
    }
}
