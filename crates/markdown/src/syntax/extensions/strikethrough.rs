use crate::syntax::commonmark::inlines::emphasis::paired;
use crate::{
    NodeKind,
    ast::ExtensionData,
    engine::{Plugin, inline::InlineRule},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "contracts", derive(ts_rs::TS, schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct StrikethroughData {}
impl ExtensionData for StrikethroughData {
    const NAME: &'static str = "generic/strikethrough@1";
}

pub fn inline_rule() -> &'static InlineRule {
    static RULE: std::sync::LazyLock<InlineRule> = std::sync::LazyLock::new(|| {
        InlineRule::new(b"~", |input, budget| {
            paired(input, budget, 2, false, false, |_| NodeKind::Extension {
                name: StrikethroughData::NAME.into(),
                data: serde_json::json!({}),
            })
        })
        .named("strikethrough")
        .produces::<StrikethroughData>()
    });
    &RULE
}

pub fn plugin() -> &'static Plugin {
    static PLUGIN: std::sync::LazyLock<Plugin> = std::sync::LazyLock::new(|| {
        let mut plugin = crate::syntax::extensions::GROUP
            .new()
            .named("strikethrough");
        plugin.add(inline_rule());
        plugin
    });
    &PLUGIN
}
