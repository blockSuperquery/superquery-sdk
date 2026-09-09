//! The EVM [`ChainIntegration`] implementation.

use alloy_primitives::Address;
use camino::Utf8Path;
use superquery_chain_api::{
    ChainIntegration, ChainValidation, CodegenError, GeneratedModule, KindSpec,
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
                    format!(
                        "unknown EVM datasource kind `{}`; expected `evm/Runtime`",
                        ds.kind
                    ),
                );
            }

            match ds.options.get_str("address") {
                Some(address) if address.parse::<Address>().is_err() => out.error(
                    at(".options.address"),
                    format!("`{address}` is not a valid EVM address"),
                ),
                None => out.warn(
                    at(".options.address"),
                    "no address set — this datasource matches every contract on the chain"
                        .to_owned(),
                ),
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
                        format!(
                            "unknown EVM handler kind `{}`; expected one of {}",
                            handler.kind,
                            kinds::HANDLER_KINDS
                                .iter()
                                .map(|k| k.kind)
                                .collect::<Vec<_>>()
                                .join(", ")
                        ),
                    );
                    continue;
                }

                if let Some(HandlerFilter::Log(filter)) = &handler.filter
                    && !is_event_signature(&filter.event)
                {
                    out.error(
                        at_h(".filter.event"),
                        format!(
                            "`{}` is not an event signature; write it as `Transfer(address,address,uint256)`",
                            filter.event
                        ),
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
