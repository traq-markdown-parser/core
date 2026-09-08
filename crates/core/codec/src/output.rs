use super::Codec;
use markdown_ast::{Document, Node};
use serde::{Serialize, Serializer};

pub(super) fn encode(document: &Document, codec: &Codec) -> serde_json::Result<Vec<u8>> {
    // Check before recursive serialization, including subtrees a renderer may hide.
    let limits = crate::DecodeLimits::default();
    if document.source.len() > limits.source_bytes {
        return Err(crate::fields::error("source byte limit"));
    }
    let mut pending: Vec<_> = document
        .children
        .iter()
        .map(|node| (node, 1, 0..document.source.len()))
        .collect();
    let mut count = 0;
    while let Some((node, depth, parent)) = pending.pop() {
        count += 1;
        if count > limits.nodes {
            return Err(crate::fields::error("node limit"));
        }
        if depth > limits.depth {
            return Err(crate::fields::error("depth limit"));
        }
        let span = node.span;
        if span.start > span.end
            || span.start < parent.start
            || span.end > parent.end
            || !document.source.is_char_boundary(span.start)
            || !document.source.is_char_boundary(span.end)
        {
            return Err(crate::fields::error("invalid span"));
        }
        if !node.validate() {
            return Err(crate::fields::error("invalid node shape"));
        }
        pending.extend(
            node.children
                .iter()
                .map(|child| (child, depth + 1, span.start..span.end)),
        );
    }
    let mut output = LimitedOutput {
        bytes: vec![],
        limit: limits.json_bytes,
    };
    serde_json::to_writer(&mut output, &DocumentOutput { document, codec })?;
    Ok(output.bytes)
}

struct LimitedOutput {
    bytes: Vec<u8>,
    limit: usize,
}
impl std::io::Write for LimitedOutput {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        if bytes.len() > self.limit.saturating_sub(self.bytes.len()) {
            return Err(std::io::Error::other("json byte limit"));
        }
        self.bytes.extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

pub(super) struct DocumentOutput<'a> {
    pub document: &'a Document,
    pub codec: &'a Codec,
}
impl Serialize for DocumentOutput<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct Frame<'a> {
            source: &'a str,
            children: Children<'a>,
        }
        Frame {
            source: &self.document.source,
            children: Children {
                nodes: &self.document.children,
                codec: self.codec,
            },
        }
        .serialize(serializer)
    }
}
struct NodeOutput<'a> {
    node: &'a Node,
    codec: &'a Codec,
}
impl Serialize for NodeOutput<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let entry = self
            .codec
            .entries
            .get(&self.node.kind.data_type_id())
            .ok_or_else(|| serde::ser::Error::custom("unregistered node type"))?;
        #[derive(Serialize)]
        struct Span {
            start: usize,
            end: usize,
        }
        #[derive(Serialize)]
        struct Frame<'a> {
            kind: &'a str,
            span: Span,
            data: &'a dyn erased_serde::Serialize,
            #[serde(skip_serializing_if = "Children::is_empty")]
            children: Children<'a>,
        }
        Frame {
            kind: &entry.name,
            span: Span {
                start: self.node.span.start,
                end: self.node.span.end,
            },
            data: (entry.borrow)(&self.node.kind),
            children: Children {
                nodes: &self.node.children,
                codec: self.codec,
            },
        }
        .serialize(serializer)
    }
}
struct Children<'a> {
    nodes: &'a [Node],
    codec: &'a Codec,
}
impl Children<'_> {
    fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}
impl Serialize for Children<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_seq(self.nodes.iter().map(|node| NodeOutput {
            node,
            codec: self.codec,
        }))
    }
}
