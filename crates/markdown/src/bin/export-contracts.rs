use std::collections::BTreeMap;
use traq_markdown::{Document, NodeKind, ParseError, bindings};
use ts_rs::{Config, TS};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args().nth(1).ok_or("Pass the output directory")?;
    let config = Config::default()
        .with_out_dir(&path)
        .with_import_extension(Some("js"));
    let mut schemas = BTreeMap::new();
    let catalog = bindings::bundled();
    let mut definitions = vec![];
    for definition in catalog.extensions() {
        if register_schema(&mut schemas, definition.name, (definition.schema)())? {
            definitions.push(definition);
        }
    }
    Document::export_all(&config)?;
    NodeKind::export_all(&config)?;
    ParseError::export_all(&config)?;
    for definition in definitions {
        (definition.export)(&config)?;
    }
    let manifest = serde_json::json!({"catalog":catalog.describe(),"extensions":schemas});
    std::fs::create_dir_all(&path)?;
    std::fs::write(
        std::path::Path::new(&path).join("contracts.json"),
        serde_json::to_vec_pretty(&manifest)?,
    )?;
    Ok(())
}

fn register_schema(
    schemas: &mut BTreeMap<&'static str, serde_json::Value>,
    name: &'static str,
    schema: serde_json::Value,
) -> Result<bool, String> {
    if let Some(previous) = schemas.get(name) {
        return if previous == &schema {
            Ok(false)
        } else {
            Err(format!(
                "Conflicting payload contract across profiles: {name}"
            ))
        };
    }
    let title = schema["title"]
        .as_str()
        .ok_or("Missing payload type name")?;
    if schemas.values().any(|s| s["title"].as_str() == Some(title)) {
        return Err(format!("Duplicate payload type name: {title}"));
    }
    schemas.insert(name, schema);
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn profiles_share_identical_contracts_but_cannot_overwrite_them() {
        let mut schemas = BTreeMap::new();
        let schema = json!({"title": "ExampleData", "type": "object"});
        assert!(register_schema(&mut schemas, "test/example@1", schema.clone()).unwrap());
        assert!(!register_schema(&mut schemas, "test/example@1", schema.clone()).unwrap());
        let before = schemas.clone();
        assert!(register_schema(&mut schemas, "other/example@1", schema).is_err());
        assert!(
            register_schema(
                &mut schemas,
                "test/example@1",
                json!({"title": "ExampleData", "type": "string"})
            )
            .is_err()
        );
        assert_eq!(schemas, before);
    }
}
