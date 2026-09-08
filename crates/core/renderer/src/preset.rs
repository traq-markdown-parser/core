use crate::{Plugin, Result, plugin::Handler};
use markdown_definitions::PluginGroup;
use std::{any::TypeId, collections::HashMap, sync::Arc};

/// Immutable, reusable handlers. No parser or runtime resources are retained.
#[derive(Clone)]
pub struct Preset {
    pub(crate) handlers: Arc<HashMap<TypeId, Handler>>,
}

#[derive(Clone, Default)]
pub struct PresetBuilder {
    plugins: Vec<Plugin>,
}
impl PresetBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Check the whole plugin before changing the builder.
    pub fn add(&mut self, plugin: &Plugin) -> Result<&mut Self> {
        for existing in &self.plugins {
            if existing.same(plugin) {
                return Err("duplicate_plugin");
            }
            if plugin
                .handlers
                .keys()
                .any(|id| existing.handlers.contains_key(id))
            {
                return Err("duplicate_handler");
            }
        }
        self.plugins.push(plugin.clone());
        Ok(self)
    }

    /// Remove the registered implementation snapshot, as in GrammarBuilder.
    pub fn remove(&mut self, plugin: &Plugin) -> Result<&mut Self> {
        let index = self
            .plugins
            .iter()
            .position(|p| p.same(plugin))
            .ok_or("missing_plugin")?;
        self.plugins.remove(index);
        Ok(self)
    }

    pub fn build(self) -> Result<Preset> {
        validate_names(&self.plugins)?;
        let handlers = self
            .plugins
            .iter()
            .flat_map(|plugin| {
                plugin
                    .handlers
                    .iter()
                    .map(|(id, handler)| (*id, handler.clone()))
            })
            .collect();
        Ok(Preset {
            handlers: Arc::new(handlers),
        })
    }
}

// Only selected declarations participate, using parent identity rather than a
// slash-separated identifier. These are display-name collisions, not type IDs.
fn validate_names(plugins: &[Plugin]) -> Result<()> {
    let mut groups: Vec<&PluginGroup> = vec![];
    for plugin in plugins {
        let mut parent = plugin.declaration.namespace();
        while let Some(group) = parent {
            if groups.contains(&group) {
                break;
            }
            groups.push(group);
            parent = group.parent();
        }
    }
    let entries: Vec<_> = groups
        .iter()
        .map(|g| (g.parent(), g.name()))
        .chain(
            plugins
                .iter()
                .map(|p| (p.declaration.namespace(), p.declaration.name())),
        )
        .collect();
    for (index, entry) in entries.iter().enumerate() {
        if entries[..index].contains(entry) {
            return Err("duplicate_name");
        }
    }
    Ok(())
}
