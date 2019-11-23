use quick_xml::Error as QuickXmlError;
use serde_json::Error as SerdeJsonError;
use std::str::Utf8Error;

#[derive(Debug)]
pub enum Error {
    XmlParseStringError(Utf8Error),
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
