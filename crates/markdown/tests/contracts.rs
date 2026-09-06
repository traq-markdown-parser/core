use traq_markdown::{
    GrammarBuilder, Limits, Node, NodeKind, ParseError, Parser, Span,
    engine::{
        Plugin,
        inline::{InlineAction, InlineMatch, InlineRule},
    },
    presets,
};

#[test]
fn invalid_plugin_results_never_return_partial_documents() {
    for action in [
        InlineMatch::literal(0),
        InlineMatch::literal(1), // Inside the UTF-8 character.
        InlineMatch::literal(99),
        InlineMatch::leaf(3, NodeKind::Heading { level: 99 }),
        InlineMatch::leaf(
            3,
            NodeKind::Extension {
                name: "missing@1".into(),
                data: serde_json::json!({}),
            },
        ),
        InlineMatch {
            end: 3,
            action: InlineAction::Node {
                kind: NodeKind::Emphasis,
                inhibit_brackets: false,
                children: vec![Node::leaf(
                    Span { start: 1, end: 2 },
                    NodeKind::Text {
                        value: "bad span".into(),
                    },
                )],
            },
        },
    ] {
        let action = std::sync::Mutex::new(Some(action));
        let mut plugin = Plugin::new().named("invalid");
        plugin.add(
            InlineRule::new(b"", move |_, _| Ok(action.lock().unwrap().take())).named("invalid"),
        );
        let mut builder = GrammarBuilder::new();
        builder.add(&plugin).unwrap();
        assert_eq!(
            Parser::new(&builder.build().unwrap()).parse_inline("日"),
            Err(ParseError::InternalError)
        );
    }
}

#[test]
fn budgets_apply_to_unsuccessful_rules_and_nested_results() {
    let mut plugin = Plugin::new().named("expensive");
    plugin.add(
        InlineRule::new(b"", |_, budget| {
            budget.spend(10)?;
            Ok(None)
        })
        .named("expensive"),
    );
    let mut builder = GrammarBuilder::new();
    builder.add(&plugin).unwrap();
    let grammar = builder.build().unwrap();
    let parser = Parser::new(&grammar).with_limits(Limits {
        work: 5,
        ..Limits::default()
    });
    assert!(
        matches!(parser.parse_inline("x"), Err(ParseError::ResourceLimit { resource }) if resource == "work")
    );
    for (source, limits, resource) in [
        (
            "日本",
            Limits {
                input_bytes: 5,
                ..Limits::default()
            },
            "input_bytes",
        ),
        (
            "**a *b***",
            Limits {
                depth: 2,
                ..Limits::default()
            },
            "depth",
        ),
        (
            "*a* *b*",
            Limits {
                nodes: 2,
                ..Limits::default()
            },
            "tokens",
        ),
    ] {
        assert!(
            matches!(presets::traq::v1::parser().with_limits(limits).parse(source), Err(ParseError::ResourceLimit { resource: actual }) if actual == resource)
        );
    }
}

#[test]
fn syntax_and_rendering_policy_are_separate() {
    let source = "[x](javascript:alert) <file:///a>";
    let commonmark = presets::commonmark::parser().parse_inline(source).unwrap();
    assert_eq!(
        commonmark
            .children
            .iter()
            .filter(|node| matches!(node.kind, NodeKind::Link { .. }))
            .count(),
        2
    );
    let v1 = presets::traq::v1::parser().parse_inline(source).unwrap();
    assert!(
        v1.children
            .iter()
            .all(|node| matches!(node.kind, NodeKind::Text { .. }))
    );
}
