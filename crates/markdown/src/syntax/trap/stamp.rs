use crate::{
    ast::ExtensionData,
    engine::{
        Plugin,
        inline::{InlineMatch, InlineRule},
    },
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "contracts", derive(ts_rs::TS, schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct StampData {
    pub literal: String,
}
impl ExtensionData for StampData {
    const NAME: &'static str = "trap/stamp@1";
}

pub fn inline_rule() -> &'static InlineRule {
    static RULE: std::sync::LazyLock<InlineRule> = std::sync::LazyLock::new(|| {
        InlineRule::new(b":", |input, budget| {
        static STAMP: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| regex::Regex::new(r#"^:(?:[a-zA-Z0-9+_-]{1,32}|@(?:Webhook#)?[a-zA-Z0-9_-]+|[a-zA-Z0-9_]+\([^:<>"'=+!?]+\))(?:\.[a-zA-Z0-9_+.-]+)?:"#).unwrap());
        let tail = input.tail();
        let limit = tail[1..].find(':').map_or(tail.len(), |n| n + 2);
        budget.spend(limit)?;
        let Some(matched) = STAMP.find(&tail[..limit]) else { return Ok(None); };
        let data = StampData { literal: tail[..matched.end()].into() };
        Ok(Some(InlineMatch::leaf(input.position + matched.end(), data.node_kind()?)))
    }).named("stamp").produces::<StampData>()
    });
    &RULE
}

pub fn plugin() -> &'static Plugin {
    static PLUGIN: std::sync::LazyLock<Plugin> = std::sync::LazyLock::new(|| {
        let mut plugin = crate::syntax::trap::GROUP.new().named("stamp");
        plugin.add(inline_rule());
        plugin
    });
    &PLUGIN
}
