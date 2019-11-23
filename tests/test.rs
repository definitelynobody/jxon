use jxon::{json_to_xml, xml_to_json};
use serde_json::{json, Value};

fn check(xml: &str, expected_json: Value) {
    let json_value = xml_to_json(xml).expect("xml to json").to_string();
    assert_eq!(
        json_value,
        expected_json.to_string(),
        "converting xml to json"
    );
    assert_eq!(
        json_to_xml(&json_value, None).expect("json to xml"),
        xml,
        "converting json to xml"
    );
}

#[test]
fn empty_root_tag() {
    check(
        "<root/>",
        json!({
            "root": [{
            }]
        }),
    );
}

#[test]
fn text() {
    check(
        "<root>test</root>",
        json!({
            "root": [{
                "_": "test"
            }]
        }),
    );

    check(
        "<root> test </root>",
        json!({
            "root": [{
                "_": " test "
            }]
        }),
    );

    check(
        r#"<root>"</root>"#,
        json!({
            "root": [{
                "_": "\""
            }]
        }),
    );
}

#[test]
fn attributes() {
    check(
        r#"<root Something="value"/>"#,
        json!({
            "root": [
                {
                    "$Something": "value"
                }
            ]
        }),
    );

    check(
        r#"<root Something="value" SomethingElse="value2"/>"#,
        json!({
            "root": [
                {
                    "$Something": "value",
                    "$SomethingElse": "value2"
                }
            ]
        }),
    );
}

#[test]
fn children() {
    check(
        r#"<one><two><three/></two></one>"#,
        json!({
            "one": [
                {
                    "two": [
                        {
                            "three": [
                                {}
                            ]
                        }
                    ]
                }
            ]
        }),
    );

    check(
        r#"<root><sib1/><sib2/></root>"#,
        json!({
            "root": [
                {
                    "sib1": [
                        {
                        }
                    ],
                    "sib2": [
                        {
                        }
                    ]
                }
            ]
        }),
    );
}

#[test]
fn decl() {
    check(
        r#"<?xml version="1.0"?><root/>"#,
        json!({
            "#": {
                "version": "1.0"
            },
            "root": [
                {}
            ]
        }),
    );

    check(
        r#"<?xml version="1.0" encoding="UTF-8"?><root/>"#,
        json!({
            "#": {
                "version": "1.0",
                "encoding": "UTF-8"
            },
            "root": [
                {}
            ]
        }),
    );

    check(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?><root/>"#,
        json!({
            "#": {
                "version": "1.0",
                "encoding": "UTF-8",
                "standalone": "no"
            },
            "root": [
                {}
            ]
        }),
    );
}
