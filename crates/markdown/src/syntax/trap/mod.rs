pub mod compat;
pub mod references;
pub mod spoiler;
pub mod stamp;

/// Shared namespace for the built-in definitions.
pub static GROUP: std::sync::LazyLock<crate::engine::PluginGroup> =
    std::sync::LazyLock::new(|| crate::engine::Plugin::group().named("trap"));
