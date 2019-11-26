mod constants;
mod error;
mod to_json;
mod to_xml;

pub use error::Error;
use serde::{de::DeserializeOwned, Serialize};
use std::str::from_utf8;
pub use to_json::xml_to_json;
pub use to_xml::json_to_xml;

fn bytes_to_string(bytes: &[u8]) -> Result<String, Error> {
    from_utf8(bytes)
        .map(|s| s.to_owned())
        .map_err(|e| Error::ParseStringError(e))
}

/// Serialize a JXON compatible struct into an XML string.
pub fn serialize<T: Serialize>(t: T, indent: Option<(u8, usize)>) -> Result<String, Error> {
    json_to_xml(
        &serde_json::to_string(&t).map_err(|e| Error::JsonParseError(e))?,
        indent,
    )
}

/// Deserialize an XML string into a JXON compatible struct.
pub fn deserialize<T: DeserializeOwned>(xml: &str) -> Result<T, Error> {
    serde_json::from_value(xml_to_json(xml)?).map_err(|e| Error::JsonParseError(e))
}
