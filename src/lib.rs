mod constants;
mod error;
mod to_json;
mod to_xml;

pub use error::Error;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// An XML declaration.
pub struct Declaration {
    /// Specifies the version of the XML standard used.
    pub version: String,
    /// It defines the character encoding used in
    /// the document. UTF-8 is the default encoding used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
    /// It informs the parser whether the document relies on the
    /// information from an external source, such as external document
    /// type definition (DTD), for its content. The default value
    /// is set to no. Setting it to yes tells the processor there
    /// are no external declarations required for parsing the document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standalone: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
/// A tag with no attributes and some text content.
pub struct TextContent<T> {
    /// The text inside the tag.
    #[serde(rename = "_")]
    pub content: T,
}
