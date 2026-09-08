use markdown_extractor::{Plugin, Result};
use markdown_trap_contracts::{ReferenceData, ReferenceKind};
use serde::Serialize;

#[derive(Debug, Default, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct References {
    pub mentions: Vec<String>,
    pub group_mentions: Vec<String>,
    pub channel_links: Vec<String>,
}
/// Repeated calls share the default implementation; editing a value is isolated.
pub fn plugin() -> Plugin<References> {
    static PLUGIN: std::sync::LazyLock<Plugin<References>> =
        std::sync::LazyLock::new(|| build().expect("valid reference extraction plugin"));
    PLUGIN.clone()
}

fn build() -> Result<Plugin<References>> {
    let mut plugin = Plugin::<References>::new(&markdown_trap_contracts::preset().references);
    plugin.on::<ReferenceData>(|reference, result| {
        if let Some(id) = crate::uuid::normalize(&reference.id) {
            let ids = match reference.target {
                ReferenceKind::User => &mut result.mentions,
                ReferenceKind::Group => &mut result.group_mentions,
                ReferenceKind::Channel => &mut result.channel_links,
            };
            // Preserve document order and duplicates, including inside spoilers.
            ids.push(id);
        }
        Ok(())
    })?;
    Ok(plugin)
}
