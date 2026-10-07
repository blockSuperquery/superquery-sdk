//! The ERC-20 Transfers starter project, embedded from `templates/evm`.
//!
//! Embedded rather than read at run time so an installed `superquery` binary
//! works without a checkout. The files stay real files under `templates/` so
//! CI can validate and build them as an ordinary project.

use superquery_chain_api::{ProjectTemplate, TemplateFile};

macro_rules! template_files {
    ($($path:literal),* $(,)?) => {
        &[$(TemplateFile {
            path: $path,
            contents: include_str!(concat!("../../../../templates/evm/", $path)),
        }),*]
    };
}

/// `superquery init --chain evm`.
pub static TEMPLATE: ProjectTemplate = ProjectTemplate {
    placeholder_name: "erc20-transfers",
    files: template_files![
        ".gitignore",
        "Cargo.toml",
        "README.md",
        "abis/ERC20.json",
        "project.yaml",
        "schema.graphql",
        "src/lib.rs",
    ],
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_placeholder_name_is_what_the_manifest_and_crate_declare() {
        let file = |path| {
            TEMPLATE
                .files
                .iter()
                .find(|f| f.path == path)
                .unwrap()
                .contents
        };
        let name = TEMPLATE.placeholder_name;
        assert!(file("project.yaml").contains(&format!("\nname: {name}\n")));
        assert!(file("Cargo.toml").contains(&format!("\nname = \"{name}\"\n")));
    }
}
