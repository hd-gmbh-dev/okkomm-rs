use bytes::{buf::Writer, BytesMut};

pub type XmlWriter = quick_xml::Writer<Writer<BytesMut>>;

pub trait WriteXml {
    fn write_xml(&self, writer: &mut XmlWriter) -> Result<(), quick_xml::Error>;
}
