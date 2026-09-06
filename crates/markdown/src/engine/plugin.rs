use super::{
    Contribution, PluginGroup,
    block::BlockRule,
    inline::{InlineRule, TextRule},
};
use std::sync::Arc;

/// A plugin definition shared by builders. Editing a shared copy creates a new definition.
#[derive(Clone, Default)]
pub struct Plugin {
    pub(crate) rules: Arc<Vec<Contribution>>,
    pub(crate) label: Option<String>,
    pub(crate) group: Option<PluginGroup>,
}
impl Plugin {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn named(mut self, name: impl Into<String>) -> Self {
        self.label = Some(name.into());
        self
    }
    pub fn group() -> PluginGroup {
        PluginGroup::default()
    }
    pub fn name(&self) -> Option<&str> {
        self.label.as_deref()
    }
    pub fn namespace(&self) -> Option<&PluginGroup> {
        self.group.as_ref()
    }
    pub fn add(&mut self, rule: impl Into<Contribution>) -> &mut Self {
        Arc::make_mut(&mut self.rules).push(rule.into());
        self
    }
    pub fn inline_rules(&self) -> impl Iterator<Item = &InlineRule> {
        self.rules.iter().filter_map(|r| {
            if let Contribution::Inline(r) = r {
                Some(r)
            } else {
                None
            }
        })
    }
    pub fn block_rules(&self) -> impl Iterator<Item = &BlockRule> {
        self.rules.iter().filter_map(|r| {
            if let Contribution::Block(r) = r {
                Some(r)
            } else {
                None
            }
        })
    }
    pub fn text_rules(&self) -> impl Iterator<Item = &TextRule> {
        self.rules.iter().filter_map(|r| {
            if let Contribution::Text(r) = r {
                Some(r)
            } else {
                None
            }
        })
    }
    pub(crate) fn same(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.rules, &other.rules)
    }
    pub(crate) fn description(&self) -> String {
        let label = self.name().unwrap_or("<anonymous plugin>");
        self.namespace()
            .map_or_else(|| label.into(), |g| format!("{}/{label}", g.description()))
    }
}
