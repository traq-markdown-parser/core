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
pub struct SpoilerData {}
impl ExtensionData for SpoilerData {
    const NAME: &'static str = "trap/spoiler@1";
}

/// frontPrior consumes pairs and leaves an odd '!' for JSON or image syntax.
pub fn inline_rule() -> &'static InlineRule {
    static RULE: std::sync::LazyLock<InlineRule> = std::sync::LazyLock::new(|| {
        InlineRule::new(b"!", |input, budget| {
            paired(input, budget, 2, false, false, |_| NodeKind::Extension {
                name: SpoilerData::NAME.into(),
                data: serde_json::json!({}),
            })
        })
        .named("spoiler")
        .produces::<SpoilerData>()
    });
    &RULE
}

pub fn plugin() -> &'static Plugin {
    static PLUGIN: std::sync::LazyLock<Plugin> = std::sync::LazyLock::new(|| {
        let mut plugin = crate::syntax::trap::GROUP.new().named("spoiler");
        plugin.add(inline_rule());
        plugin
    });
    &PLUGIN
}
