//! A Markdown parser with explicit, composable syntax rules.
#![forbid(unsafe_code)]
// SourceView intentionally takes a list of byte ranges, including singleton lists.
#![allow(clippy::single_range_in_vec_init)]

pub mod ast;
#[doc(hidden)]
pub mod bindings;
pub mod engine;
pub mod presets;
pub mod syntax;

pub use ast::{Document, LinkForm, Node, NodeKind, Span};
pub use engine::{Grammar, GrammarBuilder, Limits, ParseError, Parser};
