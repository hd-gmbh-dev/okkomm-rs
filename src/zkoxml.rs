use base64::Engine;
use std::io::Write;

use bytes::{BufMut, Bytes, BytesMut};
use chrono::{DateTime, Utc};
use chrono_tz::Europe::Berlin;
use chrono_tz::Tz;
use quick_xml::events::BytesCData;
pub use quick_xml::{
    events::{BytesDecl, BytesText, Event},
    Error, Writer,
};

use crate::xml::{WriteXml, XmlWriter};

fn write_field_opt<W>(
    w: &mut Writer<W>,
    name: &'static str,
    value: Option<&str>,
) -> Result<(), Error>
where
    W: std::io::Write,
{
    let el = w.create_element(name);
    if let Some(v) = value {
        el.write_text_content(BytesText::new(v))?;
    }
    Ok(())
}

#[derive(Debug, Clone, serde::Deserialize, PartialEq)]
pub struct AppsInfo {
    #[serde(rename = "APPS_TYP")]
    pub typ: Option<String>,
    #[serde(rename = "APPS_NAME")]
    pub name: Option<String>,
    #[serde(rename = "APPS_VERSION")]
    pub version: Option<String>,
    #[serde(rename = "APPS_AGS")]
    pub ags: Option<String>,
    #[serde(rename = "APPS_DATUM")]
    pub datum: Option<String>,
    #[serde(rename = "APPS_UHRZEIT")]
    pub uhrzeit: Option<String>,
    #[serde(rename = "APPS_REQUEST_ID")]
    pub request_id: Option<String>,
    #[serde(rename = "APPS_SOURCE_ID")]
    pub source_id: Option<String>,
    #[serde(rename = "APPS_KENNUNG")]
    pub kennung: Option<String>,
    #[serde(rename = "APPS_IP_ADRESSE")]
    pub ip_adresse: Option<String>,
    #[serde(rename = "APPS_ZIEL_URL")]
    pub ziel_url: Option<String>,
    #[serde(rename = "APPS_RETURN_QUEUE")]
    pub return_queue: Option<String>,
}

impl WriteXml for AppsInfo {
    fn write_xml(&self, w: &mut XmlWriter) -> Result<(), Error> {
        w.create_element("APPS_INFO").write_inner_content(|w| {
            write_field_opt(w, "APPS_TYP", self.typ.as_deref())?;
            write_field_opt(w, "APPS_NAME", self.name.as_deref())?;
            write_field_opt(w, "APPS_VERSION", self.version.as_deref())?;
            write_field_opt(w, "APPS_AGS", self.ags.as_deref())?;
            write_field_opt(w, "APPS_DATUM", self.datum.as_deref())?;
            write_field_opt(w, "APPS_UHRZEIT", self.uhrzeit.as_deref())?;
            write_field_opt(w, "APPS_REQUEST_ID", self.request_id.as_deref())?;
            write_field_opt(w, "APPS_SOURCE_ID", self.source_id.as_deref())?;
            write_field_opt(w, "APPS_KENNUNG", self.kennung.as_deref())?;
            write_field_opt(w, "APPS_IP_ADRESSE", self.ip_adresse.as_deref())?;
            write_field_opt(w, "APPS_ZIEL_URL", self.ziel_url.as_deref())?;
            write_field_opt(w, "APPS_RETURN_QUEUE", self.return_queue.as_deref())?;
            Ok(())
        })?;
        Ok(())
    }
}

#[derive(Debug, Clone, serde::Deserialize, PartialEq)]
pub struct Fehler {
    #[serde(rename = "FEH_TYP")]
    pub typ: Option<String>,
    #[serde(rename = "FEH_TEXT")]
    pub text: Option<String>,
    #[serde(rename = "FEH_WERT")]
    pub wert: Option<String>,
    #[serde(rename = "FEH_FELD")]
    pub feld: Option<String>,
}

impl WriteXml for Fehler {
    fn write_xml(&self, w: &mut XmlWriter) -> Result<(), Error> {
        w.create_element("FEHLER").write_inner_content(|w| {
            write_field_opt(w, "FEH_TYP", self.typ.as_deref())?;
            write_field_opt(w, "FEH_TEXT", self.text.as_deref())?;
            write_field_opt(w, "FEH_WERT", self.wert.as_deref())?;
            write_field_opt(w, "FEH_FELD", self.feld.as_deref())?;
            Ok(())
        })?;
        Ok(())
    }
}

#[derive(Debug, Clone, serde::Deserialize, PartialEq)]
pub struct Aktion {
    #[serde(rename = "AKT_VERFAHREN")]
    pub verfahren: Option<String>,
    #[serde(rename = "AKT_TYP")]
    pub typ: Option<String>,
    #[serde(rename = "AKT_AUSFUEHRUNG")]
    pub ausfuehrung: Option<String>,
    #[serde(rename = "AKT_ZIEL_AGS")]
    pub ziel_ags: Option<String>,
}

