use traq_markdown::{
    GrammarBuilder, Parser,
    engine::{BuildError, Plugin, inline::InlineRule},
    presets,
    syntax::extensions::math,
};
fn names(plugins: &[&Plugin]) -> Result<(), BuildError> {
    let mut builder = GrammarBuilder::new();
    for plugin in plugins {
        builder.add(plugin)?;
    }
    builder.build().map(|_| ())
}
#[test]
fn adopted_namespaces_are_symbols_with_optional_local_names() {
    let generic = Plugin::group().named("generic");
    let github = generic.group().named("github");
    names(&[&generic.new().named("math"), &github.new().named("math")]).unwrap();
    assert!(matches!(
        names(&[&generic.new().named("math"), &generic.new().named("math")]),
        Err(BuildError::DuplicateName { .. })
    ));
    // Repeating a label creates another symbol, so adopted root labels collide.
    let independent = Plugin::group().named("generic");
    assert!(names(&[&generic.new(), &independent.new()]).is_err());
    // Unused conflicting namespaces do not enter a composition.
    names(&[&generic.new()]).unwrap();
    names(&[
        &Plugin::new(),
        &Plugin::new(),
        &Plugin::group().new(),
        &Plugin::group().new(),
    ])
    .unwrap();
    assert!(names(&[&generic.new().named("github"), &github.new()]).is_err());
}
#[test]
fn rule_names_are_checked_per_plugin_and_phase() {
    let mut first = Plugin::new();
    first.add(InlineRule::new(b"x", |_, _| Ok(None)).named("rule"));
    first.add(InlineRule::new(b"y", |_, _| Ok(None)).named("rule"));
    assert!(matches!(
        names(&[&first]),
        Err(BuildError::DuplicateName { .. })
    ));
    let mut second = Plugin::new();
    second.add(InlineRule::new(b"x", |_, _| Ok(None)));
    second.add(InlineRule::new(b"y", |_, _| Ok(None)));
    names(&[&second]).unwrap();
}
#[test]
fn parser_owns_compiled_data_and_forks_do_not_change_the_original() {
    let original = presets::traq::v1::grammar();
    let mut builder = original.to_builder();
    builder.remove(math::plugin()).unwrap();
    let changed = builder.build().unwrap();
    let parser = Parser::new(&changed);
    drop(changed);
    let changed = serde_json::to_string(&parser.parse("$x$").unwrap()).unwrap();
    let original = serde_json::to_string(&Parser::new(original).parse("$x$").unwrap()).unwrap();
    assert!(!changed.contains("generic/math_inline"));
    assert!(original.contains("generic/math_inline"));
}
