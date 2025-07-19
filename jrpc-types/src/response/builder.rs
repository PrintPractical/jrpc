//! This module implements a Builder class for the Request object.

use crate::{
    error::Error,
    id::Id as JId,
    response::{Response, Status},
};

// =======================
// Type State Structs
// =======================
pub struct IdNone;
pub struct Id(JId);
pub struct CodeNone;
pub struct Code(i32);
pub struct MessageNone;
pub struct Message(String);
// =======================

/// The Builder class for a Request object.
pub struct Builder<I> {
    id: I,
}

impl Default for Builder<IdNone> {
    fn default() -> Self {
        Self::new()
    }
}

impl Builder<IdNone> {
    pub fn new() -> Self {
        Builder { id: IdNone }
    }

    pub fn id<T: Into<JId>>(self, i: T) -> Builder<Id> {
        Builder { id: Id(i.into()) }
    }
}

impl<I> Builder<I> {
    pub fn success(self) -> SuccessBuilder<I> {
        SuccessBuilder {
            id: self.id,
            result: None,
        }
    }

    pub fn error(self) -> ErrorBuilder<I, CodeNone, MessageNone> {
        ErrorBuilder {
            id: self.id,
            code: CodeNone,
            message: MessageNone,
            data: None,
        }
    }
}

pub struct SuccessBuilder<I> {
    id: I,
    result: Option<serde_json::Value>,
}

impl SuccessBuilder<IdNone> {
    pub fn id<T: Into<JId>>(self, i: T) -> SuccessBuilder<Id> {
        SuccessBuilder {
            id: Id(i.into()),
            result: self.result,
        }
    }
}

impl<I> SuccessBuilder<I> {
    pub fn result(self, p: serde_json::Value) -> SuccessBuilder<I> {
        SuccessBuilder {
            id: self.id,
            result: Some(p),
        }
    }

    pub fn result_serialize<T: serde::Serialize>(self, p: T) -> Result<SuccessBuilder<I>, Error> {
        let value = serde_json::to_value(p).map_err(Error::from)?;
        Ok(SuccessBuilder {
            id: self.id,
            result: Some(value),
        })
    }

    pub fn result_str(self, p: &str) -> Result<SuccessBuilder<I>, Error> {
        let value = serde_json::to_value(p).map_err(Error::from)?;
        Ok(SuccessBuilder {
            id: self.id,
            result: Some(value),
        })
    }
}

impl SuccessBuilder<Id> {
    pub fn build(self) -> Response {
        Response {
            jsonrpc: "2.0".to_string(),
            id: self.id.0,
            status: Status::Success(self.result.unwrap_or(serde_json::Value::Null)),
        }
    }
}

pub struct ErrorBuilder<I, C, M> {
    id: I,
    code: C,
    message: M,
    data: Option<serde_json::Value>,
}

impl<C, M> ErrorBuilder<IdNone, C, M> {
    pub fn id<T: Into<JId>>(self, i: T) -> ErrorBuilder<Id, C, M> {
        ErrorBuilder {
            id: Id(i.into()),
            code: self.code,
            message: self.message,
            data: self.data,
        }
    }
}

impl<I, M> ErrorBuilder<I, CodeNone, M> {
    pub fn code(self, c: i32) -> ErrorBuilder<I, Code, M> {
        ErrorBuilder {
            id: self.id,
            code: Code(c),
            message: self.message,
            data: self.data,
        }
    }
}

impl<I, C> ErrorBuilder<I, C, MessageNone> {
    pub fn message(self, m: &str) -> ErrorBuilder<I, C, Message> {
        ErrorBuilder {
            id: self.id,
            code: self.code,
            message: Message(m.to_string()),
            data: self.data,
        }
    }
}

impl<I> ErrorBuilder<I, CodeNone, MessageNone> {
    pub fn parse_error(self) -> ErrorBuilder<I, Code, Message> {
        ErrorBuilder {
            id: self.id,
            code: Code(-32700),
            message: Message("Parse error".to_string()),
            data: self.data,
        }
    }

    pub fn invalid_request(self) -> ErrorBuilder<I, Code, Message> {
        ErrorBuilder {
            id: self.id,
            code: Code(-32600),
            message: Message("Invalid Request".to_string()),
            data: self.data,
        }
    }

    pub fn method_not_found(self) -> ErrorBuilder<I, Code, Message> {
        ErrorBuilder {
            id: self.id,
            code: Code(-32601),
            message: Message("Method not found".to_string()),
            data: self.data,
        }
    }

    pub fn invalid_params(self) -> ErrorBuilder<I, Code, Message> {
        ErrorBuilder {
            id: self.id,
            code: Code(-32602),
            message: Message("Invalid params".to_string()),
            data: self.data,
        }
    }

    pub fn internal_error(self) -> ErrorBuilder<I, Code, Message> {
        ErrorBuilder {
            id: self.id,
            code: Code(-32603),
            message: Message("Internal error".to_string()),
            data: self.data,
        }
    }

    pub fn server_error<T: Into<ServerErrorCode>>(self, code: T) -> ErrorBuilder<I, Code, Message> {
        ErrorBuilder {
            id: self.id,
            code: Code(Into::<ServerErrorCode>::into(code).into()),
            message: Message("Server error".to_string()),
            data: self.data,
        }
    }
}

impl<I, C, M> ErrorBuilder<I, C, M> {
    pub fn data(self, p: serde_json::Value) -> ErrorBuilder<I, C, M> {
        ErrorBuilder {
            id: self.id,
            code: self.code,
            message: self.message,
            data: Some(p),
        }
    }

    pub fn data_serialize<T: serde::Serialize>(self, p: T) -> Result<ErrorBuilder<I, C, M>, Error> {
        let value = serde_json::to_value(p).map_err(Error::from)?;
        Ok(ErrorBuilder {
            id: self.id,
            code: self.code,
            message: self.message,
            data: Some(value),
        })
    }

    pub fn data_str(self, p: &str) -> Result<ErrorBuilder<I, C, M>, Error> {
        let value = serde_json::to_value(p).map_err(Error::from)?;
        Ok(ErrorBuilder {
            id: self.id,
            code: self.code,
            message: self.message,
            data: Some(value),
        })
    }
}

impl ErrorBuilder<Id, Code, Message> {
    pub fn build(self) -> Response {
        Response {
            jsonrpc: "2.0".to_string(),
            id: self.id.0,
            status: Status::Error {
                code: self.code.0,
                message: self.message.0,
                data: self.data,
            },
        }
    }
}

// so here we are creating a bounded int type that fails at compile time,
// to make sure server error is within range
pub struct ServerErrorCode(i32);

impl ServerErrorCode {
    pub const fn new(val: i32) -> Self {
        if val <= -32000 || val >= -32099 {
            panic!("server error code range is -32099 <--> -32000");
        }
        Self(val)
    }
}

impl From<i32> for ServerErrorCode {
    fn from(value: i32) -> Self {
        ServerErrorCode::new(value)
    }
}

impl From<ServerErrorCode> for i32 {
    fn from(value: ServerErrorCode) -> Self {
        value.0
    }
}
