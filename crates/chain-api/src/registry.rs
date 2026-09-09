//! Looking up the integration for a family.

use std::collections::BTreeMap;
use superquery_types::ChainFamily;

use crate::integration::ChainIntegration;

/// The set of chain integrations linked into this build.
#[derive(Default)]
pub struct Registry {
    integrations: BTreeMap<ChainFamily, Box<dyn ChainIntegration>>,
}

impl Registry {
    /// An empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Register an integration, replacing any previous one for its family.
    pub fn register(&mut self, integration: Box<dyn ChainIntegration>) -> &mut Self {
        self.integrations.insert(integration.family(), integration);
        self
    }

    /// Look up the integration for a family.
    pub fn get(&self, family: ChainFamily) -> Result<&dyn ChainIntegration, UnsupportedFamily> {
        self.integrations
            .get(&family)
            .map(AsRef::as_ref)
            .ok_or(UnsupportedFamily { family })
    }

    /// Families this build can handle.
    pub fn families(&self) -> impl Iterator<Item = ChainFamily> + '_ {
        self.integrations.keys().copied()
    }
}

impl std::fmt::Debug for Registry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Registry")
            .field("families", &self.families().collect::<Vec<_>>())
            .finish()
    }
}

/// No integration is linked for the requested family.
#[derive(Debug, thiserror::Error)]
#[error("chain family `{family}` is not supported by this build")]
pub struct UnsupportedFamily {
    /// The family that was requested.
    pub family: ChainFamily,
}
