#[path = "support/html.rs"]
mod html;
use serde_json::Value;
use traq_markdown::presets;

#[test]
fn commonmark_0_31_2_specification() {
    let cases: Vec<Value> =
        serde_json::from_str(include_str!("fixtures/commonmark-0.31.2.json")).unwrap();
    let parser = presets::commonmark::parser();
    assert_eq!(cases.len(), 652);
    for case in cases {
        let document = parser.parse(case["markdown"].as_str().unwrap()).unwrap();
        assert_eq!(
            html::render(&document.children),
            case["html"].as_str().unwrap(),
            "CommonMark example {} ({})",
            case["example"],
            case["section"]
        );
    }
}
