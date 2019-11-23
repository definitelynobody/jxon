use crate::{constants::*, error::Error, *};
use quick_xml::{events::*, Writer};
use serde_json::Value;
use std::io::Cursor;

fn is_attribute_property_name(name: &str) -> bool {
    name.find(ATTRIBUTE_START_CHARACTER) == Some(0)
}

fn is_decl(name: &str) -> bool {
    name == DECL_STRING
}

fn write_value(writer: &mut Writer<Cursor<Vec<u8>>>, value: Value) -> Result<(), Error> {
    match value {
        Value::Null => return Err(Error::JsonParseUnexpectedNull),
        Value::Bool(_) => return Err(Error::JsonParseUnexpectedBool),
        Value::Number(_) => return Err(Error::JsonParseUnexpectedNumber),
        Value::String(string) => {
            writer
                .write_event(Event::Text(BytesText::from_escaped_str(&string)))
                .map_err(|e| Error::XmlQuickXmlError(e))?;
        }
        Value::Array(_) => return Err(Error::JsonParseUnexpectedArray),
        Value::Object(map) => {
            for (key, value) in map {
                if is_attribute_property_name(&key) {
                    continue;
                }

                if is_decl(&key) {
                    writer
                        .write_event(Event::Decl(BytesDecl::new(
                            value
                                .get("version")
                                .ok_or(Error::JsonParseDeclMissingVersion)?
                                .as_str()
                                .ok_or(Error::JsonParseInvalidDecl)?
                                .as_bytes(),
                            match value.get("encoding") {
                                Some(v) => {
                                    Some(v.as_str().ok_or(Error::JsonParseInvalidDecl)?.as_bytes())
                                }
                                None => None,
                            },
                            match value.get("standalone") {
                                Some(v) => {
                                    Some(v.as_str().ok_or(Error::JsonParseInvalidDecl)?.as_bytes())
                                }
                                None => None,
                            },
                        )))
                        .map_err(|e| Error::XmlQuickXmlError(e))?;

                    continue;
                }

                match value {
                    Value::String(_) => write_value(writer, value)?,
                    Value::Array(values) => {
                        for value in values {
                            let has_children = match &value {
                                Value::Null
                                | Value::Bool(_)
                                | Value::Number(_)
                                | Value::String(_) => false,
                                Value::Array(array) => array.is_empty(),
                                Value::Object(object) => object
                                    .iter()
                                    .find(|(key, _)| !is_attribute_property_name(key))
                                    .is_some(),
                            };
                            let mut bytes_start = BytesStart::borrowed(key.as_bytes(), key.len());

                            match &value {
                                Value::Object(object) => {
                                    for (key, value) in object.iter() {
                                        if is_attribute_property_name(key) {
                                            bytes_start.push_attribute((
                                                key.get(1..)
                                                    .ok_or(Error::JsonParseInvalidAttributeName)?,
                                                match value {
                                                    Value::String(string) => string,
                                                    _ => {
                                                        return Err(
                                                            Error::JsonParseInvalidAttributeValue,
                                                        )
                                                    }
                                                }
                                                .as_str(),
                                            ));
                                        }
                                    }
                                }
                                _ => return Err(Error::JsonParseExpectedObject),
                            }

                            if !has_children {
                                writer
                                    .write_event(Event::Empty(bytes_start))
                                    .map_err(|e| Error::XmlQuickXmlError(e))?;
                            } else {
                                writer
                                    .write_event(Event::Start(bytes_start))
                                    .map_err(|e| Error::XmlQuickXmlError(e))?;

                                write_value(writer, value)?;

                                writer
                                    .write_event(Event::End(BytesEnd::borrowed(key.as_bytes())))
                                    .map_err(|e| Error::XmlQuickXmlError(e))?;
                            }
                        }
                    }
                    _ => return Err(Error::JsonParseExpectedArray),
                }
            }
        }
    }

    Ok(())
}

/// Convert a JSON string to an XML string.
/// The JSON provided must be compatible with the conventions used by the jxon crate.
pub fn json_to_xml(json: &str, indent: Option<(u8, usize)>) -> Result<String, Error> {
    let mut writer = match indent {
        None => Writer::new(Cursor::new(Vec::new())),
        Some((c, size)) => Writer::new_with_indent(Cursor::new(vec![]), c, size),
    };
    write_value(
        &mut writer,
        serde_json::from_str(json).map_err(|e| Error::JsonParseError(e))?,
    )?;
    bytes_to_string(&writer.into_inner().into_inner())
}