impl WriteXml for Aktion {
    fn write_xml(&self, w: &mut XmlWriter) -> Result<(), Error> {
        w.create_element("AKTION").write_inner_content(|w| {
            write_field_opt(w, "AKT_VERFAHREN", self.verfahren.as_deref())?;
            write_field_opt(w, "AKT_TYP", self.typ.as_deref())?;
            write_field_opt(w, "AKT_AUSFUEHRUNG", self.ausfuehrung.as_deref())?;
            write_field_opt(w, "AKT_ZIEL_AGS", self.ziel_ags.as_deref())?;
            Ok(())
        })?;
        Ok(())
    }
}

#[derive(Debug, Clone, serde::Deserialize, PartialEq)]
pub struct Login {
    #[serde(rename = "AKT_TECHUSER")]
    pub techuser: Option<String>,
    #[serde(rename = "AKT_TECHPWD")]
    pub techpwd: Option<String>,
}

impl WriteXml for Login {
    fn write_xml(&self, w: &mut XmlWriter) -> Result<(), Error> {
        w.create_element("AKT_LOGIN").write_inner_content(|w| {
            write_field_opt(w, "AKT_TECHUSER", self.techuser.as_deref())?;
            write_field_opt(w, "AKT_TECHPWD", self.techpwd.as_deref())?;
            Ok(())
        })?;
        Ok(())
    }
}

#[derive(Debug, Clone, serde::Deserialize, PartialEq)]
pub struct Antwort {
    #[serde(rename = "ANT_TYP")]
    pub typ: Option<String>,
    #[serde(rename = "ANT_APPS")]
    pub apps: Option<String>,
    #[serde(rename = "ANT_STRUKTUR")]
    pub struktur: Option<String>,
    #[serde(rename = "ANT_DATUM")]
    pub datum: Option<String>,
    #[serde(rename = "ANT_UHRZEIT")]
    pub uhrzeit: Option<String>,
    #[serde(rename = "FEHLER")]
    pub fehler: Option<Fehler>,
}

impl WriteXml for Antwort {
    fn write_xml(&self, w: &mut XmlWriter) -> Result<(), Error> {
        w.create_element("ANTWORT").write_inner_content(|w| {
            write_field_opt(w, "ANT_TYP", self.typ.as_deref())?;
            write_field_opt(w, "ANT_APPS", self.apps.as_deref())?;
            write_field_opt(w, "ANT_STRUKTUR", self.struktur.as_deref())?;
            write_field_opt(w, "ANT_DATUM", self.datum.as_deref())?;
            write_field_opt(w, "ANT_UHRZEIT", self.uhrzeit.as_deref())?;
            if let Some(v) = self.fehler.as_ref() {
                v.write_xml(w)?;
            }
            Ok(())
        })?;
        Ok(())
    }
}

#[derive(Debug, Clone, serde::Deserialize, PartialEq)]
pub struct System {
    #[serde(rename = "AKTION")]
    pub aktion: Option<Aktion>,
    #[serde(rename = "AKT_LOGIN")]
    pub akt_login: Option<Login>,
    #[serde(rename = "ANTWORT")]
    pub antwort: Option<Antwort>,
    #[serde(rename = "APPS_INFO")]
    pub apps_info: Option<AppsInfo>,
}

impl WriteXml for System {
    fn write_xml(&self, w: &mut XmlWriter) -> Result<(), Error> {
        w.create_element("SYSTEM").write_inner_content(|w| {
            if let Some(v) = self.aktion.as_ref() {
                v.write_xml(w)?;
            }
            if let Some(v) = self.akt_login.as_ref() {
                v.write_xml(w)?;
            }
            if let Some(v) = self.antwort.as_ref() {
                v.write_xml(w)?;
            }
            if let Some(v) = self.apps_info.as_ref() {
                v.write_xml(w)?;
            }
            Ok(())
        })?;
        Ok(())
    }
}

#[derive(Debug, Clone, serde::Deserialize, PartialEq)]
pub struct XmlSystem {
    #[serde(rename = "SYSTEM")]
    pub system: System,
}

impl WriteXml for XmlSystem {
    fn write_xml(&self, w: &mut XmlWriter) -> Result<(), Error> {
        w.create_element("XML_SYSTEM").write_inner_content(|w| {
            self.system.write_xml(w)?;
            Ok(())
        })?;
        Ok(())
    }
}

#[derive(Debug, Clone, serde::Deserialize, PartialEq)]
pub struct ZkocxmlInfo {
    #[serde(rename = "XML_SYSTEM")]
    pub xml_system: XmlSystem,
}

