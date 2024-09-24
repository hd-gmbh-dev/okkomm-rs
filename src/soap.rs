use crate::xml::WriteXml;
use bytes::{BufMut, BytesMut};
use quick_xml::{
    events::{BytesDecl, Event},
    Writer,
};
use serde::de::DeserializeOwned;
use std::str::FromStr;

#[derive(Debug, serde::Deserialize, PartialEq)]
pub struct ResponseEnvelope<T> {
    #[serde(rename = "Header")]
    _header: Option<String>,
    #[serde(rename = "Body")]
    body: Option<T>,
}

#[derive(Debug)]
pub struct SoapResponse<T>
where
    T: DeserializeOwned,
{
    envelope: ResponseEnvelope<T>,
}

impl<T> SoapResponse<T>
where
    T: DeserializeOwned,
{
    pub fn into_inner(self) -> Option<T> {
        self.envelope.body
    }
}

impl<T> FromStr for SoapResponse<T>
where
    T: DeserializeOwned,
{
    type Err = quick_xml::de::DeError;

    fn from_str(payload: &str) -> Result<Self, quick_xml::de::DeError> {
        let envelope: ResponseEnvelope<T> = quick_xml::de::from_str(payload)?;
        Ok(Self { envelope })
    }
}

pub struct RequestEnvelope<B>
where
    B: WriteXml,
{
    body: B,
}

pub struct SoapRequest<B>
where
    B: WriteXml,
{
    envelope: RequestEnvelope<B>,
}

impl<B> SoapRequest<B>
where
    B: WriteXml,
{
    pub fn new(body: B) -> Self {
        Self {
            envelope: RequestEnvelope { body },
        }
    }

    pub fn to_message(&self) -> Result<bytes::Bytes, quick_xml::Error> {
        let write_buf = BytesMut::new();
        let mut writer = Writer::new(write_buf.writer());
        writer.write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)))?;
        writer
            .create_element("SOAP-ENV:Envelope")
            .with_attribute((
                "xmlns:SOAP-ENV",
                "http://schemas.xmlsoap.org/soap/envelope/",
            ))
            .write_inner_content(|w| {
                w.create_element("SOAP-ENV:Header").write_empty()?;
                w.create_element("SOAP-ENV:Body")
                    .with_attribute(("xmlns:xsd", "http://www.w3.org/2001/XMLSchema"))
                    .write_inner_content(|w| {
                        self.envelope.body.write_xml(w)?;
                        Ok(())
                    })?;
                Ok(())
            })?;
        Ok(writer.into_inner().into_inner().freeze())
    }
}
