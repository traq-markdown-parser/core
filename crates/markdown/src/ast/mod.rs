mod common;
mod contracts;
mod extension;

pub use common::{LinkForm, NodeKind};
pub use extension::{ExtensionData, ExtensionDefinition};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "contracts", derive(ts_rs::TS))]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "contracts", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Document {
    pub source: String,
    pub children: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "contracts", derive(ts_rs::TS))]
pub struct Node {
    pub span: Span,
    #[serde(flatten)]
    pub kind: NodeKind,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<Node>,
}

impl Node {
    pub fn new(span: Span, kind: NodeKind, children: Vec<Node>) -> Self {
        Self {
            span,
            kind,
            children,
        }
    }
    pub fn leaf(span: Span, kind: NodeKind) -> Self {
        Self::new(span, kind, Vec::new())
    }
}
