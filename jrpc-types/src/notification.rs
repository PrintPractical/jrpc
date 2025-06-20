//! This module implements the notification JSON-RPC object.

use crate::{error::Error, id::Id, params::Params};

pub mod builder;

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
/// The JSON-RPC Notification Object
pub struct Notification {
    #[serde(deserialize_with="crate::version::version_deserialize")]
    jsonrpc: String,
    pub method: String,
    pub params: Option<Params>,
}

impl Notification {
    pub fn builder() -> builder::Builder<builder::MethodNone> {
        builder::Builder::new()
    }
}

impl TryFrom<&str> for Notification {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(value).map_err(|e| e.into())
    }
}

impl TryFrom<Notification> for String {
    type Error = Error;

    fn try_from(value: Notification) -> Result<Self, Self::Error> {
        serde_json::to_string(&value).map_err(|e| e.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_spec_notifications() {
        let req = r#"{"jsonrpc": "2.0", "method": "update", "params": [1,2,3,4,5]}"#;
        let req_obj = TryInto::<Notification>::try_into(req);
        assert!(req_obj.is_ok());
        let req_obj = req_obj.unwrap();
        assert_eq!(req_obj.jsonrpc, "2.0");
        assert_eq!(req_obj.method, "update");

        let req = r#"{"jsonrpc": "2.0", "method": "notify_hello", "params": [7]}"#;
        let req_obj = TryInto::<Notification>::try_into(req);
        assert!(req_obj.is_ok());
        let req_obj = req_obj.unwrap();
        assert_eq!(req_obj.jsonrpc, "2.0");
        assert_eq!(req_obj.method, "notify_hello");

        let req = r#"{"jsonrpc": "2.0", "method": "notify_hello", "params": {"hi":"hello"}}"#;
        let req_obj = TryInto::<Notification>::try_into(req);
        assert!(req_obj.is_ok());
        let req_obj = req_obj.unwrap();
        assert_eq!(req_obj.jsonrpc, "2.0");
        assert_eq!(req_obj.method, "notify_hello");
    }

    #[test]
    fn negative_serde_tests() {
        let req = r#"{"jsonrpc": "2.0", "method": "subtract", "params": 1}"#; // params is number
        let req_obj = TryInto::<Notification>::try_into(req);
        assert!(req_obj.is_err());

        let req = r#"{"jsonrpc": "2.0", "method": "subtract", "params": "hello"}"#; // params is string
        let req_obj = TryInto::<Notification>::try_into(req);
        assert!(req_obj.is_err());

        let req = r#"{"jsonrpc": "2.1", "method": "subtract"}"#; // jsonrpc version wrong
        let req_obj = TryInto::<Notification>::try_into(req);
        assert!(req_obj.is_err());

        let req = r#"{"jsonrpc": 2.0, "method": "subtract"}"#; // jsonrpc version string
        let req_obj = TryInto::<Notification>::try_into(req);
        assert!(req_obj.is_err());

        let req = r#"{"jsonrpc": 2.0, "method": "subtract", id: 2}"#; // id exists (i.e. a request)
        let req_obj = TryInto::<Notification>::try_into(req);
        assert!(req_obj.is_err());
    }

    #[test]
    fn builder() {
        let params = vec![10, 0];
        let req = Notification::builder().method("test-notification").params(params).unwrap().build();
        let req_str = TryInto::<String>::try_into(req).unwrap();
        let new_req = TryInto::<Notification>::try_into(req_str.as_str());
        assert!(new_req.is_ok());
    }
}