use super::*;

// fn gedcomx_record() -> GedcomX {
//     GedcomX::new().person(Person::new())
// }

// #[test]
// fn serialize_gedcomx_record_as_xml() -> Result<(), Box<dyn Error>> {
//     let gedcomx = gedcomx_record();

//     let mut buffer = Vec::new();
//     let mut ser = quick_xml::se::Serializer::new(&mut buffer);
//     // let result = quick_xml::se::to_string(&gedcomx)?;
//     gedcomx.serialize(&mut ser)?;
//     assert_eq!(String::from_utf8(buffer)?, "<gedcomx/>");
//     Ok(())
// }

#[test]
fn serialize_empty_as_xml() -> Result<(), Box<dyn std::error::Error>> {
    let gedcomx = Batch::new();

    println!("gedcomx={:?}", gedcomx);
    Ok(())
}

// #[test]
// fn serialize_empty_as_json() -> Result<(), Box<dyn std::error::Error>> {
//     let gedcomx = GedcomX::new();

//     let result = serde_json::to_string(&gedcomx)?;
//     let expected = r#"{"attribution":null,"persons":[],"relationships":[],"sourceDescriptions":[],"agents":[],"documents":[]}"#;
//     assert_eq!(result, expected);
//     Ok(())
// }
