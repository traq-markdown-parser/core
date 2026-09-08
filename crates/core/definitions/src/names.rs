use crate::{Plugin, PluginGroup};

/// A display-name collision among selected declarations, not an identity error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NameCollision {
    pub scope: String,
    pub name: String,
}
impl std::fmt::Display for NameCollision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "duplicate name {:?} in {}", self.name, self.scope)
    }
}
impl std::error::Error for NameCollision {}

/// Check selected plugins and their ancestors for equal names under one parent.
///
/// Unselected declarations do not participate. Parent identity determines the
/// scope; a slash in a display name does not create a namespace. This checks
/// neither implementation identity nor rule/handler registration.
pub fn validate_names<'a>(
    plugins: impl IntoIterator<Item = &'a Plugin>,
) -> Result<(), NameCollision> {
    let plugins: Vec<_> = plugins.into_iter().collect();
    let mut groups: Vec<&PluginGroup> = vec![];
    for plugin in &plugins {
        let mut parent = plugin.namespace();
        while let Some(group) = parent {
            if groups.contains(&group) {
                break;
            }
            groups.push(group);
            parent = group.parent();
        }
    }
    let entries: Vec<_> = groups
        .iter()
        .map(|group| (group.parent(), group.name()))
        .chain(
            plugins
                .iter()
                .map(|plugin| (plugin.namespace(), plugin.name())),
        )
        .collect();
    for (index, (parent, name)) in entries.iter().enumerate() {
        if entries[..index].contains(&(*parent, *name)) {
            let mut scope = vec![];
            let mut parent = *parent;
            while let Some(group) = parent {
                scope.push(group.name());
                parent = group.parent();
            }
            scope.reverse();
            return Err(NameCollision {
                scope: if scope.is_empty() {
                    "<root>".into()
                } else {
                    scope.join("/")
                },
                name: (*name).into(),
            });
        }
    }
    Ok(())
}
