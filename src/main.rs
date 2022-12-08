use kml::KmlWriter;
use std::io::{BufWriter, Write};
use std::str::FromStr;
use std::{collections::HashMap, convert::TryInto};

fn main() {
    let geo_json_str = r#"{"geometry":{"coordinates":[-111.23904861509797,46.35882082926133,0],"type":"Point"},"properties":{"color":[255,255,255,1],"icon":"Location"},"type":"Feature"}"#;
    let gjson = geojson::Feature::from_str(geo_json_str).unwrap();
    let gt: geo_types::Geometry = gjson.try_into().unwrap();

    let mut attrs = HashMap::new();

    attrs.insert("some_attr_one".to_owned(), "I am thing1.".to_owned());
    attrs.insert("some_attr_two".to_owned(), "I am thing2.".to_owned());

    let pmark = kml::types::Placemark {
        name: Some("Test Point".to_owned()),
        description: None,
        geometry: Some(kml::types::Geometry::from(gt)),
        attrs,
        children: Vec::new(),
    };

    let kd = kml::KmlDocument {
        elements: vec![kml::Kml::Placemark(pmark)],
        version: kml::KmlVersion::V23,
        ..Default::default()
    };

    let mut buf = BufWriter::new(Vec::new());
    write_doc(&mut buf, kd).unwrap();
    let bytes = buf.into_inner().unwrap();

    let resp = String::from_utf8(bytes).unwrap();
    println!("{}", resp);
}

pub fn write_doc<T: Write>(buf: &mut T, doc: kml::KmlDocument) -> Result<(), &'static str> {
    let mut writer = KmlWriter::from_writer(buf);
    writer.write(&kml::Kml::KmlDocument(doc)).unwrap();
    Ok(())
}
