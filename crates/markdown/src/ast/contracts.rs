/// Optional build-time reflection. No schema generation code enters the Wasm runtime.
#[cfg(feature = "contracts")]
pub trait Contract: ts_rs::TS + schemars::JsonSchema + 'static {}
#[cfg(feature = "contracts")]
impl<T: ts_rs::TS + schemars::JsonSchema + 'static> Contract for T {}
#[cfg(not(feature = "contracts"))]
pub trait Contract: 'static {}
#[cfg(not(feature = "contracts"))]
impl<T: 'static> Contract for T {}
