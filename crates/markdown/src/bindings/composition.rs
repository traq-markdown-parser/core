use super::Catalog;
use crate::{
    Grammar, GrammarBuilder,
    engine::{BuildError, Plugin},
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GroupSpec {
    pub parent: Option<usize>,
    pub name: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PluginSpec {
    pub group: Option<usize>,
    pub name: Option<String>,
    pub rules: Vec<usize>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Composition {
    pub groups: Vec<GroupSpec>,
    pub plugins: Vec<PluginSpec>,
    pub order: Vec<usize>,
}
fn invalid(reason: &str) -> BuildError {
    BuildError::InvalidDefinition {
        reason: reason.into(),
    }
}
impl Catalog {
    pub fn build(&self, spec: &Composition) -> Result<Grammar, BuildError> {
        // Bound configuration work independently of message parsing.
        if spec.groups.len() > 256 || spec.plugins.len() > 256 || spec.order.len() > 4096 {
            return Err(invalid("composition limit exceeded"));
        }
        let mut groups: Vec<crate::engine::PluginGroup> = vec![];
        let mut depths = vec![];
        for (index, group) in spec.groups.iter().enumerate() {
            let (mut value, depth) = if let Some(parent) = group.parent {
                if parent >= index {
                    return Err(invalid("namespace parent must precede child"));
                }
                (groups[parent].group(), depths[parent] + 1)
            } else {
                (Plugin::group(), 1)
            };
            if depth > 64 {
                return Err(invalid("namespace depth limit exceeded"));
            }
            if let Some(name) = &group.name {
                value = value.named(name);
            }
            groups.push(value);
            depths.push(depth);
        }
        let mut builder = GrammarBuilder::new();
        for plugin in &spec.plugins {
            let mut value = match plugin.group {
                Some(index) => groups
                    .get(index)
                    .ok_or_else(|| invalid("unknown namespace"))?
                    .new(),
                None => Plugin::new(),
            };
            if let Some(name) = &plugin.name {
                value = value.named(name);
            }
            if plugin.rules.len() > 4096 {
                return Err(invalid("rule limit exceeded"));
            }
            for &index in &plugin.rules {
                value.add(
                    self.rules
                        .get(index)
                        .ok_or_else(|| invalid("unknown rule"))?
                        .clone(),
                );
            }
            builder.add(&value)?;
        }
        if spec.order.len() != builder.rules.len() {
            return Err(invalid("incomplete rule order"));
        }
        let mut seen = HashSet::new();
        let mut ordered = vec![];
        for &index in &spec.order {
            if !seen.insert(index) {
                return Err(invalid("duplicate ordered rule"));
            }
            let rule = self
                .rules
                .get(index)
                .ok_or_else(|| invalid("unknown ordered rule"))?;
            let entry = builder
                .rules
                .iter()
                .find(|(_, r)| r.same(rule))
                .ok_or_else(|| invalid("ordered rule is not registered"))?;
            ordered.push(entry.clone());
        }
        builder.rules = ordered;
        builder.build()
    }
}
