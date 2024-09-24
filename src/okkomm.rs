use crate::{xml::WriteXml, zkoxml};
use base64::{engine::general_purpose::STANDARD, Engine};
use quick_xml::events::{BytesText, Event};
use std::io::Cursor;
use zkoxml::ZkocxmlInfo;

#[derive(thiserror::Error, Clone, Debug)]
pub enum Error {
    #[error("xml error")]
    XmlError(#[from] quick_xml::Error),

    #[error("xml deserialize error")]
    XmlDeserializeError(#[from] quick_xml::DeError),

    #[error("base64 decode error")]
    Base64DecodeError(#[from] base64::DecodeError),

    #[error("invalid utf-8")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
}

#[derive(Debug, Clone, serde::Deserialize, PartialEq)]
pub struct Base64Body {
    #[serde(rename = "xmlParameter")]
    inner: Option<String>,
    #[serde(rename = "callApplicationByteReturn")]
    byte_return: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize, PartialEq)]
pub struct OkKommCallApplicationByteResponse {
    #[serde(rename = "callApplicationByteResponse")]
    bytes: Option<Base64Body>,
}

impl OkKommCallApplicationByteResponse {
    pub fn decode(&self) -> Result<(Option<ZkocxmlInfo>, Option<String>), Error> {
        if let Some(v) = self
            .bytes
            .as_ref()
            .and_then(|v| v.inner.as_deref().or(v.byte_return.as_deref()))
        {
            let xml = String::from_utf8(STANDARD.decode(v)?)?;
            let info = quick_xml::de::from_str::<ZkocxmlInfo>(&xml)?;
            return Ok((Some(info), read_message(&xml)?));
        }
        Ok((None, None))
    }
}

fn read_message(xml: &str) -> Result<Option<String>, quick_xml::Error> {
    let mut res = None;
    let mut buf = Vec::new();
    let mut reader = quick_xml::Reader::from_reader(Cursor::new(xml));
    reader.trim_text(true);
    loop {
        let ev = reader.read_event_into(&mut buf);
        match ev {
            Ok(Event::Eof) => break, // exits the loop when reaching end of file
            // Ok(event) => writer.write_event(event),
            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"DATEN" => {
                    let mut writer = quick_xml::writer::Writer::new(Vec::new());
                    loop {
                        let ev = reader.read_event_into(&mut buf);
                        match ev {
                            Ok(Event::End(e)) => match e.name().as_ref() {
                                b"DATEN" => break,
                                _ => {
                                    writer.write_event(Event::End(e))?;
                                }
                            },
                            Ok(Event::Eof) => {
                                writer.write_event(Event::Eof)?;
                                break;
                            }
                            Ok(e) => {
                                writer.write_event(e)?;
                            }
                            Err(e) => {
                                panic!("Error at position {}: {:?}", reader.buffer_position(), e)
                            }
                        }
                    }
                    let s = std::str::from_utf8(&writer.into_inner())
                        .expect("Failed to convert a slice of bytes to a string slice")
                        .to_string();
                    res = Some(s);
                }
                _ => {
                    log::debug!("{}", String::from_utf8_lossy(e.name().as_ref()));
                }
            },
            Ok(_) => {}
            Err(e) => Err(e)?,
        };
        // If we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
        buf.clear();
    }
    Ok(res)
}

pub struct OkKommCallApplicationByte<B>
where
    B: AsRef<[u8]>,
{
    body: B,
}

impl<B> OkKommCallApplicationByte<B>
where
    B: AsRef<[u8]>,
{
    pub fn new(body: B) -> Self {
        Self { body }
    }
}

impl<B> WriteXml for OkKommCallApplicationByte<B>
where
    B: AsRef<[u8]>,
{
    fn write_xml(
        &self,
        w: &mut quick_xml::Writer<bytes::buf::Writer<bytes::BytesMut>>,
    ) -> Result<(), quick_xml::Error> {
        w.create_element("okk:callApplicationByte")
            .with_attribute(("xmlns:okk", "urn:akdb:ok.komm:komm-service"))
            .write_inner_content(|w| {
                w.create_element("okk:xmlParameter")
                    .with_attributes([
                        ("xmlns:xsi", "http://www.w3.org/2001/XMLSchema-instance"),
                        ("xsi:type", "xsd:base64Binary"),
                    ])
                    .write_text_content(BytesText::new(&STANDARD.encode(self.body.as_ref())))?;
                Ok(())
            })?;
        Ok(())
    }
}
