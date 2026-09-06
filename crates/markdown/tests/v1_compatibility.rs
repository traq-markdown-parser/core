use serde_json::{Value, json};
use traq_markdown::presets;

fn actual(source: &str) -> Value {
    let parser = presets::traq::v1::parser();
    json!({"block": parser.parse(source), "inline": parser.parse_inline(source)})
}

#[test]
fn commonmark_inputs_preserve_the_v1_contract() {
    let inputs: Vec<Value> =
        serde_json::from_str(include_str!("fixtures/commonmark-0.31.2.json")).unwrap();
    let expected: Vec<Value> =
        serde_json::from_str(include_str!("fixtures/traq-v1-commonmark.json")).unwrap();
    assert_eq!(inputs.len(), expected.len());
    for (input, expected) in inputs.iter().zip(expected) {
        assert_eq!(input["example"], expected["example"]);
        assert_eq!(
            actual(input["markdown"].as_str().unwrap()),
            expected["expected"],
            "traQ V1, CommonMark input {}",
            input["example"]
        );
    }
}

#[test]
fn extensions_preserve_the_v1_contract() {
    let cases: Vec<Value> =
        serde_json::from_str(include_str!("fixtures/traq-v1-extensions.json")).unwrap();
    for case in cases {
        assert_eq!(
            actual(case["source"].as_str().unwrap()),
            case["expected"],
            "traQ V1 extension: {}",
            case["name"]
        );
    }
}
