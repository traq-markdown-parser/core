mod cells;
mod parse;
use crate::{
    ast::ExtensionData,
    engine::{
        Plugin,
        block::{BlockRule, Interrupt},
    },
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "contracts", derive(ts_rs::TS, schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct TableData {}
impl ExtensionData for TableData {
    const NAME: &'static str = "generic/table@1";
}
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "contracts", derive(ts_rs::TS, schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct RowData {
    pub header: bool,
}
impl ExtensionData for RowData {
    const NAME: &'static str = "generic/table_row@1";
}
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "contracts", derive(ts_rs::TS, schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct CellData {
    pub alignment: Option<Alignment>,
}
impl ExtensionData for CellData {
    const NAME: &'static str = "generic/table_cell@1";
}

pub fn block_rule() -> &'static BlockRule {
    static RULE: std::sync::LazyLock<BlockRule> = std::sync::LazyLock::new(|| {
        BlockRule::new(parse::parse)
            .named("table")
            .interrupts(|probe| {
                probe.context == Interrupt::Paragraph
                    && probe
                        .next
                        .is_some_and(|next| cells::alignment(probe.line, next).is_some())
            })
            .produces::<TableData>()
            .produces::<RowData>()
            .produces::<CellData>()
    });
    &RULE
}

pub fn plugin() -> &'static Plugin {
    static PLUGIN: std::sync::LazyLock<Plugin> = std::sync::LazyLock::new(|| {
        let mut plugin = crate::syntax::extensions::GROUP.new().named("table");
        plugin.add(block_rule());
        plugin
    });
    &PLUGIN
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[cfg_attr(feature = "contracts", derive(ts_rs::TS, schemars::JsonSchema))]
#[serde(rename_all = "lowercase")]
pub enum Alignment {
    Left,
    Center,
    Right,
}
