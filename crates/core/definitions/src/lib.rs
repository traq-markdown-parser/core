//! Shared declarations and generated type metadata. No AST or serde dependencies.
#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
extern crate self as markdown_definitions;
mod namespace;
mod node_type;

pub use markdown_definitions_derive::NodeType;
pub use namespace::{Plugin, PluginGroup};
pub use node_type::NodeType;
