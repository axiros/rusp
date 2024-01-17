use crate::usp::Msg;
use crate::usp_record::mod_MQTTConnectRecord::MQTTVersion;
use crate::usp_record::mod_Record::OneOfrecord_type;
use crate::usp_record::NoSessionContextRecord;
use crate::usp_record::Record;
use crate::usp_record::{
    DisconnectRecord, MQTTConnectRecord, STOMPConnectRecord, UDSConnectRecord,
    WebSocketConnectRecord,
};
use crate::usp_types::PayloadSecurity;

use anyhow::anyhow;
use anyhow::{Context, Result};

#[derive(Clone)]
enum RecordType {
    None,
    NoSessionContext,
    _SessionContext,
    WebSocketConnect,
    MQTTConnect {
        version: MQTTVersion,
        subscribed_topic: String,
    },
    STOMPConnect {
        version: crate::usp_record::mod_STOMPConnectRecord::STOMPVersion,
        subscribed_destination: String,
    },
    Disconnect {
        reason: String,
        reason_code: u32,
    },
    UDSConnect,
}

#[derive(Clone)]
pub struct RecordBuilder {
    version: String,
    to_id: Option<String>,
    from_id: Option<String>,
    sender_cert: Vec<u8>,
    mac_signature: Vec<u8>,
    payload_security: PayloadSecurity,
    payload: Option<Vec<u8>>,
    typ: RecordType,
}

impl RecordBuilder {
    pub const fn new() -> Self {
        Self {
            version: String::new(),
            to_id: None,
            from_id: None,
            sender_cert: vec![],
            mac_signature: vec![],
            payload_security: PayloadSecurity::PLAINTEXT,
            payload: None,
            typ: RecordType::None,
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

    pub fn with_no_session_context_payload(mut self, msg: Msg) -> Self {
        let mut buf = Vec::new();
        let mut writer = quick_protobuf::Writer::new(&mut buf);
        quick_protobuf::MessageWrite::write_message(&msg, &mut writer).unwrap();
        self.typ = RecordType::NoSessionContext;
        self.with_no_session_context_payload_bytes(buf)
    }

    pub fn with_no_session_context_payload_bytes(mut self, buf: Vec<u8>) -> Self {
        self.payload = Some(buf);
        self
    }

    pub fn as_websocket_connect_record(mut self) -> Self {
        self.typ = RecordType::WebSocketConnect;
        self
    }

    pub fn as_mqtt_connect_record(
        mut self,
        version: MQTTVersion,
        subscribed_topic: String,
    ) -> Self {
        self.typ = RecordType::MQTTConnect {
            version,
            subscribed_topic,
        };
        self
    }

    pub fn as_stomp_connect_record(
        mut self,
        version: crate::usp_record::mod_STOMPConnectRecord::STOMPVersion,
        subscribed_destination: String,
    ) -> Self {
        self.typ = RecordType::STOMPConnect {
            version,
            subscribed_destination,
        };
        self
    }

    pub fn as_disconnect_record(mut self, reason: String, reason_code: u32) -> Self {
        self.typ = RecordType::Disconnect {
            reason,
            reason_code,
        };
        self
    }

    pub fn as_uds_connect_record(mut self) -> Self {
        self.typ = RecordType::UDSConnect;
        self
    }

    pub fn build(self) -> Result<Record<'static>> {
        let to_id = self
            .to_id
            .with_context(|| "Cannot produce USP Record without to_id")?;
        let from_id = self
            .from_id
            .with_context(|| "Cannot produce USP Record without from_id")?;

        let mut record = Record {
            version: if !self.version.is_empty() {
                self.version.into()
            } else {
                "1.3.".into()
            },
            to_id: to_id.into(),
            from_id: from_id.into(),
            sender_cert: self.sender_cert.into(),
            mac_signature: self.mac_signature.into(),
            payload_security: self.payload_security,
            record_type: OneOfrecord_type::None,
        };

        match self.typ {
            RecordType::None => Err(anyhow!("Cannot produce a USP Record without type"))?,
            RecordType::NoSessionContext => {
                let payload = self
                    .payload
                    .with_context(|| "Cannot produce USP Record without payload")?
                    .into();

                record.record_type =
                    OneOfrecord_type::no_session_context(NoSessionContextRecord { payload });
            }
            RecordType::_SessionContext => todo!(),
            RecordType::WebSocketConnect => {
                record.record_type = OneOfrecord_type::websocket_connect(WebSocketConnectRecord {});
            }
            RecordType::MQTTConnect {
                version,
                subscribed_topic,
            } => {
                record.record_type = OneOfrecord_type::mqtt_connect(MQTTConnectRecord {
                    version,
                    subscribed_topic: subscribed_topic.into(),
                });
            }
            RecordType::STOMPConnect {
                version,
                subscribed_destination,
            } => {
                record.record_type = OneOfrecord_type::stomp_connect(STOMPConnectRecord {
                    version,
                    subscribed_destination: subscribed_destination.into(),
                });
            }
            RecordType::Disconnect {
                reason,
                reason_code,
            } => {
                record.record_type = OneOfrecord_type::disconnect(DisconnectRecord {
                    reason: reason.into(),
                    reason_code,
                });
            }
            RecordType::UDSConnect => {
                record.record_type = OneOfrecord_type::uds_connect(UDSConnectRecord {});
            }
        }

        Ok(record)
    }
}
