use markdown_parser::{
    ParseError,
    engine::{
        Budget, Plugin,
        inline::{InlineInput, InlineMatch, InlineRule},
    },
};
pub use markdown_trap_contracts::{ReferenceData, ReferenceKind};

pub fn inline_rule() -> &'static InlineRule {
    static RULE: std::sync::LazyLock<InlineRule> =
        std::sync::LazyLock::new(|| InlineRule::new(b"!", parse).named("json_reference"));
    &RULE
}

pub fn plugin() -> &'static Plugin {
    static PLUGIN: std::sync::LazyLock<Plugin> = std::sync::LazyLock::new(|| {
        let mut plugin = Plugin::new(&markdown_trap_contracts::preset().references);
        plugin.add(inline_rule());
        plugin
    });
    &PLUGIN
}

fn parse(input: &InlineInput<'_>, budget: &mut Budget) -> Result<Option<InlineMatch>, ParseError> {
    if !input.tail().starts_with("!{") {
        return Ok(None);
    }
    let bytes = input.source.text().as_bytes();
    let mut stack = vec![];
    let (mut quote, mut escaped, mut end) = (false, false, None);
    for (index, &byte) in bytes.iter().enumerate().skip(input.position + 1) {
        budget.spend(1)?;
        if quote {
            if escaped {
                escaped = false;
            } else if byte == b'\\' {
                escaped = true;
            } else if byte == b'"' {
                quote = false;
            }
            continue;
        }
        match byte {
            b'"' => quote = true,
            b'{' | b'[' => {
                stack.push(byte);
                budget.depth(stack.len())?;
            }
            b'}' | b']' => {
                if stack.pop() != Some(if byte == b'}' { b'{' } else { b'[' }) {
                    return Ok(None);
                }
                if stack.is_empty() {
                    end = Some(index + 1);
                    break;
                }
            }
            _ => {}
        }
    }
    let Some(end) = end else {
        return Ok(None);
    };
    budget.spend(end - input.position)?;
    let Ok(value) =
        serde_json::from_str::<serde_json::Value>(&input.source.text()[input.position + 1..end])
    else {
        return Ok(None);
    };
    let (Some(target), Some(id), Some(label)) = (
        value.get("type").and_then(|v| v.as_str()),
        value.get("id").and_then(|v| v.as_str()),
        value.get("raw").and_then(|v| v.as_str()),
    ) else {
        return Ok(None);
    };
    let target = match target {
        "user" => ReferenceKind::User,
        "group" => ReferenceKind::Group,
        "channel" => ReferenceKind::Channel,
        _ => return Ok(None),
    };
    let data = ReferenceData {
        target,
        id: id.into(),
        label: label.into(),
    };
    Ok(Some(InlineMatch::leaf(
        end,
        markdown_parser::NodeKind::new(data),
    )))
}
