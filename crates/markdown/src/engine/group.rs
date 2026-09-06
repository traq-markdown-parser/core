use super::Plugin;
use std::sync::Arc;

/// A namespace symbol. Names are optional labels, never lookup keys.
#[derive(Clone, Default)]
pub struct PluginGroup {
    node: Arc<GroupNode>,
    label: Option<String>,
}
#[derive(Default)]
struct GroupNode {
    parent: Option<PluginGroup>,
}
impl PluginGroup {
    pub fn named(mut self, name: impl Into<String>) -> Self {
        self.label = Some(name.into());
        self
    }
    pub fn name(&self) -> Option<&str> {
        self.label.as_deref()
    }
    pub fn parent(&self) -> Option<&Self> {
        self.node.parent.as_ref()
    }
    pub fn group(&self) -> Self {
        Self {
            node: Arc::new(GroupNode {
                parent: Some(self.clone()),
            }),
            label: None,
        }
    }
    // A namespace creates plugins; its constructor is Plugin::group().
    #[allow(clippy::new_ret_no_self)]
    pub fn new(&self) -> Plugin {
        Plugin {
            group: Some(self.clone()),
            ..Plugin::default()
        }
    }
    pub(crate) fn same(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.node, &other.node)
    }
    pub(crate) fn description(&self) -> String {
        let label = self.name().unwrap_or("<anonymous group>");
        self.parent()
            .map_or_else(|| label.into(), |p| format!("{}/{label}", p.description()))
    }
}