impl ZkocxmlInfo {
    pub fn write_xml<R, D>(
        &self,
        w: &mut XmlWriter,
        req: Option<&R>,
        data: Option<&D>,
    ) -> Result<(), Error>
    where
        R: WriteXml,
        D: WriteXml,
    {
        w.create_element("ZKOCXML").write_inner_content(|w| {
            self.xml_system.write_xml(w)?;
            {
                const TAG: &str = "XML_PROFIL";
                const CHILD_TAG: &str = "SUCHE";

                w.create_element(TAG).write_inner_content(|w| {
                    let el = w.create_element(CHILD_TAG);
                    if let Some(req) = req.as_ref() {
                        el.write_inner_content(|w| {
                            req.write_xml(w)?;
                            Ok(())
                        })?;
                    } else {
                        el.write_empty()?;
                    }
                    Ok(())
                })?;
            }
            if data.is_some() {
                const TAG: &str = "XML_DATEN";
                const CHILD_TAG: &str = "DATEN";

                w.create_element(TAG).write_inner_content(|w| {
                    let el = w.create_element(CHILD_TAG);
                    if let Some(res) = data {
                        el.write_inner_content(|w| {
                            res.write_xml(w)?;
                            Ok(())
                        })?;
                    } else {
                        el.write_empty()?;
                    }
                    Ok(())
                })?;
            }
            Ok(())
        })?;
        Ok(())
    }

    pub fn error(&self) -> Option<ZkocxmlError> {
        self.xml_system
            .system
            .antwort
            .as_ref()
            .and_then(|antwort| antwort.fehler.as_ref())
            .and_then(|f| match (&f.typ, &f.text, &f.wert, &f.feld) {
                (Some(typ), Some(text), Some(wert), Some(feld)) => {
                    if !typ.is_empty() || !text.is_empty() || !wert.is_empty() || !feld.is_empty() {
                        Some(ZkocxmlError {
                            typ,
                            text,
                            wert,
                            feld,
                        })
                    } else {
                        None
                    }
                }
                _ => None,
            })
    }
}

#[derive(Debug)]
pub struct ZkocxmlError<'a> {
    pub typ: &'a str,
    pub text: &'a str,
    pub wert: &'a str,
    pub feld: &'a str,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Request<R, D = ()>
where
    R: WriteXml,
    D: WriteXml,
{
    pub info: ZkocxmlInfo,
    pub request: Option<R>,
    pub data: Option<D>,
}

impl WriteXml for () {
    fn write_xml(&self, _: &mut XmlWriter) -> Result<(), Error> {
        Ok(())
    }
}

impl<R, D> Request<R, D>
where
    R: WriteXml,
    D: WriteXml,
{
    pub fn new(request: impl Into<Option<R>>, apps_info: Option<AppsInfo>) -> Self {
        let now: DateTime<Tz> = Utc::now().with_timezone(&Berlin);
        let date = now.format("%d.%m.%Y").to_string();
        let time = now.format("%H:%M:%S").to_string();

        Self {
            info: ZkocxmlInfo {
                xml_system: XmlSystem {
                    system: System {
                        aktion: Some(Aktion {
                            verfahren: Some(String::default()),
                            typ: Some(String::default()),
                            ausfuehrung: Some(String::default()),
                            ziel_ags: Some(String::default()),
                        }),
                        akt_login: Some(Login {
                            techuser: Some(String::default()),
                            techpwd: Some(String::default()),
                        }),
                        antwort: Some(Antwort {
                            typ: None,
                            apps: None,
                            struktur: None,
                            datum: None,
                            uhrzeit: None,
                            fehler: None,
                        }),
                        apps_info: Some(apps_info.unwrap_or(AppsInfo {
                            typ: Some("DGS".to_owned()),
                            name: Some("Digital Gov as a Service".to_owned()),
                            version: Some("2023.4.0".to_owned()),
                            ags: Some(String::default()),
                            datum: Some(date),
                            uhrzeit: Some(time),
                            request_id: Some(String::default()),
                            source_id: Some(String::default()),
                            kennung: Some(String::default()),
                            ip_adresse: Some(String::default()),
                            ziel_url: Some(String::default()),
                            return_queue: Some(String::default()),
                        })),
                    },
                },
            },
            request: request.into(),
            data: None,
        }
    }

    pub fn with_verfahren<S: ToString>(mut self, verfahren: S) -> Self {
        self.info
            .xml_system
            .system
            .aktion
            .as_mut()
            .unwrap()
            .verfahren = Some(verfahren.to_string());
        self
    }

    pub fn with_typ<S: ToString>(mut self, typ: S) -> Self {
        self.info.xml_system.system.aktion.as_mut().unwrap().typ = Some(typ.to_string());
        self
    }

    pub fn with_ausfuehrung<S: ToString>(mut self, ausfuehrung: S) -> Self {
        self.info
            .xml_system
            .system
            .aktion
            .as_mut()
            .unwrap()
            .ausfuehrung = Some(ausfuehrung.to_string());
        self
    }

    pub fn with_ziel_ags<S: ToString>(mut self, ziel_ags: S) -> Self {
        let ziel_ags = ziel_ags.to_string();
        self.info
            .xml_system
            .system
            .aktion
            .as_mut()
            .unwrap()
            .ziel_ags = Some(ziel_ags.clone());
        self.info.xml_system.system.apps_info.as_mut().unwrap().ags = Some(ziel_ags);
        self
    }

    pub fn with_xml_daten(mut self, data: impl Into<Option<D>>) -> Self {
        self.data = data.into();
        self
    }

    pub fn to_message(&self) -> Result<bytes::Bytes, Error> {
        let write_buf = BytesMut::new();
        let mut writer = Writer::new(write_buf.writer());
        writer.write_event(Event::Decl(BytesDecl::new(
            "1.0",
            Some("UTF-8"),
            Some("yes"),
        )))?;
        self.write_xml(&mut writer)?;
        Ok(writer.into_inner().into_inner().freeze())
    }
}

