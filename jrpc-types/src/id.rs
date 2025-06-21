//! This module implements the structure for the "id" field in JSON-RPC objects.

use crate::error::Error;

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(untagged)] // TODO: We can make this more efficient by writing our own deserialization impl
/// This object implements the "id" field in JSON-RPC objects.
/// 
/// "id" can only be String, Number (including Fractional), or Null
pub enum Id {
    String(String),
    Number(i64),
    Fractional(f32),
    Null,
}

impl From<&str> for Id {
    fn from(value: &str) -> Self {
        Id::String(value.to_string())
    }
}

impl TryFrom<Id> for String {
    type Error = Error;

    fn try_from(value: Id) -> Result<Self, Self::Error> {
        match value {
            Id::String(v) => Ok(v),
            Id::Number(_) => Err(Error::InvalidType("cannot convert Id type Number to String".to_string())),
            Id::Fractional(_) => Err(Error::InvalidType("cannot convert Id type Fractional to String".to_string())),
            Id::Null => Err(Error::InvalidType("cannot convert Id type Null to String".to_string())),
        }
    }
}

impl From<i64> for Id {
    fn from(value: i64) -> Self {
        Id::Number(value)
    }
}

impl TryFrom<Id> for i64 {
    type Error = Error;

    fn try_from(value: Id) -> Result<Self, Self::Error> {
        match value {
            Id::String(_) => Err(Error::InvalidType("cannot convert Id type String to Number".to_string())),
            Id::Number(v) => Ok(v),
            Id::Fractional(_) => Err(Error::InvalidType("cannot convert Id type Fractional to Number".to_string())),
            Id::Null => Err(Error::InvalidType("cannot convert Id type Null to Number".to_string())),
        }
    }
}

impl From<f32> for Id {
    fn from(value: f32) -> Self {
        Id::Fractional(value)
    }
}

impl TryFrom<Id> for f32 {
    type Error = Error;

    fn try_from(value: Id) -> Result<Self, Self::Error> {
        match value {
            Id::String(_) => Err(Error::InvalidType("cannot convert Id type String to Fractional".to_string())),
            Id::Number(_) => Err(Error::InvalidType("cannot convert Id type Number to Fractional".to_string())),
            Id::Fractional(v) => Ok(v),
            Id::Null => Err(Error::InvalidType("cannot convert Id type Null to Fractional".to_string())),
        }
    }
}

impl From<()> for Id {
    fn from(_: ()) -> Self {
        Id::Null
    }
}

impl TryFrom<Id> for () {
    type Error = Error;

    fn try_from(value: Id) -> Result<Self, Self::Error> {
        match value {
            Id::String(_) => Err(Error::InvalidType("cannot convert Id type String to ()".to_string())),
            Id::Number(_) => Err(Error::InvalidType("cannot convert Id type Number to ()".to_string())),
            Id::Fractional(_) => Err(Error::InvalidType("cannot convert Id type Fractional to ()".to_string())),
            Id::Null => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, serde::Deserialize, serde::Serialize)]
    struct TestObject {
        pub id: Id,
    }

    #[test]
    fn string_id() {
        let initial_id = "string-id";
        let to_id: Id = initial_id.into();
        let from_id = TryInto::<String>::try_into(to_id);
        assert!(from_id.is_ok());
        assert_eq!(from_id.unwrap(), initial_id);

        let test_obj_str = r#"{"id":"string-id"}"#;
        let _test_id = "string-id".to_string();
        let to = serde_json::from_str::<TestObject>(test_obj_str);
        assert!(to.is_ok(), "{:?}", to.unwrap_err().to_string());
        let to = to.unwrap();
        assert!(matches!(&to.id, Id::String(_test_id)));
        let to_str = serde_json::to_string(&to);
        assert!(to_str.is_ok(), "{:?}", to_str.unwrap_err().to_string());
        assert_eq!(to_str.unwrap(), test_obj_str);
    }

    #[test]
    fn number_id() {
        let initial_id = 25;
        let to_id: Id = initial_id.clone().into();
        let from_id = TryInto::<i64>::try_into(to_id);
        assert!(from_id.is_ok());
        assert_eq!(from_id.unwrap(), initial_id);

        let test_obj_str = r#"{"id":64}"#;
        let _test_id = 64i64;
        let to = serde_json::from_str::<TestObject>(test_obj_str);
        assert!(to.is_ok(), "{:?}", to.unwrap_err().to_string());
        let to = to.unwrap();
        assert!(matches!(&to.id, Id::Number(_test_id)));
        let to_str = serde_json::to_string(&to);
        assert!(to_str.is_ok(), "{:?}", to_str.unwrap_err().to_string());
        assert_eq!(to_str.unwrap(), test_obj_str);
    }

    #[test]
    fn fractional_id() {
        let initial_id = 1.2;
        let to_id: Id = initial_id.clone().into();
        let from_id = TryInto::<f32>::try_into(to_id);
        assert!(from_id.is_ok());
        assert_eq!(from_id.unwrap(), initial_id);

        let test_obj_str = r#"{"id":1.2}"#;
        let _test_id = 1.2f32;
        let to = serde_json::from_str::<TestObject>(test_obj_str);
        assert!(to.is_ok(), "{:?}", to.unwrap_err().to_string());
        let to = to.unwrap();
        assert!(matches!(&to.id, Id::Fractional(_test_id)));
        let to_str = serde_json::to_string(&to);
        assert!(to_str.is_ok(), "{:?}", to_str.unwrap_err().to_string());
        assert_eq!(to_str.unwrap(), test_obj_str);
    }

    #[test]
    fn null_id() {
        let initial_id = ();
        let to_id: Id = initial_id.clone().into();
        let from_id = TryInto::<()>::try_into(to_id);
        assert!(from_id.is_ok());
        assert_eq!(from_id.unwrap(), initial_id);

        let test_obj_str = r#"{"id":null}"#;
        let to = serde_json::from_str::<TestObject>(test_obj_str);
        assert!(to.is_ok(), "{:?}", to.unwrap_err().to_string());
        let to = to.unwrap();
        assert!(matches!(&to.id, Id::Null));
        let to_str = serde_json::to_string(&to);
        assert!(to_str.is_ok(), "{:?}", to_str.unwrap_err().to_string());
        assert_eq!(to_str.unwrap(), test_obj_str);
    }

    #[test]
    fn negative_serde_tests() {
        // id as object
        let obj = r#"{"id":{"test":"id"}}"#;
        assert!(serde_json::from_str::<TestObject>(obj).is_err());

        // id as array
        let obj = r#"{"id":["test","id"]}"#;
        assert!(serde_json::from_str::<TestObject>(obj).is_err());

        // id missing
        let obj = r#"{}"#;
        assert!(serde_json::from_str::<TestObject>(obj).is_err());
    }
}