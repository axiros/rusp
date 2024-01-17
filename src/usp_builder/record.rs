use crate::usp::Msg;
use crate::usp_record::mod_Record::OneOfrecord_type::no_session_context;
use crate::usp_record::NoSessionContextRecord;
use crate::usp_record::Record;
use crate::usp_types::PayloadSecurity;

use anyhow::{Context, Result};

#[derive(Clone)]
pub struct RecordBuilder {
    version: String,
    to_id: Option<String>,
    from_id: Option<String>,
    sender_cert: Vec<u8>,
    mac_signature: Vec<u8>,
    payload_security: PayloadSecurity,
    payload: Option<Vec<u8>>,
}

impl RecordBuilder {
    pub fn new() -> Self {
        Self {
            version: "1.3".into(),
            to_id: None,
            from_id: None,
            sender_cert: vec![],
            mac_signature: vec![],
            payload_security: PayloadSecurity::PLAINTEXT,
            payload: None,
        }
    }

    pub fn with_version(mut self, version: String) -> Self {
        self.version = version;
        self
    }

    pub fn with_to_id(mut self, id: String) -> Self {
        self.to_id = Some(id);
        self
    }

    pub fn with_from_id(mut self, id: String) -> Self {
        self.from_id = Some(id);
        self
    }

    pub fn with_payload(self, msg: Msg) -> Self {
        let mut buf = Vec::new();
        let mut writer = quick_protobuf::Writer::new(&mut buf);
        quick_protobuf::MessageWrite::write_message(&msg, &mut writer).unwrap();
        self.with_payload_bytes(buf)
    }

    pub fn with_payload_bytes(mut self, buf: Vec<u8>) -> Self {
        self.payload = Some(buf);
        self
    }

    pub fn build<'a>(self) -> Result<Record<'a>> {
        let to_id = self
            .to_id
            .with_context(|| "Cannot produce USP Record without to_id")?;
        let from_id = self
            .from_id
            .with_context(|| "Cannot produce USP Record without from_id")?;
        let payload = self
            .payload
            .with_context(|| "Cannot produce USP Record without payload")?;

        Ok(Record {
            version: self.version.into(),
            to_id: to_id.into(),
            from_id: from_id.into(),
            sender_cert: self.sender_cert.into(),
            mac_signature: self.mac_signature.into(),
            payload_security: self.payload_security,
            record_type: no_session_context(NoSessionContextRecord {
                payload: payload.into(),
            }),
        })
    }
}
