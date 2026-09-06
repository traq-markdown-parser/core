//! Distribution metadata and checked composition used by the Wasm bindings.
mod catalog;
mod composition;
pub use catalog::{Catalog, PresetSpec, RuleSpec};
pub use composition::{Composition, GroupSpec, PluginSpec};
use std::sync::LazyLock;

pub fn bundled() -> &'static Catalog {
    static CATALOG: LazyLock<Catalog> = LazyLock::new(crate::presets::exports::catalog);
    &CATALOG
}
