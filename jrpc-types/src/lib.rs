//! This crate implements the structures as defined in the [JSON-RPC 2.0 Specification](https://www.jsonrpc.org/specification)
//! 
//! ## Requirements
//! 
//! These requirements derived from the specification itself.
//! 
//! - The "jsonrpc" field MUST be present in requests, responses, and notifications. Futhormore, the value of the "jsonrpc" field MUST be "2.0".
//! - The "id" field MUST contain a String, Number, or Null value in any request or response.
//!   - The Null value for "id" in a request object is discouraged, as it's used for responses in which the original request id is unknown. The crate MUST log a warning if requests are created with Null value.
//!   - The Number value for "id" SHOULD NOT contain fractional parts, as many decimal fractions cannot be represented exactly as binary fraction. The crate MUST log a warning if requests are created with a fractional value (i.e. float).
//! - The "method" field MUST contain a String in any request.
//! - The optional "params" field MUST be either an Object (by-name) or Array (by-position) IF the "params" are present in the request or notification.
//! 
//! ## Features
//! 
//! - **log** (default): turn this feature off to supress any warning logs.
//! 
//! ## Dependencies
//! 
//! - **serde**: used for (de)serialization
//! - **serde_json**: used for JSON de(serialization) implementation
//! - **thiserror**: used for error reporting
//! - **log** (optional): used for any warning logs required to be omitted.
//! 
//! ## Usage
//! 
//! ### Requests
//! 
//! You can easily build a JSON-RPC request:
//! ```rust
//! use jrpc_types::JsonRpcRequest;
//! 
//! let data = vec![10, 293, 2, 193, 2];
//! let req = JsonRpcRequest::builder()
//!     .method("sort")
//!     .params(data).unwrap() // Serialization of parameters could fail, so you need to catch this.
//!     .id(2)
//!     .build();
//! ```
//! 
//! Deserializing a request from a string is super easy:
//! ```rust
//! use jrpc_types::JsonRpcRequest;
//! 
//! let data = r#"{"jsonrpc": "2.0", "method": "subtract", "params": [42, 23], "id": 1}"#;
//! match TryInto::<JsonRpcRequest>::try_into(data) {
//!     Ok(req) => {
//!         // .. do something ..
//!     }
//!     Err(e) => {
//!         // .. handle error ..
//!     }
//! }
//! ```
//! 
//! ### Notifications
//! 
//! JSON-RPC Notifications are pretty much Requests, without an ID... You can build Notifications like:
//! ```rust
//! use jrpc_types::JsonRpcNotification;
//! 
//! let data = vec![10, 293, 2, 193, 2];
//! let req = JsonRpcNotification::builder()
//!     .method("event")
//!     .params(data).unwrap() // Serialization of parameters could fail, so you need to catch this.
//!     .build();
//! ```
//! 
//! Deserializing a notification from a string is super easy:
//! ```rust
//! use jrpc_types::JsonRpcNotification;
//! 
//! let data = r#"{"jsonrpc": "2.0", "method": "update", "params": [1,2,3,4,5]}"#;
//! match TryInto::<JsonRpcNotification>::try_into(data) {
//!     Ok(req) => {
//!         // .. do something ..
//!     }
//!     Err(e) => {
//!         // .. handle error ..
//!     }
//! }
//! ```
//! 
//! ### Response
//! 
//! A JSON-RPC response contains different fields depending on success or error.
//! There is also a select set of (code, message) pairs for errors that are defined in the spec.
//! The builder implementation makes it incredibly easy to build these responses.
//! 
//! Success response (w/ optional data):
//! ```rust
//! use jrpc_types::JsonRpcResponse;
//! 
//! let data = vec![10, 293, 2, 193, 2];
//! let req = JsonRpcResponse::builder()
//!     .success()
//!     .result(data).unwrap() // Serialization of parameters could fail, so you need to catch this.
//!     .id(2)
//!     .build();
//! ```
//! 
//! Invalid request error:
//! ```rust
//! use jrpc_types::JsonRpcResponse;
//! 
//! let data = vec![10, 293, 2, 193, 2];
//! let req = JsonRpcResponse::builder()
//!     .error()
//!     .invalid_request()
//!     .id(2)
//!     .build();
//! ```
//! 
//! Custom Error:
//! ```rust
//! use jrpc_types::JsonRpcResponse;
//! 
//! let data = vec![10, 293, 2, 193, 2];
//! let req = JsonRpcResponse::builder()
//!     .error()
//!     .code(-23)
//!     .message("bad request man...")
//!     .id(2)
//!     .build();
//! ```
//! 
//! **NOTE**: If you are processing a JsonRpcRequest and building a JsonRpcResponse, you can use &JsonRpcRequest in the id() builder function.
//! ```rust
//! use jrpc_types::{JsonRpcRequest, JsonRpcResponse};
//! 
//! // Just for docs, creating this here.. assume you have this.
//! let req = JsonRpcRequest::builder()
//!     .id(10)
//!     .method("subtract")
//!     .params_str("[5,2]").unwrap() // Serialization of parameters could fail, so you need to catch this.
//!     .build();
//! 
//! // ... process stuff ...
//! 
//! let rsp = JsonRpcResponse::builder()
//!     .id(&req)
//!     .success()
//!     .result(3).unwrap() // Serialization of parameters could fail, so you need to catch this.
//!     .build();
//! ```

pub mod error;
pub mod id;
pub mod version;
pub mod params;
pub mod request;
pub mod notification;
pub mod response;

pub use error::Error as JsonRpcError;
pub use request::Request as JsonRpcRequest;
pub use notification::Notification as JsonRpcNotification;
pub use response::Response as JsonRpcResponse;