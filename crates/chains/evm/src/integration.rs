//! The EVM [`ChainIntegration`] implementation.

use alloy_primitives::Address;
use camino::Utf8Path;
use superquery_chain_api::{
    ChainIntegration, ChainValidation, CodegenError, GeneratedModule, KindSpec, ProjectTemplate,
};
use superquery_manifest::{DataSource, HandlerFilter, ProjectManifest};
use superquery_types::ChainFamily;

use crate::abi::LoadedAbi;
use crate::kinds;

/// EVM support, registered into a [`superquery_chain_api::Registry`].
#[derive(Debug, Default, Clone, Copy)]
pub struct EvmIntegration;

impl ChainIntegration for EvmIntegration {
    fn family(&self) -> ChainFamily {
        ChainFamily::Evm
    }

    fn kinds(&self) -> &[KindSpec] {
        // `kinds::all()` allocates; the trait wants a slice, so hand back the
        // two static tables joined by the caller when it needs both.
        kinds::HANDLER_KINDS
    }

    fn validate(&self, manifest: &ProjectManifest) -> ChainValidation {
        let mut out = ChainValidation::default();

        for (i, ds) in manifest.data_sources.iter().enumerate() {
            let at = |suffix: &str| format!("dataSources[{i}]{suffix}");

            if !kinds::DATA_SOURCE_KINDS
                .iter()
                .any(|k| k.kind == ds.kind.as_str())
            {
                out.error(
                    at(".kind"),
                    format!("unknown EVM datasource kind `{}`", ds.kind),
                )
                .with_help("EVM datasources are `evm/Runtime`");
            }

            match ds.options.get_str("address") {
                Some(address) if address.parse::<Address>().is_err() => {
                    out.error(
                        at(".options.address"),
                        format!("`{address}` is not a valid EVM address"),
                    )
                    .with_help("write a 20-byte hex address, e.g. \"0xA0b8...eB48\"");
                }
                None => {
                    out.warn(
                        at(".options.address"),
                        "no address set — this datasource matches every contract on the chain",
                    )
                    .with_help("set `options.address` unless indexing every emitter is intended");
                }
                Some(_) => {}
            }

            for (j, handler) in ds.handlers.iter().enumerate() {
                let at_h = |suffix: &str| format!("dataSources[{i}].handlers[{j}]{suffix}");

                if !kinds::HANDLER_KINDS
                    .iter()
                    .any(|k| k.kind == handler.kind.as_str())
                {
                    out.error(
                        at_h(".kind"),
                        format!("unknown EVM handler kind `{}`", handler.kind),
                    )
                    .with_help(format!(
                        "expected one of {}",
                        kinds::HANDLER_KINDS
                            .iter()
                            .map(|k| k.kind)
                            .collect::<Vec<_>>()
                            .join(", ")
                    ));
                    continue;
                }

                if let Some(HandlerFilter::Log(filter)) = &handler.filter
                    && !is_event_signature(&filter.event)
                {
                    out.error(
                        at_h(".filter.event"),
                        format!("`{}` is not an event signature", filter.event),
                    )
                    .with_help(
                        "write the full signature, e.g. `Transfer(address,address,uint256)`",
                    );
                }
            }
        }

        out
    }

    fn codegen(
        &self,
        _manifest: &ProjectManifest,
        data_source: &DataSource,
        base_dir: &Utf8Path,
    ) -> Result<Vec<GeneratedModule>, CodegenError> {
        data_source
            .assets
            .iter()
            .map(|(name, asset)| {
                let path = base_dir.join(&asset.file);
                Ok(LoadedAbi::load(name, &path)?.generate())
            })
            .collect()
    }

    fn template(&self) -> Option<&'static ProjectTemplate> {
        Some(&crate::template::TEMPLATE)
    }
}

/// A cheap shape check for `Name(type,type)`.
///
/// Full ABI type validation belongs to the ABI itself; this catches the common
/// slip of writing a bare event name and wondering why nothing matches.
fn is_event_signature(sig: &str) -> bool {
    let Some((name, rest)) = sig.split_once('(') else {
        return false;
    };
    let Some(args) = rest.strip_suffix(')') else {
        return false;
    };
    if name.is_empty() || !name.starts_with(|c: char| c.is_ascii_alphabetic()) {
        return false;
    }
    args.is_empty() || args.split(',').all(|a| !a.trim().is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn manifest(data_source: &str) -> ProjectManifest {
        let yaml = format!(
            "specVersion: \"1.0\"\nname: t\nversion: \"0.1.0\"\n\
             network: {{ family: evm, chainId: \"1\" }}\n\
             schema: {{ file: ./schema.graphql }}\n\
             dataSources:\n{data_source}"
        );
        superquery_manifest::from_str(&yaml, "test.yaml").expect("fixture parses")
    }

    #[test]
    fn a_malformed_address_is_an_error_with_a_fix() {
        let report = EvmIntegration.validate(&manifest(
            "  - kind: evm/Runtime\n    options: { address: \"0x12\" }\n    handlers: []\n",
        ));
        let [finding] = &report.errors[..] else {
            panic!("{report:?}");
        };
        assert_eq!(finding.field, "dataSources[0].options.address");
        assert!(finding.help.is_some());
    }

    #[test]
    fn a_missing_address_only_warns() {
        let report =
            EvmIntegration.validate(&manifest("  - kind: evm/Runtime\n    handlers: []\n"));
        assert!(!report.has_errors(), "{report:?}");
        assert_eq!(report.warnings[0].field, "dataSources[0].options.address");
    }

    #[test]
    fn an_unknown_handler_kind_lists_the_legal_ones() {
        let report = EvmIntegration.validate(&manifest(
            "  - kind: evm/Runtime\n    options: { address: \"0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48\" }\n    \
             handlers:\n      - { handler: h, kind: evm/CallHandler }\n",
        ));
        let help = report.errors[0].help.as_deref().unwrap();
        assert!(help.contains("evm/LogHandler"), "{help}");
    }

    #[test]
    fn recognises_well_formed_event_signatures() {
        assert!(is_event_signature("Transfer(address,address,uint256)"));
        assert!(is_event_signature("Paused()"));
    }

    #[test]
    fn rejects_a_bare_event_name_and_malformed_arguments() {
        assert!(!is_event_signature("Transfer"));
        assert!(!is_event_signature("Transfer(address,,uint256)"));
        assert!(!is_event_signature("(address)"));
        assert!(!is_event_signature("Transfer(address"));
    }
}
