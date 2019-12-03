use quick_xml::Error as QuickXmlError;
use serde_json::Error as SerdeJsonError;
use std::{fmt, str::Utf8Error};

#[derive(Debug)]
pub enum Error {
    ParseStringError(Utf8Error),
    XmlQuickXmlError(QuickXmlError),
    XmlParseUnexpectedEof,
    JsonParseError(SerdeJsonError),
    JsonParseInvalidAttributeName,
    JsonParseInvalidAttributeValue,
    JsonParseExpectedArray,
    JsonParseExpectedObject,
    JsonParseUnexpectedArray,
    JsonParseUnexpectedNull,
    JsonParseUnexpectedBool,
    JsonParseUnexpectedNumber,
    JsonParseDeclMissingVersion,
    JsonParseInvalidDecl,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ParseStringError(e) => write!(f, "Failed to parse utf8 string: {}", e),
            Error::XmlQuickXmlError(e) => write!(f, "quick-xml error: {}", e),
            Error::XmlParseUnexpectedEof => write!(f, "xml parse error: Unexpected end of file"),
            Error::JsonParseError(e) => write!(f, "json parse error:  {}", e),
            Error::JsonParseInvalidAttributeName => {
                write!(f, "json parse error: invalid attribute name")
            }
            Error::JsonParseInvalidAttributeValue => {
                write!(f, "json parse error: invalid attribute value")
            }
            Error::JsonParseExpectedArray => write!(f, "json parse error: expected an array"),
            Error::JsonParseExpectedObject => write!(f, "json parse error: expected an object"),
            Error::JsonParseUnexpectedArray => write!(f, "json parse error: unexpected array"),
            Error::JsonParseUnexpectedNull => write!(f, "json parse error: unexpected null"),
            Error::JsonParseUnexpectedBool => write!(f, "json parse error: unexpected bool"),
            Error::JsonParseUnexpectedNumber => write!(f, "json parse error: unexpected number"),
            Error::JsonParseDeclMissingVersion => {
                write!(f, "json parse error: missing xml version")
            }
            Error::JsonParseInvalidDecl => write!(f, "json parse error: invalid xml declaration"),
        }
    }
}
