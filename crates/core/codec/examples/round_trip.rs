use markdown_ast::{Document, Node, Span};
use markdown_codec::Codec;
use markdown_commonmark_contracts::{Heading, Text};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut codec = Codec::default();
    codec.register::<Heading>()?;
    codec.register::<Text>()?;
    let document = Document {
        source: "# hello".into(),
        children: vec![Node::new(
            Span { start: 0, end: 7 },
            Heading { level: 1 },
            vec![Node::leaf(
                Span { start: 2, end: 7 },
                Text {
                    value: "hello".into(),
                },
            )],
        )],
    };
    let json = codec.encode(&document)?;
    let decoded = codec.decode(&json)?;
    assert_eq!(document, decoded);
    println!("{}", String::from_utf8(json)?);
    Ok(())
}
