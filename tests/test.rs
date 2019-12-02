use jxon::*;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

fn check(xml: &str, expected_json: Value) {
    check_different_xml(xml, xml, expected_json);
}

fn check_different_xml(xml: &str, xml_transformed: &str, expected_json: Value) {
    let json_value = xml_to_json(xml).expect("xml to json").to_string();
    assert_eq!(
        json_value,
        expected_json.to_string(),
        "converting xml to json"
    );
    assert_eq!(
        json_to_xml(&json_value, None).expect("json to xml"),
        xml_transformed,
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
    let xml = "<root>test</root>";
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Test {
        root: Vec<TextContent<String>>,
    }

    assert_eq!(
        deserialize::<Test>(xml).unwrap(),
        Test {
            root: vec![TextContent {
                content: "test".to_owned()
            }]
        }
    );

    check(
        xml,
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
        r#"<root>&amp;</root>"#,
        json!({
            "root": [{
                "_": "&"
            }]
        }),
    );

    check_different_xml(
        r#"
<root>
    <something>&amp;</something>
</root>
"#,
        r#"<root><something>&amp;</something></root>"#,
        json!({
            "root": [{
                "something": [
                    {
                        "_": "&"
                    }
                ]
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

    check(
        r#"<root Something="&amp;"/>"#,
        json!({
            "root": [
                {
                    "$Something": "&"
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
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Test {
        #[serde(rename = "#")]
        decl: Declaration,
        root: Vec<Root>,
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Root {}

    let xml = r#"<?xml version="1.0"?><root/>"#;

    assert_eq!(
        deserialize::<Test>(xml).unwrap(),
        Test {
            decl: Declaration {
                version: "1.0".to_owned(),
                encoding: None,
                standalone: None,
            },
            root: vec![Root {}]
        }
    );

    check(
        xml,
        json!({
            "#": {
                "version": "1.0"
            },
            "root": [
                {}
            ]
        }),
    );

    let xml = r#"<?xml version="1.0" encoding="UTF-8"?><root/>"#;

    assert_eq!(
        deserialize::<Test>(xml).unwrap(),
        Test {
            decl: Declaration {
                version: "1.0".to_owned(),
                encoding: Some("UTF-8".to_owned()),
                standalone: None,
            },
            root: vec![Root {}]
        }
    );

    check(
        xml,
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

    let xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?><root/>"#;

    assert_eq!(
        deserialize::<Test>(xml).unwrap(),
        Test {
            decl: Declaration {
                version: "1.0".to_owned(),
                encoding: Some("UTF-8".to_owned()),
                standalone: Some("no".to_owned()),
            },
            root: vec![Root {}]
        }
    );

    check(
        xml,
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

#[test]
fn serde() {
    let xml = r#"<root attribute="value"><child>text</child></root>"#;

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Test {
        root: Vec<Root>,
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Root {
        #[serde(rename = "$attribute")]
        attribute: String,
        child: Vec<Child>,
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Child {
        #[serde(rename = "_")]
        content: String,
    }

    let test = Test {
        root: vec![Root {
            attribute: "value".to_owned(),
            child: vec![Child {
                content: "text".to_owned(),
            }],
        }],
    };

    assert_eq!(deserialize::<Test>(xml).unwrap(), test);
    assert_eq!(serialize(test, None).unwrap(), xml);
}
