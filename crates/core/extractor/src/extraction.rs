use crate::{Preset, Result};
use markdown_ast::{Document, Node};

pub struct Extractor<R> {
    preset: Preset<R>,
}
impl<R> Extractor<R> {
    pub fn new(preset: &Preset<R>) -> Self {
        Self {
            preset: preset.clone(),
        }
    }

    /// Start a fresh result and visit every node in document preorder. Unknown
    /// types are not collected, but their validation and descendants still count.
    pub fn extract(&self, document: &Document) -> Result<R>
    where
        R: Default,
    {
        let nodes = validated(document)?;
        let mut result = R::default();
        for node in nodes {
            if let Some(handler) = self.preset.handlers.get(&node.data_type_id()) {
                handler(node, &mut result)?;
            }
        }
        Ok(result)
    }
}

// Validate before invoking any handler, including for types without handlers.
// Keep this traversal independent of rendering (which may hide descendants).
fn validated(document: &Document) -> Result<Vec<&Node>> {
    if document.source.len() > 65_536 || document.children.len() > 16_384 {
        return Err("resource_limit");
    }
    let mut scheduled = document.children.len();
    let mut nodes = Vec::new();
    let mut pending: Vec<_> = document
        .children
        .iter()
        .rev()
        .map(|node| (node, 1, 0..document.source.len()))
        .collect();
    while let Some((node, depth, parent)) = pending.pop() {
        if depth > 64 {
            return Err("resource_limit");
        }
        let span = node.span;
        if span.start > span.end
            || span.start < parent.start
            || span.end > parent.end
            || !document.source.is_char_boundary(span.start)
            || !document.source.is_char_boundary(span.end)
            || !node.validate()
        {
            return Err("invalid_node");
        }
        scheduled = scheduled.saturating_add(node.children.len());
        if scheduled > 16_384 {
            return Err("resource_limit");
        }
        nodes.push(node);
        pending.extend(
            node.children
                .iter()
                .rev()
                .map(|child| (child, depth + 1, span.start..span.end)),
        );
    }
    Ok(nodes)
}
