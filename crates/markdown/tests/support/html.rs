//! A literal spec-test renderer. Raw HTML is intentionally passed through only
//! here; this module is not a production renderer or a browser integration.
use traq_markdown::{Node, NodeKind};

fn escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
fn plain(nodes: &[Node]) -> String {
    nodes
        .iter()
        .map(|node| match &node.kind {
            NodeKind::Text { value } => value.clone(),
            NodeKind::InlineCode { literal } => literal.clone(),
            NodeKind::Softbreak | NodeKind::Hardbreak => "\n".into(),
            _ => plain(&node.children),
        })
        .collect()
}
pub fn render(nodes: &[Node]) -> String {
    render_with(nodes, false)
}
fn render_with(nodes: &[Node], tight: bool) -> String {
    let mut output = String::new();
    for node in nodes {
        let nested = || render_with(&node.children, false);
        let value = match &node.kind {
            NodeKind::Text { value } => escape(value),
            NodeKind::Paragraph if tight => nested(),
            NodeKind::Paragraph => format!("<p>{}</p>\n", nested()),
            NodeKind::Heading { level } => format!("<h{level}>{}</h{level}>\n", nested()),
            NodeKind::Blockquote => format!("<blockquote>\n{}</blockquote>\n", nested()),
            NodeKind::List {
                ordered,
                start,
                tight,
            } => {
                let tag = if *ordered { "ol" } else { "ul" };
                let start = if *ordered && *start != 1 {
                    format!(" start=\"{start}\"")
                } else {
                    String::new()
                };
                format!(
                    "<{tag}{start}>\n{}</{tag}>\n",
                    render_with(&node.children, *tight)
                )
            }
            NodeKind::ListItem { .. } => {
                let mut content = render_with(&node.children, tight);
                if node
                    .children
                    .first()
                    .is_some_and(|n| !tight || !matches!(n.kind, NodeKind::Paragraph))
                {
                    content.insert(0, '\n');
                }
                // A following block begins on a new line after a tight paragraph.
                if tight
                    && node.children.len() > 1
                    && matches!(node.children[0].kind, NodeKind::Paragraph)
                {
                    let first = render_with(&node.children[..1], true);
                    content = format!("{first}\n{}", render_with(&node.children[1..], tight));
                }
                format!("<li>{content}</li>\n")
            }
            NodeKind::CodeBlock { info, literal, .. } => {
                let info = traq_markdown::syntax::commonmark::inlines::entities::unescape(info);
                let class = info
                    .split_whitespace()
                    .next()
                    .map(|language| format!(" class=\"language-{}\"", escape(language)))
                    .unwrap_or_default();
                format!("<pre><code{class}>{}</code></pre>\n", escape(literal))
            }
            NodeKind::ThematicBreak { .. } => "<hr />\n".into(),
            NodeKind::Softbreak => "\n".into(),
            NodeKind::Hardbreak => "<br />\n".into(),
            NodeKind::InlineCode { literal } => format!("<code>{}</code>", escape(literal)),
            NodeKind::Emphasis => format!("<em>{}</em>", nested()),
            NodeKind::Strong => format!("<strong>{}</strong>", nested()),
            NodeKind::Link {
                destination, title, ..
            } => {
                let title = title
                    .as_ref()
                    .map(|t| format!(" title=\"{}\"", escape(t)))
                    .unwrap_or_default();
                format!(
                    "<a href=\"{}\"{title}>{}</a>",
                    escape(destination),
                    nested()
                )
            }
            NodeKind::Image {
                destination, title, ..
            } => {
                let title = title
                    .as_ref()
                    .map(|t| format!(" title=\"{}\"", escape(t)))
                    .unwrap_or_default();
                format!(
                    "<img src=\"{}\" alt=\"{}\"{title} />",
                    escape(destination),
                    escape(&plain(&node.children))
                )
            }
            NodeKind::HtmlInline { literal } | NodeKind::HtmlBlock { literal } => literal.clone(),
            NodeKind::Extension { name, .. } => panic!("extension in CommonMark: {name}"),
        };
        output.push_str(&value);
    }
    output
}
