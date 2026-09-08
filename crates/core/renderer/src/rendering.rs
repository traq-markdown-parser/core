use crate::{Preset, Result};
use markdown_ast::{Document, Node};
use std::cell::Cell;

pub struct Renderer {
    preset: Preset,
}
impl Renderer {
    pub fn new(preset: &Preset) -> Self {
        Self {
            preset: preset.clone(),
        }
    }

    pub fn render(&self, doc: &Document) -> Result<String> {
        if doc.source.len() > 65_536 {
            return Err("resource_limit");
        }
        // Validate every descendant before any handler executes, even when a
        // parent hides its children. Bound pending work before allocating it.
        let mut pending = vec![];
        let mut count = 0usize;
        let mut push = |nodes: &[Node]| -> Result<()> {
            count = count.saturating_add(nodes.len());
            if count > 16_384 {
                return Err("resource_limit");
            }
            Ok(())
        };
        push(&doc.children)?;
        pending.extend(doc.children.iter().map(|n| (n, 1, 0..doc.source.len())));
        while let Some((node, depth, parent)) = pending.pop() {
            if depth > 64 {
                return Err("resource_limit");
            }
            let span = node.span;
            if span.start > span.end
                || span.start < parent.start
                || span.end > parent.end
                || !doc.source.is_char_boundary(span.start)
                || !doc.source.is_char_boundary(span.end)
                || !node.validate()
            {
                return Err("invalid_node");
            }
            if !self.preset.handlers.contains_key(&node.data_type_id()) {
                return Err("unsupported_node");
            }
            push(&node.children)?;
            pending.extend(
                node.children
                    .iter()
                    .map(|n| (n, depth + 1, span.start..span.end)),
            );
        }
        Context {
            renderer: self,
            work: Cell::new(0),
        }
        .children(&doc.children)
    }
}

pub struct Context<'a> {
    renderer: &'a Renderer,
    work: Cell<usize>,
}
impl Context<'_> {
    pub fn append(&self, output: &mut String, value: &str) -> Result<()> {
        let work = self.work.get().saturating_add(value.len());
        if output.len().saturating_add(value.len()) > 1_048_576 || work > 8_388_608 {
            return Err("resource_limit");
        }
        self.work.set(work);
        output.push_str(value);
        Ok(())
    }

    pub fn children(&self, nodes: &[Node]) -> Result<String> {
        let mut output = String::new();
        for node in nodes {
            let handler = self
                .renderer
                .preset
                .handlers
                .get(&node.data_type_id())
                .ok_or("unsupported_node")?;
            self.append(&mut output, &handler(node, self)?)?;
        }
        Ok(output)
    }
}
