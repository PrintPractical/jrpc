//! This module implements the response JSON-RPC object.

use crate::{error::Error, id::Id};

pub mod builder;

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
/// The JSON-RPC Response Object
pub struct Response {
    #[serde(deserialize_with = "crate::version::version_deserialize")]
    jsonrpc: String,
    pub id: Id,
    #[serde(flatten)]
    pub status: Status,
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
/// The Response Status can be either success or failure.
pub enum Status {
    #[serde(rename = "result")]
    Success(serde_json::Value),
    #[serde(rename = "error")]
    Error {
        code: i32,
        message: String,
        data: Option<serde_json::Value>,
    },
}

impl Response {
    pub fn builder() -> builder::Builder<builder::IdNone> {
        builder::Builder::new()
    }
}

impl TryFrom<&str> for Response {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(value).map_err(|e| e.into())
    }
}

impl TryFrom<Response> for String {
    type Error = Error;

    fn try_from(value: Response) -> Result<Self, Self::Error> {
        serde_json::to_string(&value).map_err(|e| e.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_spec_requests() {
        let req = r#"{"jsonrpc": "2.0", "result": 19, "id": 1}"#;
        let req_obj = TryInto::<Response>::try_into(req);
        assert!(req_obj.is_ok());
        let req_obj = req_obj.unwrap();
        assert_eq!(req_obj.jsonrpc, "2.0");
        assert!(matches!(
            req_obj.status,
            Status::Success(serde_json::Value::Number(_))
        ));

        let req = r#"{"jsonrpc": "2.0", "result": -19, "id": 2}"#;
        let req_obj = TryInto::<Response>::try_into(req);
        assert!(req_obj.is_ok());
        let req_obj = req_obj.unwrap();
        assert_eq!(req_obj.jsonrpc, "2.0");
        assert!(matches!(
            req_obj.status,
            Status::Success(serde_json::Value::Number(_))
        ));

        let req = r#"{"jsonrpc": "2.0", "error": {"code": -32601, "message": "Method not found"}, "id": "1"}"#;
        let req_obj = TryInto::<Response>::try_into(req);
        assert!(req_obj.is_ok());
        let req_obj = req_obj.unwrap();
        assert_eq!(req_obj.jsonrpc, "2.0");
        assert!(matches!(
            req_obj.status,
            Status::Error {
                code: _,
                message: _,
                data: _
            }
        ));

        let req = r#"{"jsonrpc": "2.0", "error": {"code": -32600, "message": "Invalid Request"}, "id": null}"#;
        let req_obj = TryInto::<Response>::try_into(req);
        assert!(req_obj.is_ok());
        let req_obj = req_obj.unwrap();
        assert_eq!(req_obj.jsonrpc, "2.0");
        assert!(matches!(
            req_obj.status,
            Status::Error {
                code: _,
                message: _,
                data: _
            }
        ));
    }

    #[test]
    fn negative_serde_tests() {
        let req = r#"{"jsonrpc": "2.1", "error": {"code": -32600, "message": "Invalid Request"}, "id": null}"#; // invalid version
        let req_obj = TryInto::<Response>::try_into(req);
        assert!(req_obj.is_err());

        let req = r#"{"jsonrpc": 2.1, "error": {"code": -32600, "message": "Invalid Request"}, "id": null}"#; // number version
        let req_obj = TryInto::<Response>::try_into(req);
        assert!(req_obj.is_err());

        let req = r#"{"jsonrpc": "2.0", "id": null}"#; // no result or error
        let req_obj = TryInto::<Response>::try_into(req);
        assert!(req_obj.is_err());

        let req = r#"{"jsonrpc": "2.0", "error": {"code": -32600, "message": "Invalid Request"}}"#; // no id
        let req_obj = TryInto::<Response>::try_into(req);
        assert!(req_obj.is_err());
    }

    #[test]
    fn builder() {
        let params = vec![10, 0];
        let rsp = Response::builder()
            .id(10)
            .success()
            .result_serialize(params)
            .unwrap()
            .build();
        let rsp_str = TryInto::<String>::try_into(rsp).unwrap();
        let new_req = TryInto::<Response>::try_into(rsp_str.as_str());
        assert!(new_req.is_ok());

        let rsp = Response::builder().error().invalid_params().id(10).build();
        let rsp_str = TryInto::<String>::try_into(rsp).unwrap();
        let new_req = TryInto::<Response>::try_into(rsp_str.as_str());
        assert!(new_req.is_ok());
    }
}
