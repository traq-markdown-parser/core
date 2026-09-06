pub mod blocks;
pub mod html;
pub mod inlines;
pub(crate) mod references;

use crate::engine::Plugin;
pub use inlines::links::LinkOptions;

/// Configured CommonMark rules expose typed anchors for extension composition.
pub struct Syntax {
    pub plugin: Plugin,
    pub inline: inlines::Rules,
    pub block: blocks::Rules,
}
impl Syntax {
    pub fn new(options: LinkOptions) -> Self {
        let inline = inlines::rules(options);
        let block = blocks::rules(options);
        let mut plugin = GROUP.new().named("core");
        inline.register(&mut plugin);
        block.register(&mut plugin);
        Self {
            plugin,
            inline,
            block,
        }
    }
}
impl Default for Syntax {
    fn default() -> Self {
        Self::new(LinkOptions::default())
    }
}

/// Shared namespace for the built-in definitions.
pub static GROUP: std::sync::LazyLock<crate::engine::PluginGroup> =
    std::sync::LazyLock::new(|| crate::engine::Plugin::group().named("commonmark"));
