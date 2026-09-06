use serde::{Deserialize, Serialize};

/// Node data is separate from children so traversal never depends on extensions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "contracts", derive(ts_rs::TS))]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum NodeKind {
    Paragraph,
    Heading {
        level: u8,
    },
    Blockquote,
    List {
        ordered: bool,
        start: u32,
        tight: bool,
    },
    ListItem {
        marker: String,
    },
    CodeBlock {
        fenced: bool,
        info: String,
        literal: String,
    },
    ThematicBreak {
        marker: String,
    },
    Text {
        value: String,
    },
    Softbreak,
    Hardbreak,
    InlineCode {
        literal: String,
    },
    Emphasis,
    Strong,
    Link {
        destination: String,
        title: Option<String>,
        form: LinkForm,
    },
    Image {
        destination: String,
        title: Option<String>,
        label_source: String,
    },
    HtmlInline {
        literal: String,
    },
    HtmlBlock {
        literal: String,
    },
    Extension {
        name: String,
        data: serde_json::Value,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "contracts", derive(ts_rs::TS))]
#[serde(rename_all = "snake_case")]
pub enum LinkForm {
    Explicit,
    Autolink,
    Linkify,
}

impl NodeKind {
    pub(crate) fn valid_shape(&self, has_children: bool) -> bool {
        if matches!(self, Self::Heading { level } if !(1..=6).contains(level)) {
            return false;
        }
        !has_children
            || matches!(
                self,
                Self::Paragraph
                    | Self::Heading { .. }
                    | Self::Blockquote
                    | Self::List { .. }
                    | Self::ListItem { .. }
                    | Self::Emphasis
                    | Self::Strong
                    | Self::Link { .. }
                    | Self::Image { .. }
                    | Self::Extension { .. }
            )
    }

    pub fn text(value: impl Into<String>) -> Self {
        Self::Text {
            value: value.into(),
        }
    }
}
