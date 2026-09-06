use crate::syntax::commonmark::{self, LinkOptions};
use crate::{
    NodeKind, Span,
    ast::ExtensionData,
    engine::{
        Plugin,
        block::{BlockRule, DraftContent, DraftNode},
    },
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "contracts", derive(ts_rs::TS, schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct BlankLineData {}
impl ExtensionData for BlankLineData {
    const NAME: &'static str = "trap/blank_line@1";
}

pub fn links() -> LinkOptions {
    LinkOptions {
        direct_images: false,
        nested_autolinks: true,
        normalize: commonmark::inlines::url::normalize,
    }
}
pub fn blank_rule() -> &'static BlockRule {
    static RULE: std::sync::LazyLock<BlockRule> = std::sync::LazyLock::new(|| {
        BlockRule::new(|input, _| {
            if !input.current().bytes().all(|b| b == b' ' || b == b'\t') {
                return Ok(None);
            }
            let mut result = input.leaf(input.start + 1, BlankLineData {}.node_kind()?)?;
            result.consume_separator = false;
            Ok(Some(result))
        })
        .named("blank")
        .produces::<BlankLineData>()
    });
    &RULE
}

pub fn quote_rule() -> &'static BlockRule {
    static RULE: std::sync::LazyLock<BlockRule> = std::sync::LazyLock::new(|| {
        BlockRule::new(|input, budget| {
        let Some(mut found) = commonmark::blocks::quote::parse(input, budget)? else { return Ok(None); };
        for node in &mut found.nodes {
            if matches!(&node.content, DraftContent::Blocks(view) if view.text().trim_matches([' ', '\t', '\n', '\r']).is_empty()) {
                node.finish = Some(|node| {
                    if node.children().is_empty() {
                        let span = Span { start: node.span.end, end: node.span.end };
                        let blank = NodeKind::Extension { name: BlankLineData::NAME.into(), data: serde_json::json!({}) };
                        node.content = DraftContent::Nodes(vec![DraftNode::leaf(span, blank)]);
                    }
                });
            }
        }
        Ok(Some(found))
    }).named("quote").produces::<BlankLineData>()
    });
    &RULE
}

pub fn plugin() -> &'static Plugin {
    static PLUGIN: std::sync::LazyLock<Plugin> = std::sync::LazyLock::new(|| {
        let mut plugin = crate::syntax::trap::GROUP.new().named("compat");
        plugin.add(blank_rule());
        plugin.add(quote_rule());
        plugin
    });
    &PLUGIN
}
