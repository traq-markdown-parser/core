use super::{BuildError, Plugin, PluginGroup};
use std::collections::HashSet;

fn same_parent(a: Option<&PluginGroup>, b: Option<&PluginGroup>) -> bool {
    match (a, b) {
        (Some(a), Some(b)) => a == b,
        (None, None) => true,
        _ => false,
    }
}
fn duplicate(scope: String, name: &str) -> BuildError {
    BuildError::DuplicateName {
        scope,
        name: name.into(),
    }
}
/// Only namespaces reached through the selected plugins participate.
pub(super) fn validate(plugins: &[Plugin]) -> Result<(), BuildError> {
    let mut groups: Vec<&PluginGroup> = vec![];
    for plugin in plugins {
        let mut current = plugin.namespace();
        while let Some(group) = current {
            if groups.contains(&group) {
                break;
            }
            groups.push(group);
            current = group.parent();
        }
        let mut names = HashSet::new();
        for rule in plugin.definition.rules.iter() {
            if let Some(name) = rule.name()
                && !names.insert((rule.phase(), name))
            {
                return Err(duplicate(
                    format!("{} / {}", plugin.description(), rule.phase()),
                    name,
                ));
            }
        }
    }
    let entries = groups
        .iter()
        .map(|g| (g.parent(), g.name()))
        .chain(plugins.iter().map(|p| (p.namespace(), p.name())))
        .collect::<Vec<_>>();
    for (index, (parent, name)) in entries.iter().enumerate() {
        if entries[..index]
            .iter()
            .any(|(p, n)| same_parent(*p, *parent) && *n == *name)
        {
            let scope = parent.map_or_else(|| "<root>".into(), super::plugin::group_description);
            return Err(duplicate(scope, name));
        }
    }
    Ok(())
}
