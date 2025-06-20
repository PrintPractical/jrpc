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

pub mod error;
pub mod id;