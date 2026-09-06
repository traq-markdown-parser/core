mod block;
mod inline;
use crate::{
    ast::ExtensionData,
    engine::{Plugin, block::BlockRule, inline::InlineRule},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "contracts", derive(ts_rs::TS, schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct InlineMathData {
    pub tex: String,
}
impl ExtensionData for InlineMathData {
    const NAME: &'static str = "generic/math_inline@1";
}
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "contracts", derive(ts_rs::TS, schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct BlockMathData {
    pub tex: String,
}
impl ExtensionData for BlockMathData {
    const NAME: &'static str = "generic/math_block@1";
}

/// The legacy dollar grammar has its own fixed implementation and contract.
pub fn inline_rule() -> &'static InlineRule {
    static RULE: std::sync::LazyLock<InlineRule> = std::sync::LazyLock::new(|| {
        InlineRule::new(b"$", inline::parse)
            .named("math")
            .produces::<InlineMathData>()
    });
    &RULE
}

pub fn block_rule() -> &'static BlockRule {
    static RULE: std::sync::LazyLock<BlockRule> = std::sync::LazyLock::new(|| {
        BlockRule::new(block::parse)
            .named("math")
            .interrupts(|probe| probe.line.starts_with("$$"))
            .produces::<BlockMathData>()
    });
    &RULE
}

pub fn plugin() -> &'static Plugin {
    static PLUGIN: std::sync::LazyLock<Plugin> = std::sync::LazyLock::new(|| {
        let mut plugin = crate::syntax::extensions::GROUP.new().named("math");
        plugin.add(inline_rule());
        plugin.add(block_rule());
        plugin
    });
    &PLUGIN
}
