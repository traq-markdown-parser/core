use serde::{Deserialize, Serialize};
use traq_markdown::{
    GrammarBuilder, NodeKind, Parser,
    ast::ExtensionData,
    engine::{
        Plugin,
        inline::{InlineMatch, InlineRule},
    },
    presets,
    syntax::extensions::math,
};

#[test]
fn identity_is_shared_and_labels_are_optional() {
    let generic = Plugin::group().named("generic");
    let github = generic.group().named("github");
    let mut plugin = github.new().named("issue");
    let rule = InlineRule::new(b"x", |input, _| {
        Ok(Some(InlineMatch::leaf(
            input.position + 1,
            NodeKind::text("issue"),
        )))
    });
    plugin.add(&rule);
    let mut builder = GrammarBuilder::new();
    builder.add(&plugin).unwrap();
    let renamed = plugin.clone().named("diagnostic rename");
    assert!(builder.add(&renamed).is_err());
    builder.remove(&renamed).unwrap();
    // A distinct rule may share a label; labels never merge definitions.
    let mut independent = Plugin::new().named("generic/github/issue");
    independent.add(InlineRule::new(b"x", |input, _| {
        Ok(Some(InlineMatch::literal(input.position + 1)))
    }));
    builder.add(&plugin).unwrap().add(&independent).unwrap();
    let grammar = builder.build().unwrap();
    assert!(grammar.describe().contains("generic/github/issue"));
    assert!(
        grammar
            .describe()
            .contains("inline: generic/github/issue/<anonymous rule>")
    );
    assert_eq!(
        Parser::new(&grammar).parse_inline("x").unwrap().children[0].kind,
        NodeKind::text("issue")
    );
}
#[test]
fn mutation_of_a_shared_definition_does_not_change_existing_grammars() {
    let mut plugin = Plugin::new();
    plugin.add(InlineRule::new(b"x", |input, _| {
        Ok(Some(InlineMatch::literal(input.position + 1)))
    }));
    let mut first = GrammarBuilder::new();
    first.add(&plugin).unwrap();
    plugin.add(InlineRule::new(b"y", |input, _| {
        Ok(Some(InlineMatch::leaf(
            input.position + 1,
            NodeKind::text("changed"),
        )))
    }));
    assert!(first.remove(&plugin).is_err());
    assert_eq!(
        Parser::new(&first.build().unwrap())
            .parse_inline("y")
            .unwrap()
            .children[0]
            .kind,
        NodeKind::text("y")
    );
}
#[test]
fn builtin_factories_return_the_same_definition() {
    let mut builder = presets::traq::v1::builder();
    assert!(builder.add(math::plugin()).is_err());
    builder.remove(math::plugin()).unwrap();
    assert!(
        !builder
            .build()
            .unwrap()
            .extension_names()
            .any(|n| n.contains("math"))
    );
}
#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "contracts", derive(ts_rs::TS, schemars::JsonSchema))]
struct First {
    value: String,
}
impl ExtensionData for First {
    const NAME: &'static str = "test/collision@1";
}
#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "contracts", derive(ts_rs::TS, schemars::JsonSchema))]
struct Second {
    other: bool,
}
impl ExtensionData for Second {
    const NAME: &'static str = "test/collision@1";
}

#[test]
fn rejected_registration_and_ordering_are_atomic() {
    let rule = InlineRule::new(b"x", |_, _| Ok(None)).produces::<First>();
    let mut plugin = Plugin::new();
    plugin.add(&rule);
    let mut builder = GrammarBuilder::new();
    builder.add(&plugin).unwrap();
    let mut duplicate = Plugin::new();
    duplicate.add(&rule);
    assert!(builder.add(&duplicate).is_err());
    let mut collision = Plugin::new();
    collision.add(InlineRule::new(b"x", |_, _| Ok(None)).produces::<Second>());
    assert!(builder.add(&collision).is_err());
    let missing = InlineRule::new(b"x", |_, _| Ok(None));
    assert!(builder.before(&rule, &missing).is_err());
    let grammar = builder.build().unwrap();
    assert_eq!(
        grammar.extension_names().collect::<Vec<_>>(),
        ["test/collision@1"]
    );
}
#[test]
fn parser_and_dispatch_can_be_shared_between_threads() {
    let parser = presets::traq::v1::parser();
    std::thread::scope(|scope| {
        for _ in 0..4 {
            scope.spawn(|| {
                for _ in 0..10 {
                    let doc = parser.parse("[a][r]\n\n[r]: /here").unwrap();
                    assert!(serde_json::to_string(&doc).unwrap().contains("/here"));
                    assert!(
                        !serde_json::to_string(&parser.parse("[a][r]").unwrap())
                            .unwrap()
                            .contains("/here")
                    );
                }
            });
        }
    });
}
