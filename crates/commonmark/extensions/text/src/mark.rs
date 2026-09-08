use markdown_ast::Node;
use markdown_generic_contracts::*;
use markdown_renderer::{Context, Plugin, Result};

fn children<T>(_: &T, nodes: &[Node], ctx: &Context<'_>) -> Result<String> {
    ctx.children(nodes)
}

use std::sync::LazyLock;

pub fn plugin() -> Plugin {
    static PLUGIN: LazyLock<Plugin> = LazyLock::new(|| build().expect("valid mark text plugin"));
    PLUGIN.clone()
}

fn build() -> Result<Plugin> {
    let mut mark = Plugin::new(&markdown_generic_contracts::preset().mark);
    mark.on::<MarkData>(children)?;
    Ok(mark)
}
