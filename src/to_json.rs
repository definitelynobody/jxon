use crate::{constants::*, error::Error, *};
use quick_xml::{
    events::{attributes::Attributes, *},
    Reader,
};
use serde_json::{Map, Value};
use std::io::BufRead;

fn parse_tag<B: BufRead>(
    reader: &mut Reader<B>,
    buf: &mut Vec<u8>,
    root: bool,
) -> Result<Map<String, Value>, Error> {
    let mut children = Map::new();

    loop {
        let event = reader.read_event(buf);

        let mut start_tag =
            |name: &[u8], attributes: Attributes, map: Map<String, Value>| -> Result<(), Error> {
                let mut map = map;

                for attribute in attributes {
                    let attribute = attribute.map_err(|e| Error::XmlQuickXmlError(e))?;
                    map.insert(
                        format!(
                            "{}{}",
                            ATTRIBUTE_START_CHARACTER,
                            bytes_to_string(attribute.key)?
                        ),
                        Value::String(bytes_to_string(&attribute.value)?),
                    );
                }

                let key = bytes_to_string(name)?;

                match &mut children.get_mut(&key) {
                    None => {
                        children.insert(key, Value::Array(vec![Value::Object(map)]));
                    }
                    Some(value) => {
                        value
                            .as_array_mut()
                            .ok_or(Error::JsonParseUnexpectedArray)?
                            .push(Value::Object(map));
                    }
                }

                Ok(())
            };

        match event {
            Ok(Event::Start(ref e)) => {
                let mut buf = vec![];
                start_tag(
                    e.name(),
                    e.attributes(),
                    parse_tag(reader, &mut buf, false)?,
                )?;
            }
            Ok(Event::End(ref _e)) => {
                break;
            }
            Ok(Event::Empty(ref e)) => {
                start_tag(e.name(), e.attributes(), Map::new())?;
            }
            Ok(Event::Text(ref e)) => {
                children.insert(
                    TEXT_CHARACTER.to_string(),
                    Value::String(
                        e.unescape_and_decode(&reader)
                            .map_err(|e| Error::XmlQuickXmlError(e))?,
                    ),
                );
            }
            Ok(Event::Comment(ref _e)) => {}
            Ok(Event::CData(ref _e)) => {}
            Ok(Event::Decl(ref e)) => {
                let mut map = Map::new();

                map.insert(
                    "version".to_string(),
                    Value::String(bytes_to_string(
                        &e.version().map_err(|e| Error::XmlQuickXmlError(e))?,
                    )?),
                );

                if let Some(encoding) = e.encoding() {
                    map.insert(
                        "encoding".to_string(),
                        Value::String(bytes_to_string(
                            &encoding.map_err(|e| Error::XmlQuickXmlError(e))?,
                        )?),
                    );
                }

                if let Some(standalone) = e.standalone() {
                    map.insert(
                        "standalone".to_string(),
                        Value::String(bytes_to_string(
                            &standalone.map_err(|e| Error::XmlQuickXmlError(e))?,
                        )?),
                    );
                }

                children.insert(DECL_STRING.to_owned(), Value::Object(map));
            }
            Ok(Event::PI(ref _e)) => {}
            Ok(Event::DocType(ref _e)) => {}
            Ok(Event::Eof) => {
                if root {
                    break;
                }

                return Err(Error::XmlParseUnexpectedEof);
            }
            Err(e) => return Err(Error::XmlQuickXmlError(e)),
        }

        buf.clear();
    }

    Ok(children)
}

/// Convert an XML string to a JSON value.
pub fn xml_to_json(xml: &str) -> Result<Value, Error> {
    let mut buf = vec![];
    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);
    Ok(Value::Object(parse_tag(&mut reader, &mut buf, true)?))
}
