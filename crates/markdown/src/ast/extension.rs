use super::NodeKind;
use crate::ParseError;
use serde::{Serialize, de::DeserializeOwned};

/// An extension owns its object payload contract, independently of syntax versions.
pub trait ExtensionData: Serialize + DeserializeOwned + super::contracts::Contract {
    const NAME: &'static str;
    fn validate(&self) -> bool {
        true
    }

    fn node_kind(&self) -> Result<NodeKind, ParseError> {
        if !self.validate() {
            return Err(ParseError::InternalError);
        }
        let data = serde_json::to_value(self).map_err(|_| ParseError::InternalError)?;
        if !data.is_object() {
            return Err(ParseError::InternalError);
        }
        Ok(NodeKind::Extension {
            name: Self::NAME.into(),
            data,
        })
    }

    fn decode(kind: &NodeKind) -> Result<Self, ParseError> {
        let NodeKind::Extension { name, data } = kind else {
            return Err(ParseError::InternalError);
        };
        if name != Self::NAME || !data.is_object() {
            return Err(ParseError::InternalError);
        }
        let value: Self =
            serde_json::from_value(data.clone()).map_err(|_| ParseError::InternalError)?;
        if !value.validate() {
            return Err(ParseError::InternalError);
        }
        Ok(value)
    }
}

#[derive(Clone, Copy)]
pub struct ExtensionDefinition {
    pub name: &'static str,
    pub validate: fn(&serde_json::Value) -> bool,
    type_id: fn() -> std::any::TypeId,
    #[cfg(feature = "contracts")]
    pub schema: fn() -> serde_json::Value,
    #[cfg(feature = "contracts")]
    pub export: fn(&ts_rs::Config) -> Result<(), ts_rs::ExportError>,
}

impl ExtensionDefinition {
    pub(crate) fn type_id(&self) -> std::any::TypeId {
        (self.type_id)()
    }
    pub fn of<T: ExtensionData>() -> Self {
        Self {
            name: T::NAME,
            type_id: std::any::TypeId::of::<T>,
            #[cfg(feature = "contracts")]
            schema: || {
                serde_json::to_value(
                    schemars::generate::SchemaSettings::draft2020_12()
                        .with(|s| s.contract = schemars::generate::Contract::Serialize)
                        .into_generator()
                        .into_root_schema_for::<T>(),
                )
                .expect("serializable schema")
            },
            #[cfg(feature = "contracts")]
            export: T::export_all,
            validate: |data| {
                data.is_object()
                    && serde_json::from_value::<T>(data.clone()).is_ok_and(|value| value.validate())
            },
        }
    }
}
