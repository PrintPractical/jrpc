//! This module implements a Builder class for the Request object.

use crate::{error::Error, id::Id as JId, params::Params, request::Request};

// =======================
// Type State Structs
// =======================
pub struct MethodNone;
pub struct Method(String);
pub struct IdNone;
pub struct Id(JId);
// =======================

/// The Builder class for a Request object.
pub struct Builder<M, I> {
    method: M,
    params: Option<Params>,
    id: I,
}

impl Builder<MethodNone, IdNone> {
    pub fn new() -> Self {
        Builder { method: MethodNone, params: None, id: IdNone }
    }
}

impl<M, I> Builder<M, I> {
    pub fn params<T: serde::Serialize>(self, p: T) -> Result<Builder<M, I>, Error> {
        let value = serde_json::to_value(p).map_err(Error::from)?;
        let params = Params::try_from(value)?;
        Ok(Builder {
            method: self.method,
            params: Some(params),
            id: self.id,
        })
    }

    pub fn params_str(self, p: &str) -> Result<Builder<M, I>, Error> {
        let params = Params::try_from(p)?;
        Ok(Builder {
            method: self.method,
            params: Some(params),
            id: self.id,
        })
    }
}

impl<I> Builder<MethodNone, I> {
    pub fn method(self, m: &str) -> Builder<Method, I> {
        Builder { method: Method(m.to_string()), params: self.params, id: self.id }
    }
}

impl<M> Builder<M, IdNone> {
    pub fn id<T: Into<JId>>(self, i: T) -> Builder<M, Id> {
        Builder { method: self.method, params: self.params, id: Id(i.into()) }
    }
}

impl Builder<Method, Id> {
    pub fn build(self) -> Request {
        Request { jsonrpc: "2.0".to_string(), method: self.method.0, params: self.params, id: self.id.0 }
    }
}