use std::str::FromStr;

use anyhow::Error;
use bytes::{BufMut, BytesMut};
use quick_xml::Writer;
use reqwest::{header, Response};
use serde::Deserialize;

use soap::SoapRequest;
use zkoxml::ContentContainerAttachment;

use crate::okkomm::{OkKommCallApplicationByte, OkKommCallApplicationByteResponse};
use crate::soap::SoapResponse;
use crate::xml::WriteXml;
use crate::zkoxml::{AppsInfo, ContentContainer, ContentContainerMessage, RawBase64, Request};

pub mod okkomm;
pub mod soap;
pub mod xml;
pub mod zkoxml;

#[derive(Clone)]
pub struct Client {
    client: reqwest::Client,
    pub url: String,
}

pub struct OkKommAktion {
    pub verfahren: String,
    pub typ: String,
    pub ausfuehrung: String,
    pub ziel_ags: String,
}

impl OkKommAktion {
    pub fn new(verfahren: String, typ: String, ausfuehrung: String, ziel_ags: String) -> Self {
        Self {
            verfahren,
            typ,
            ausfuehrung,
            ziel_ags,
        }
    }
}

impl Client {
    pub fn new(url: String, tls_root_certificates: Option<Vec<Vec<u8>>>) -> anyhow::Result<Self> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Content-Type",
            header::HeaderValue::from_static("text/xml; charset=utf-8"),
        );

        let mut client_builder = reqwest::ClientBuilder::new()
            .no_proxy()
            .default_headers(headers);

        if let Some(certs) = tls_root_certificates {
            for cert in certs {
                match reqwest::Certificate::from_pem(&cert) {
                    Ok(cert) => {
                        client_builder = client_builder.add_root_certificate(cert);
                    }
                    Err(err) => log::error!("Error while parsing certificate: {err:#?}"),
                }
            }
        }

        Ok(Self {
            client: client_builder.build()?,
            url,
        })
    }

    pub fn soap_body<R, D>(
        &self,
        info: OkKommAktion,
        request: impl Into<Option<R>>,
        data: impl Into<Option<D>>,
        apps_info: Option<AppsInfo>,
    ) -> Result<SoapRequest<OkKommCallApplicationByte<bytes::Bytes>>, quick_xml::Error>
    where
        R: WriteXml,
        D: WriteXml,
    {
        let zkoxml_body = Request::new(request, apps_info)
            .with_verfahren(info.verfahren)
            .with_typ(info.typ)
            .with_ausfuehrung(info.ausfuehrung)
            .with_ziel_ags(info.ziel_ags)
            .with_xml_daten(data)
            .to_message()?;
        Ok(SoapRequest::new(OkKommCallApplicationByte::new(
            zkoxml_body,
        )))
    }

    pub fn request_soap<T>(
        &self,
        soap_request: SoapRequest<T>,
    ) -> Result<reqwest::RequestBuilder, quick_xml::Error>
    where
        T: WriteXml,
    {
        let body = soap_request.to_message()?;
        Ok(self.client.post(&self.url).body(body))
    }

    pub fn request<T>(
        &self,
        info: OkKommAktion,
        body: T,
        apps_info: Option<AppsInfo>,
    ) -> Result<reqwest::RequestBuilder, quick_xml::Error>
    where
        T: WriteXml,
    {
        let soap_request = self.soap_body(info, body, (), apps_info)?;
        self.request_soap(soap_request)
    }

    fn request_xml_base64<T>(
        &self,
        info: OkKommAktion,
        body: T,
        apps_info: Option<AppsInfo>,
    ) -> Result<reqwest::RequestBuilder, quick_xml::Error>
    where
        T: WriteXml,
    {
        let write_buf = BytesMut::new();
        let mut writer = Writer::new(write_buf.writer());
        body.write_xml(&mut writer)?;

        self.request(
            info,
            RawBase64 {
                body: String::from_utf8_lossy(&writer.into_inner().into_inner().freeze())
                    .to_string(),
            },
            apps_info,
        )
    }

    fn request_xml_in_content_container<T>(
        &self,
        info: OkKommAktion,
        body: T,
        attachments: Vec<ContentContainerAttachment>,
        ref_id: String,
        apps_info: Option<AppsInfo>,
    ) -> Result<reqwest::RequestBuilder, quick_xml::Error>
    where
        T: WriteXml,
    {
        let write_buf = BytesMut::new();
        let mut writer = Writer::new(write_buf.writer());
        body.write_xml(&mut writer)?;

        self.request(
            info,
            ContentContainer {
                messages: &vec![ContentContainerMessage {
                    content_type: "text/xml".to_string(),
                    ref_id,
                    content: String::from_utf8_lossy(&writer.into_inner().into_inner().freeze())
                        .to_string(),
                }],
                attachments: &attachments,
            },
            apps_info,
        )
    }

    async fn handle_request_result<R>(result: Result<Response, reqwest::Error>) -> anyhow::Result<R>
    where
        R: for<'a> Deserialize<'a>,
    {
        match result {
            Ok(response) => match response.text().await {
                Ok(response_text) => {
                    match SoapResponse::<OkKommCallApplicationByteResponse>::from_str(
                        response_text.as_str(),
                    ) {
                        Ok(soap_response) => match soap_response.into_inner() {
                            Some(soap_xml) => match soap_xml.decode() {
                                Ok((info, xml)) => match xml {
                                    Some(xml) => match quick_xml::de::from_str::<R>(xml.as_str()) {
                                        Ok(result) => Ok(result),
                                        Err(err) => Err(Error::from(err)),
                                    },
                                    None => Err(Error::msg(format!(
                                        "OK.KOMM result cannot be parsed: {xml:#?} / {info:#?}"
                                    ))),
                                },
                                Err(err) => Err(Error::msg(format!(
                                    "Error while parsing OK.KOMM response: {err:#?}"
                                ))),
                            },
                            None => Err(Error::msg(format!(
                                "SOAP response from OK.KOMM cannot be parsed: {response_text:#?}"
                            ))),
                        },
                        Err(err) => Err(Error::msg(format!(
                            "Error while parsing SOAP response from OK.KOMM: {err:#?}"
                        ))),
                    }
                }
                Err(err) => Err(Error::msg(format!(
                    "Error while receiving SOAP response from OK.KOMM: {err:#?}"
                ))),
            },
            Err(err) => Err(Error::msg(format!(
                "Error while sending SOAP request to OK.KOMM: {err:#?}"
            ))),
        }
    }

    pub async fn send_request_xml<T, R>(
        &self,
        info: OkKommAktion,
        body: T,
        apps_info: Option<AppsInfo>,
    ) -> anyhow::Result<R>
    where
        T: WriteXml,
        R: for<'a> Deserialize<'a>,
    {
        let result = self.request(info, body, apps_info)?.send().await;
        Self::handle_request_result(result).await
    }

    pub async fn send_request_xml_base64<T, R>(
        &self,
        info: OkKommAktion,
        body: T,
        apps_info: Option<AppsInfo>,
    ) -> anyhow::Result<R>
    where
        T: WriteXml,
        R: for<'a> Deserialize<'a>,
    {
        let result = self.request_xml_base64(info, body, apps_info)?.send().await;
        Self::handle_request_result(result).await
    }

    pub async fn send_request_xml_in_content_container<T, R>(
        &self,
        info: OkKommAktion,
        body: T,
        attachments: Vec<ContentContainerAttachment>,
        ref_id: String,
        apps_info: Option<AppsInfo>,
    ) -> anyhow::Result<R>
    where
        T: WriteXml,
        R: for<'a> Deserialize<'a>,
    {
        let result = self
            .request_xml_in_content_container(info, body, attachments, ref_id, apps_info)?
            .send()
            .await;
        Self::handle_request_result(result).await
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use soap::SoapResponse;
    use zkoxml::Request;

    use crate::okkomm::{OkKommCallApplicationByte, OkKommCallApplicationByteResponse};
    use crate::soap::SoapRequest;
    use crate::zkoxml::RawRequest;
    use crate::{soap, zkoxml, Client, OkKommAktion};

    #[test]
    fn test_to_message_soap_envelope() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?><SOAP-ENV:Envelope xmlns:SOAP-ENV="http://schemas.xmlsoap.org/soap/envelope/"><SOAP-ENV:Header/><SOAP-ENV:Body xmlns:xsd="http://www.w3.org/2001/XMLSchema"><okk:callApplicationByte xmlns:okk="urn:akdb:ok.komm:komm-service"><okk:xmlParameter xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:type="xsd:base64Binary">dGVzdA==</okk:xmlParameter></okk:callApplicationByte></SOAP-ENV:Body></SOAP-ENV:Envelope>"#;
        let req = SoapRequest::new(OkKommCallApplicationByte::new("test"));
        let msg = req.to_message()?;
        let msg_str = String::from_utf8_lossy(&msg);
        eprintln!("{msg_str}");
        assert_eq!(msg_str, xml);
        Ok(())
    }

    // #[tokio::test]
    // async fn client_test() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    //     let client = Client::new(
    //         "http://localhost:8380/okkommetest/services/KomService".to_owned(),
    //         None,
    //     )?;
    //     let raw_request = RawRequest("<MANDANTENANFRAGE></MANDANTENANFRAGE>".to_owned());
    //     let res = client
    //         .request(
    //             OkKommAktion::new(
    //                 "EWO".to_owned(),
    //                 "WEBWAHLSCHEIN".to_owned(),
    //                 "ABRUFEN".to_owned(),
    //                 "09000011".to_owned(),
    //             ),
    //             raw_request,
    //             None,
    //         )?
    //         .send()
    //         .await?;
    //     let body = res.text().await?;
    //     let res = SoapResponse::<OkKommCallApplicationByteResponse>::from_str(&body)?;
    //     if let Some(res) = res.into_inner() {
    //         let (info, data) = res.decode()?;
    //         eprintln!("{info:#?}");
    //         eprintln!("{data:#?}");
    //     }
    //     Ok(())
    // }

    #[test]
    fn test_to_message_zkocxml() -> Result<(), Box<dyn std::error::Error>> {
        let xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><ZKOCXML><XML_SYSTEM><SYSTEM><AKTION><AKT_VERFAHREN></AKT_VERFAHREN><AKT_TYP></AKT_TYP><AKT_AUSFUEHRUNG></AKT_AUSFUEHRUNG><AKT_ZIEL_AGS></AKT_ZIEL_AGS></AKTION><AKT_LOGIN><AKT_TECHUSER></AKT_TECHUSER><AKT_TECHPWD></AKT_TECHPWD></AKT_LOGIN><ANTWORT></ANTWORT><APPS_INFO><APPS_TYP>DGS</APPS_TYP><APPS_NAME>Digital Gov as a Service</APPS_NAME><APPS_VERSION></APPS_VERSION><APPS_AGS></APPS_AGS><APPS_DATUM>24.01.2023</APPS_DATUM><APPS_UHRZEIT>12:17:37</APPS_UHRZEIT><APPS_REQUEST_ID></APPS_REQUEST_ID><APPS_SOURCE_ID></APPS_SOURCE_ID><APPS_KENNUNG></APPS_KENNUNG><APPS_IP_ADRESSE></APPS_IP_ADRESSE><APPS_ZIEL_URL></APPS_ZIEL_URL><APPS_RETURN_QUEUE></APPS_RETURN_QUEUE></APPS_INFO></SYSTEM></XML_SYSTEM><XML_PROFIL><SUCHE>test</SUCHE></XML_PROFIL></ZKOCXML>"#;
        let req = Request::<_, ()>::new(RawRequest("test".to_owned()), None);
        let msg = req.to_message()?;
        let msg_str = String::from_utf8_lossy(&msg);
        eprintln!("{msg_str}");
        assert_eq!(&msg_str[0..400], &xml[0..400]);
        Ok(())
    }
}
