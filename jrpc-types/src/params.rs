//! This module implements the structure for the "params" field in JSON-RPC objects.

use std::{fmt::Write, ops::{Deref, DerefMut}};

use crate::error::Error;

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
/// This object implements the "params" field in JSON-RPC objects.
/// 
/// "params" can only be a Structured Value (by-name, by-position)
pub struct Params(pub serde_json::Value);

impl Deref for Params {
    type Target = serde_json::Value;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Params {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'de> serde::Deserialize<'de> for Params {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        match value {
            serde_json::Value::Object(_) | serde_json::Value::Array(_) => Ok(Params(value)),
            _ => Err(serde::de::Error::custom(
                r#""params" must be a JSON object or array"#,
            )),
        }
    }
}

impl TryFrom<&str> for Params {
    type Error = Error;
    
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(value).map_err(|e| e.into())
    }
}

impl TryFrom<Params> for String {
    type Error = Error;

    fn try_from(value: Params) -> Result<Self, Self::Error> {
        serde_json::to_string(&value).map_err(|e| e.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_object() {
        let params = r#"{"val":123}"#;
        let params_obj = TryInto::<Params>::try_into(params);
        assert!(params_obj.is_ok());
    }

    #[test]
    fn params_array() {
        let params = r#"["hello","hi"]"#;
        let params_obj = TryInto::<Params>::try_into(params);
        assert!(params_obj.is_ok());
    }

    #[test]
    fn params_negative_tests() {
        let params = r#"12"#;
        let params_obj = TryInto::<Params>::try_into(params);
        assert!(params_obj.is_err());

        let params = r#""hello""#;
        let params_obj = TryInto::<Params>::try_into(params);
        assert!(params_obj.is_err());

        let params = r#""#;
        let params_obj = TryInto::<Params>::try_into(params);
        assert!(params_obj.is_err());

        let params = r#"null"#;
        let params_obj = TryInto::<Params>::try_into(params);
        assert!(params_obj.is_err());
    }
}