impl<R, D> WriteXml for Request<R, D>
where
    R: WriteXml,
    D: WriteXml,
{
    fn write_xml(&self, w: &mut XmlWriter) -> Result<(), Error> {
        self.info
            .write_xml(w, self.request.as_ref(), self.data.as_ref())?;
        Ok(())
    }
}

pub struct RawRequest(pub String);

impl WriteXml for RawRequest {
    fn write_xml(&self, w: &mut XmlWriter) -> Result<(), Error> {
        w.inner().write_all(self.0.as_bytes())?;
        Ok(())
    }
}

pub struct BytesRequest(pub bytes::Bytes);

impl WriteXml for BytesRequest {
    fn write_xml(&self, w: &mut XmlWriter) -> Result<(), Error> {
        w.inner().write_all(&self.0)?;
        Ok(())
    }
}

pub struct ContentContainerMessage {
    pub content_type: String,
    pub ref_id: String,
    pub content: String,
}
pub struct ContentContainerAttachment {
    pub content_type: String,
    pub ref_id: String,
    pub content: Bytes,
}

pub struct ContentContainer<'m> {
    pub messages: &'m Vec<ContentContainerMessage>,
    pub attachments: &'m Vec<ContentContainerAttachment>,
}

impl WriteXml for ContentContainer<'_> {
    fn write_xml(&self, w: &mut XmlWriter) -> Result<(), Error> {
        w.create_element("OK_KOMM_CONTENTCONTAINER")
            .write_inner_content(|w| {
                w.create_element("MESSAGES")
                    .with_attribute(("type", "include"))
                    .write_inner_content(|w| {
                        for message in self.messages {
                            w.create_element("MESSAGE")
                                .with_attribute(("contentType", message.content_type.as_str()))
                                .with_attribute(("refId", message.ref_id.as_str()))
                                .write_inner_content(|w| {
                                    w.create_element("OK_KOMM_RAW_BASE64").write_cdata_content(
                                        BytesCData::new(
                                            base64::engine::general_purpose::STANDARD
                                                .encode(message.content.as_str())
                                                .as_str(),
                                        ),
                                    )?;
                                    Ok(())
                                })?;
                        }
                        Ok(())
                    })?;
                if !&self.attachments.is_empty() {
                    w.create_element("ATTACHMENTS")
                        .with_attribute(("type", "include"))
                        .write_inner_content(|w| {
                            for attachment in self.attachments {
                                w.create_element("ATTACHMENT")
                                    .with_attribute((
                                        "contentType",
                                        attachment.content_type.as_str(),
                                    ))
                                    .with_attribute(("refId", attachment.ref_id.as_str()))
                                    .write_inner_content(|w| {
                                        w.create_element("OK_KOMM_RAW_BASE64")
                                            .write_cdata_content(BytesCData::new(
                                                base64::engine::general_purpose::STANDARD
                                                    .encode(&attachment.content)
                                                    .as_str(),
                                            ))?;
                                        Ok(())
                                    })?;
                            }
                            Ok(())
                        })?;
                }
                Ok(())
            })?;
        Ok(())
    }
}

pub struct RawBase64 {
    pub body: String,
}
impl WriteXml for RawBase64 {
    fn write_xml(&self, w: &mut XmlWriter) -> Result<(), Error> {
        w.create_element("OK_KOMM_RAW_BASE64")
            .write_cdata_content(BytesCData::new(
                base64::engine::general_purpose::STANDARD
                    .encode(self.body.clone())
                    .as_str(),
            ))?;
        Ok(())
    }
}
