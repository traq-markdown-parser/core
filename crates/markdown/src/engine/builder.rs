use super::{BuildError, Contribution, Grammar, Plugin, rule::Rule};
use crate::ast::ExtensionDefinition;
use std::collections::HashMap;

#[derive(Clone, Default)]
pub struct GrammarBuilder {
    pub(crate) plugins: Vec<Plugin>,
    pub(crate) rules: Vec<(Plugin, Contribution)>,
}
impl GrammarBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add(&mut self, plugin: &Plugin) -> Result<&mut Self, BuildError> {
        if self.plugins.iter().any(|p| p.same(plugin)) {
            return Err(BuildError::Duplicate {
                element: plugin.description(),
            });
        }
        let mut pending = self.rules.iter().map(|(_, r)| r).collect::<Vec<_>>();
        for rule in plugin.rules.iter() {
            if pending.iter().any(|r| r.same(rule)) {
                return Err(BuildError::Duplicate {
                    element: rule.description(),
                });
            }
            pending.push(rule);
        }
        definitions(pending.into_iter())?;
        self.plugins.push(plugin.clone());
        self.rules.extend(
            plugin
                .rules
                .iter()
                .cloned()
                .map(|rule| (plugin.clone(), rule)),
        );
        Ok(self)
    }
    pub fn remove(&mut self, plugin: &Plugin) -> Result<&mut Self, BuildError> {
        if !self.plugins.iter().any(|p| p.same(plugin)) {
            return Err(BuildError::Missing {
                element: plugin.description(),
            });
        }
        self.plugins.retain(|p| !p.same(plugin));
        self.rules.retain(|(owner, _)| !owner.same(plugin));
        Ok(self)
    }
    /// Order two rules in the same phase. Other phases are a compile-time error.
    /// ```compile_fail
    /// use traq_markdown::{GrammarBuilder, engine::{inline::InlineRule, block::BlockRule}};
    /// let mut builder = GrammarBuilder::new();
    /// let inline = InlineRule::new(b"x", |_, _| Ok(None));
    /// let block = BlockRule::new(|_, _| Ok(None));
    /// builder.before(&inline, &block);
    /// ```
    pub fn before<T: Clone>(
        &mut self,
        rule: &Rule<T>,
        anchor: &Rule<T>,
    ) -> Result<&mut Self, BuildError>
    where
        for<'a> Contribution: From<&'a Rule<T>>,
    {
        let (rule, anchor) = (Contribution::from(rule), Contribution::from(anchor));
        let from = self
            .rules
            .iter()
            .position(|(_, r)| r.same(&rule))
            .ok_or_else(|| BuildError::Missing {
                element: rule.description(),
            })?;
        let to = self
            .rules
            .iter()
            .position(|(_, r)| r.same(&anchor))
            .ok_or_else(|| BuildError::Missing {
                element: anchor.description(),
            })?;
        if from != to {
            let entry = self.rules.remove(from);
            self.rules
                .insert(if from < to { to - 1 } else { to }, entry);
        }
        Ok(self)
    }
    pub(crate) fn describe(&self) -> String {
        let mut lines = vec![];
        for plugin in &self.plugins {
            lines.push(format!("plugin: {}", plugin.description()));
        }
        for phase in ["block", "inline", "text"] {
            for (plugin, rule) in &self.rules {
                if rule.phase() == phase {
                    lines.push(format!(
                        "{phase}: {}/{}",
                        plugin.description(),
                        rule.description()
                    ));
                }
            }
        }
        lines.join("\n")
    }
    pub(crate) fn validate(&self) -> Result<Vec<ExtensionDefinition>, BuildError> {
        super::names::validate(&self.plugins)?;
        definitions(self.rules.iter().map(|(_, rule)| rule))
    }
    pub fn build(self) -> Result<Grammar, BuildError> {
        let extensions = self.validate()?;
        let extension_index = extensions
            .iter()
            .enumerate()
            .map(|(i, d)| (d.name, i))
            .collect();
        let mut grammar = super::registry::CompiledGrammar {
            definition: self,
            inline: vec![],
            block: vec![],
            text: vec![],
            extensions,
            extension_index,
            dispatch: std::array::from_fn(|_| vec![]),
        };
        for (_, rule) in &grammar.definition.rules {
            match rule {
                Contribution::Inline(r) => grammar.inline.push(r.clone()),
                Contribution::Block(r) => grammar.block.push(r.clone()),
                Contribution::Text(r) => grammar.text.push(r.clone()),
            }
        }
        for (index, rule) in grammar.inline.iter().enumerate() {
            let markers = rule.data.implementation.markers;
            for (byte, candidates) in grammar.dispatch.iter_mut().enumerate() {
                if markers.is_empty() || markers.contains(&(byte as u8)) {
                    candidates.push(index);
                }
            }
        }
        Ok(Grammar {
            data: std::sync::Arc::new(grammar),
        })
    }
}
fn definitions<'a>(
    rules: impl Iterator<Item = &'a Contribution>,
) -> Result<Vec<ExtensionDefinition>, BuildError> {
    let mut seen = HashMap::new();
    let mut result = vec![];
    for rule in rules {
        for definition in rule.extensions() {
            if let Some(previous) = seen.insert(definition.name, definition.type_id()) {
                if previous != definition.type_id() {
                    return Err(BuildError::Duplicate {
                        element: definition.name.into(),
                    });
                }
            } else {
                result.push(*definition);
            }
        }
    }
    Ok(result)
}
