use traq_markdown::{
    Parser,
    bindings::{Composition, GroupSpec, bundled},
    presets,
};

#[test]
fn bundled_compositions_preserve_presets_and_validate_untrusted_recipes() {
    let catalog = bundled();
    for (index, expected) in [presets::commonmark::grammar(), presets::traq::v1::grammar()]
        .into_iter()
        .enumerate()
    {
        let metadata = &catalog.describe()["presets"][index];
        assert_eq!(metadata["description"], expected.describe());
        assert_eq!(
            metadata["extensions"],
            serde_json::json!(expected.extension_names().collect::<Vec<_>>())
        );
        let recipe = catalog.preset_composition(index).unwrap();
        let encoded = serde_json::to_string(&recipe).unwrap();
        let recipe: Composition = serde_json::from_str(&encoded).unwrap();
        let grammar = catalog.build(&recipe).unwrap();
        assert_eq!(grammar.describe(), expected.describe());
        for source in [
            "**text** <i>x</i>",
            "$x$ ==x== :stamp:",
            "- item\n\t- child",
            "!{\"type\":\"user\",\"id\":\"u\",\"raw\":\"@u\"}",
        ] {
            assert_eq!(
                Parser::new(&grammar).parse(source).unwrap(),
                Parser::new(expected).parse(source).unwrap()
            );
        }
        let mut incomplete = recipe.clone();
        incomplete.order.pop();
        assert!(catalog.build(&incomplete).is_err());
        let mut duplicate = recipe.clone();
        duplicate.order[1] = duplicate.order[0];
        assert!(catalog.build(&duplicate).is_err());
        let mut unknown = recipe.clone();
        unknown.plugins[0].rules[0] = usize::MAX;
        assert!(catalog.build(&unknown).is_err());
        let mut cycle = recipe;
        cycle.groups = vec![GroupSpec {
            parent: Some(0),
            name: None,
        }];
        assert!(catalog.build(&cycle).is_err());
    }
}
