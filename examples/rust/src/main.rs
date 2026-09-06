use traq_markdown::{Parser, presets, syntax::extensions::math};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = "**hello** :stamp: $x$";
    let parser = presets::traq::v1::parser();
    println!("{}", serde_json::to_string_pretty(&parser.parse(source)?)?);

    // A new composition leaves the original preset and Parser unchanged.
    let mut builder = presets::traq::v1::grammar().to_builder();
    builder.remove(math::plugin())?;
    let grammar = builder.build()?;
    let without_math = Parser::new(&grammar);
    drop(grammar); // The Parser retains the compiled grammar.

    println!(
        "{}",
        serde_json::to_string_pretty(&without_math.parse_inline("$x$")?)?
    );
    Ok(())
}
