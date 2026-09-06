use super::{
    GrammarBuilder, Plugin,
    block::BlockRule,
    inline::{InlineRule, TextRule},
};
use crate::ast::ExtensionDefinition;
use std::collections::HashMap;

/// Immutable compiled grammar. Its composition can be edited through a new builder.
#[derive(Clone)]
pub struct Grammar {
    pub(crate) data: std::sync::Arc<CompiledGrammar>,
}
pub(crate) struct CompiledGrammar {
    pub(crate) definition: GrammarBuilder,
    pub(crate) inline: Vec<InlineRule>,
    pub(crate) block: Vec<BlockRule>,
    pub(crate) text: Vec<TextRule>,
    pub(crate) extensions: Vec<ExtensionDefinition>,
    pub(crate) extension_index: HashMap<&'static str, usize>,
    pub(crate) dispatch: [Vec<usize>; 256],
}
impl Grammar {
    pub fn to_builder(&self) -> GrammarBuilder {
        self.data.definition.clone()
    }
    pub fn plugins(&self) -> &[Plugin] {
        &self.data.definition.plugins
    }
    pub fn extension_names(&self) -> impl Iterator<Item = &'static str> + '_ {
        self.data
            .extensions
            .iter()
            .map(|definition| definition.name)
    }
    pub fn extensions(&self) -> &[ExtensionDefinition] {
        &self.data.extensions
    }
    pub fn describe(&self) -> String {
        self.data.definition.describe()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "code", rename_all = "snake_case")]
pub enum BuildError {
    Duplicate { element: String },
    Missing { element: String },
    DuplicateName { scope: String, name: String },
    InvalidDefinition { reason: String },
}
impl std::fmt::Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for BuildError {}
