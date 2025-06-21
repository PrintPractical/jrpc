//! This module implements the validation logic for JSON-RPC "jsonrpc" field.

use std::fmt::Write;

use serde::{de::Visitor, Deserializer};

pub fn version_deserialize<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de> {
    
    struct VersionVisitor;
    impl<'de> Visitor<'de> for VersionVisitor {
        type Value = String;
    
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str(r#"jsonrpc version MUST be "2.0""#)
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error, {
            match v {
                "2.0" => {
                    Ok(v.to_string())
                }
                _ => {
                    Err(
                        E::custom(
                            format!("jsonrpc version NOT 2.0: {v}")
                        )
                    )
                }
            }
        }
    }
    deserializer.deserialize_string(VersionVisitor)
}