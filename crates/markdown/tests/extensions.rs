use serde::{Deserialize, Serialize};
use traq_markdown::{
    NodeKind, Parser,
    ast::ExtensionData,
    engine::{
        Plugin,
        block::BlockRule,
        inline::{InlineMatch, InlineRule, TextMatch, TextRule},
    },
    presets,
};

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "contracts", derive(ts_rs::TS, schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
struct Note {
    title: String,
}
impl ExtensionData for Note {
    const NAME: &'static str = "test/note@1";
}

fn note() -> (Plugin, BlockRule) {
    let block = BlockRule::new(|input, budget| {
        let Some(title) = input.current().strip_prefix(":::note ") else {
            return Ok(None);
        };
        let mut end = input.start + 1;
        while end < input.lines.len() && input.line(end) != ":::" {
            budget.spend(input.line(end).len() + 1)?;
            end += 1;
        }
        if end == input.lines.len() {
            return Ok(None);
        }
        Ok(Some(
            input.blocks(
                end + 1,
                Note {
                    title: title.into(),
                }
                .node_kind()?,
                input.start + 1..end,
            )?,
        ))
    })
    .interrupts(|probe| probe.line.starts_with(":::note "))
    .produces::<Note>();
    let mut plugin = Plugin::group().named("test").new().named("note");
    plugin.add(&block);
    plugin.add(
        InlineRule::new(b"~", |input, _| {
            if !input.tail().starts_with("~note~") {
                return Ok(None);
            }
            Ok(Some(InlineMatch::leaf(
                input.position + 6,
                Note {
                    title: "inline".into(),
                }
                .node_kind()?,
            )))
        })
        .produces::<Note>(),
    );
    plugin.add(
        TextRule::new(|input, _| {
            let Some(offset) = input.text().find("NOTE") else {
                return Ok(vec![]);
            };
            Ok(vec![TextMatch {
                start: input.range.start + offset,
                end: input.range.start + offset + 4,
                kind: Note {
                    title: "text".into(),
                }
                .node_kind()?,
                children: vec![],
            }])
        })
        .produces::<Note>(),
    );
    (plugin, block)
}
fn count(nodes: &[traq_markdown::Node]) -> usize {
    nodes
        .iter()
        .map(|n| usize::from(matches!(n.kind, NodeKind::Extension { .. })) + count(&n.children))
        .sum()
}

#[test]
fn block_bodies_resolve_nested_and_later_references() {
    let (plugin, block) = note();
    let mut builder = presets::commonmark::builder();
    builder
        .add(&plugin)
        .unwrap()
        .before(&block, &presets::commonmark::syntax().block.paragraph)
        .unwrap();
    let grammar = builder.build().unwrap();
    assert_eq!(
        grammar.extension_names().collect::<Vec<_>>(),
        ["test/note@1"]
    );
    let parser = Parser::new(&grammar);
    for source in [
        "before\n:::note A\n[one][r]\n:::\n\n[r]: /later\n",
        "> :::note A\n> [one][r]\n> :::\n\n[r]: /later\n",
        "- :::note A\n  [one][r]\n\n  [r]: /later\n  :::\n\n[outside][r]\n",
        ":::note A\n\tcode\n\n[one][r]\n:::\n\n[r]: /later\n",
        ":::note empty\n:::\n",
    ] {
        let doc = parser.parse(source).unwrap();
        assert!(count(&doc.children) > 0);
        let json = serde_json::to_string(&doc).unwrap();
        if source.contains("[one]") {
            assert!(json.contains("/later"))
        }
        assert!(!json.contains("Pending"));
    }
}
#[test]
fn removal_covers_every_phase_and_presets_do_not_leak() {
    let (plugin, block) = note();
    let source = "before\n:::note A\n~note~ NOTE\n:::\n";
    let mut builder = presets::commonmark::builder();
    builder
        .add(&plugin)
        .unwrap()
        .before(&block, &presets::commonmark::syntax().block.paragraph)
        .unwrap();
    let enabled = builder.build().unwrap();
    let mut builder = presets::commonmark::builder();
    builder.add(&plugin).unwrap().remove(&plugin).unwrap();
    let disabled = builder.build().unwrap();
    for _ in 0..3 {
        assert_eq!(
            count(&Parser::new(&enabled).parse(source).unwrap().children),
            3
        );
        assert_eq!(
            Parser::new(&disabled).parse(source).unwrap(),
            presets::commonmark::parser().parse(source).unwrap()
        );
    }
    assert_eq!(disabled.extension_names().count(), 0);
}
#[test]
fn source_helpers_reject_invalid_ranges() {
    for mode in 0..3 {
        let mut plugin = Plugin::new();
        plugin.add(BlockRule::new(move |input, _| match mode {
            0 => input.leaf(input.start, NodeKind::Paragraph).map(Some),
            1 => input
                .blocks(input.lines.len() + 1, NodeKind::Paragraph, 0..0)
                .map(Some),
            _ => input.blocks(1, NodeKind::Paragraph, 0..99).map(Some),
        }));
        let mut builder = traq_markdown::GrammarBuilder::new();
        builder.add(&plugin).unwrap();
        assert!(
            Parser::new(&builder.build().unwrap())
                .parse("text")
                .is_err()
        );
    }
}
