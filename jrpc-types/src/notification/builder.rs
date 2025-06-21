//! This module implements a Builder class for the Request object.

use crate::{error::Error, notification::Notification, params::Params};

// =======================
// Type State Structs
// =======================
pub struct MethodNone;
pub struct Method(String);
// =======================

/// The Builder class for a Request object.
pub struct Builder<M> {
    method: M,
    params: Option<Params>,
}

impl Default for Builder<MethodNone> {
    fn default() -> Self {
        Self::new()
    }
}

impl Builder<MethodNone> {
    pub fn new() -> Self {
        Builder { method: MethodNone, params: None }
    }
}

impl<M> Builder<M> {
    pub fn params<T: serde::Serialize>(self, p: T) -> Result<Builder<M>, Error> {
        let value = serde_json::to_value(p).map_err(Error::from)?;
        let params = Params::try_from(value)?;
        Ok(Builder {
            method: self.method,
            params: Some(params),
        })
    }

    pub fn params_str(self, p: &str) -> Result<Builder<M>, Error> {
        let params = Params::try_from(p)?;
        Ok(Builder {
            method: self.method,
            params: Some(params),
        })
    }
}

impl Builder<MethodNone> {
    pub fn method(self, m: &str) -> Builder<Method> {
        Builder { method: Method(m.to_string()), params: self.params }
    }
}

impl Builder<Method> {
    pub fn build(self) -> Notification {
        Notification { jsonrpc: "2.0".to_string(), method: self.method.0, params: self.params }
    }
}