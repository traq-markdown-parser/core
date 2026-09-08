use markdown_definitions::Plugin as Declaration;
use markdown_parser::{
    BuildError, GrammarBuilder, NodeData, Parser, Plugin, engine::inline::InlineRule,
};

#[derive(Debug, Clone, PartialEq)]
struct Text(String);
impl NodeData for Text {}
fn plugin() -> Plugin {
    let mut plugin = Plugin::new(&Declaration::new("text"));
    plugin.text(Text);
    plugin
}

#[test]
fn mutation_and_rejected_changes_preserve_existing_snapshots() {
    let mut plugin = plugin();
    let rule = InlineRule::new(b"x", |input, _| {
        Ok(Some(markdown_parser::engine::inline::InlineMatch::literal(
            input.position + 1,
        )))
    });
    plugin.add(&rule);
    let mut builder = GrammarBuilder::new();
    builder.add(&plugin).unwrap();
    let original = builder.clone().build().unwrap();
    let mut duplicate = Plugin::new(&Declaration::new("duplicate"));
    duplicate.add(&rule);
    assert!(builder.add(&duplicate).is_err());
    let missing = InlineRule::new(b"x", |_, _| Ok(None));
    assert!(builder.before(&rule, &missing).is_err());
    assert_eq!(
        builder.clone().build().unwrap().describe(),
        original.describe()
    );

    plugin.add(InlineRule::new(b"y", |input, _| {
        Ok(Some(markdown_parser::engine::inline::InlineMatch::text(
            input.position + 1,
            "changed".into(),
        )))
    }));
    assert!(builder.remove(&plugin).is_err());
    let parser = Parser::new(&builder.build().unwrap());
    drop(original);
    assert_eq!(
        parser.parse_inline("y").unwrap().children[0].get::<Text>(),
        Some(&Text("y".into()))
    );
}

#[test]
fn rule_names_are_checked_per_plugin_and_phase() {
    for (named, expected_error) in [(true, true), (false, false)] {
        let mut plugin = plugin();
        for marker in [b"x" as &'static [u8], b"y"] {
            let rule = InlineRule::new(marker, |_, _| Ok(None));
            plugin.add(if named { rule.named("rule") } else { rule });
        }
        let mut builder = GrammarBuilder::new();
        builder.add(&plugin).unwrap();
        assert_eq!(
            matches!(builder.build(), Err(BuildError::DuplicateName { .. })),
            expected_error
        );
    }
}
