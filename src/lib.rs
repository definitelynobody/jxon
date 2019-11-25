mod constants;
mod error;
mod to_json;
mod to_xml;

pub use error::Error;
use std::str::from_utf8;
pub use to_json::xml_to_json;
pub use to_xml::json_to_xml;

fn bytes_to_string(bytes: &[u8]) -> Result<String, Error> {
    from_utf8(bytes)
        .map(|s| s.to_owned())
        .map_err(|e| Error::ParseStringError(e))
}
