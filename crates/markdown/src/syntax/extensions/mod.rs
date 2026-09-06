pub mod linkify;
pub mod mark;
pub mod math;
pub mod strikethrough;
pub mod table;

/// Shared namespace for the built-in definitions.
pub static GROUP: std::sync::LazyLock<crate::engine::PluginGroup> =
    std::sync::LazyLock::new(|| crate::engine::Plugin::group().named("generic"));
