//! This module implements the request JSON-RPC object.

use crate::{error::Error, id::Id, params::Params};

pub mod builder;

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
/// The JSON-RPC Request Object
pub struct Request {
    #[serde(deserialize_with="crate::version::version_deserialize")]
    jsonrpc: String,
    pub method: String,
    pub params: Option<Params>,
    pub id: Id,
}

impl Request {
    pub fn builder() -> builder::Builder<builder::MethodNone, builder::IdNone> {
        builder::Builder::new()
    }
}

impl TryFrom<&str> for Request {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(value).map_err(|e| e.into())
    }
}

impl TryFrom<Request> for String {
    type Error = Error;

    fn try_from(value: Request) -> Result<Self, Self::Error> {
        serde_json::to_string(&value).map_err(|e| e.into())
    }
}

// This is helpful for the response builder..
// You can pass a ref to Request to the id() function
impl From<&Request> for Id {
    fn from(value: &Request) -> Self {
        value.id.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_spec_requests() {
        let req = r#"{"jsonrpc": "2.0", "method": "subtract", "params": [42, 23], "id": 1}"#;
        let req_obj = TryInto::<Request>::try_into(req);
        assert!(req_obj.is_ok());
        let req_obj = req_obj.unwrap();
        assert_eq!(req_obj.jsonrpc, "2.0");
        assert_eq!(req_obj.method, "subtract");

        let req = r#"{"jsonrpc": "2.0", "method": "subtract", "params": [23, 42], "id": 2}"#;
        let req_obj = TryInto::<Request>::try_into(req);
        assert!(req_obj.is_ok());
        let req_obj = req_obj.unwrap();
        assert_eq!(req_obj.jsonrpc, "2.0");
        assert_eq!(req_obj.method, "subtract");

        let req = r#"{"jsonrpc": "2.0", "method": "subtract", "params": {"subtrahend": 23, "minuend": 42}, "id": 3}"#;
        let req_obj = TryInto::<Request>::try_into(req);
        assert!(req_obj.is_ok());
        let req_obj = req_obj.unwrap();
        assert_eq!(req_obj.jsonrpc, "2.0");
        assert_eq!(req_obj.method, "subtract");

        let req = r#"{"jsonrpc": "2.0", "method": "subtract", "params": {"minuend": 42, "subtrahend": 23}, "id": 4}"#;
        let req_obj = TryInto::<Request>::try_into(req);
        assert!(req_obj.is_ok());
        let req_obj = req_obj.unwrap();
        assert_eq!(req_obj.jsonrpc, "2.0");
        assert_eq!(req_obj.method, "subtract");

        let req = r#"{"jsonrpc": "2.0", "method": "foobar", "id": "1"}"#;
        let req_obj = TryInto::<Request>::try_into(req);
        assert!(req_obj.is_ok());
        let req_obj = req_obj.unwrap();
        assert_eq!(req_obj.jsonrpc, "2.0");
        assert_eq!(req_obj.method, "foobar");

        let req = r#"{"jsonrpc": "2.0", "method": "foobar, "params": "bar", "baz]"#;
        let req_obj = TryInto::<Request>::try_into(req);
        assert!(req_obj.is_err());

        let req = r#"{"jsonrpc": "2.0", "method": 1, "params": "bar"}"#;
        let req_obj = TryInto::<Request>::try_into(req);
        assert!(req_obj.is_err());
    }

    #[test]
    fn negative_serde_tests() {
        let req = r#"{"jsonrpc": "2.0", "method": "subtract", "params": [42, 23]}"#; // no id
        let req_obj = TryInto::<Request>::try_into(req);
        assert!(req_obj.is_err());

        let req = r#"{"jsonrpc": "2.0", "method": "subtract", "params": [42, 23], "id":{}}"#; // id is obj
        let req_obj = TryInto::<Request>::try_into(req);
        assert!(req_obj.is_err());

        let req = r#"{"jsonrpc": "2.0", "method": "subtract", "params": [42, 23], "id":[]}"#; // id is array
        let req_obj = TryInto::<Request>::try_into(req);
        assert!(req_obj.is_err());

        let req = r#"{"jsonrpc": "2.0", "method": "subtract", "params": 1, "id":2}"#; // params is number
        let req_obj = TryInto::<Request>::try_into(req);
        assert!(req_obj.is_err());

        let req = r#"{"jsonrpc": "2.0", "method": "subtract", "params": "hello", "id":2}"#; // params is string
        let req_obj = TryInto::<Request>::try_into(req);
        assert!(req_obj.is_err());

        let req = r#"{"jsonrpc": "2.1", "method": "subtract", "id":2}"#; // jsonrpc version wrong
        let req_obj = TryInto::<Request>::try_into(req);
        assert!(req_obj.is_err());

        let req = r#"{"jsonrpc": 2.0, "method": "subtract", "id":2}"#; // jsonrpc version string
        let req_obj = TryInto::<Request>::try_into(req);
        assert!(req_obj.is_err());
    }

    #[test]
    fn builder() {
        let params = vec![10, 0];
        let req = Request::builder().id(10).method("test-method").params(params).unwrap().build();
        let req_str = TryInto::<String>::try_into(req).unwrap();
        let new_req = TryInto::<Request>::try_into(req_str.as_str());
        assert!(new_req.is_ok());
    }
}