// Automatically generated rust module for 'usp-record-1-3.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use std::borrow::Cow;
use quick_protobuf::{MessageInfo, MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result};
use quick_protobuf::sizeofs::{sizeof_len, sizeof_varint};
use super::usp_record;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Record<'a> {
    pub version: Cow<'a, str>,
    pub to_id: Cow<'a, str>,
    pub from_id: Cow<'a, str>,
    pub payload_security: usp_record::mod_Record::PayloadSecurity,
    pub mac_signature: Cow<'a, [u8]>,
    pub sender_cert: Cow<'a, [u8]>,
    pub record_type: usp_record::mod_Record::OneOfrecord_type<'a>,
}

impl<'a> MessageRead<'a> for Record<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.version = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.to_id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.from_id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(32) => msg.payload_security = r.read_enum(bytes)?,
                Ok(42) => msg.mac_signature = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(50) => msg.sender_cert = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(58) => msg.record_type = usp_record::mod_Record::OneOfrecord_type::no_session_context(r.read_message::<usp_record::NoSessionContextRecord>(bytes)?),
                Ok(66) => msg.record_type = usp_record::mod_Record::OneOfrecord_type::session_context(r.read_message::<usp_record::SessionContextRecord>(bytes)?),
                Ok(74) => msg.record_type = usp_record::mod_Record::OneOfrecord_type::websocket_connect(r.read_message::<usp_record::WebSocketConnectRecord>(bytes)?),
                Ok(82) => msg.record_type = usp_record::mod_Record::OneOfrecord_type::mqtt_connect(r.read_message::<usp_record::MQTTConnectRecord>(bytes)?),
                Ok(90) => msg.record_type = usp_record::mod_Record::OneOfrecord_type::stomp_connect(r.read_message::<usp_record::STOMPConnectRecord>(bytes)?),
                Ok(98) => msg.record_type = usp_record::mod_Record::OneOfrecord_type::disconnect(r.read_message::<usp_record::DisconnectRecord>(bytes)?),
                Ok(106) => msg.record_type = usp_record::mod_Record::OneOfrecord_type::uds_connect(r.read_message::<usp_record::UDSConnectRecord>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Record<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.version == "" { 0 } else { 1 + sizeof_len((&self.version).len()) }
        + if self.to_id == "" { 0 } else { 1 + sizeof_len((&self.to_id).len()) }
        + if self.from_id == "" { 0 } else { 1 + sizeof_len((&self.from_id).len()) }
        + if self.payload_security == usp_record::mod_Record::PayloadSecurity::PLAINTEXT { 0 } else { 1 + sizeof_varint(*(&self.payload_security) as u64) }
        + if self.mac_signature == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.mac_signature).len()) }
        + if self.sender_cert == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.sender_cert).len()) }
        + match self.record_type {
            usp_record::mod_Record::OneOfrecord_type::no_session_context(ref m) => 1 + sizeof_len((m).get_size()),
            usp_record::mod_Record::OneOfrecord_type::session_context(ref m) => 1 + sizeof_len((m).get_size()),
            usp_record::mod_Record::OneOfrecord_type::websocket_connect(ref m) => 1 + sizeof_len((m).get_size()),
            usp_record::mod_Record::OneOfrecord_type::mqtt_connect(ref m) => 1 + sizeof_len((m).get_size()),
            usp_record::mod_Record::OneOfrecord_type::stomp_connect(ref m) => 1 + sizeof_len((m).get_size()),
            usp_record::mod_Record::OneOfrecord_type::disconnect(ref m) => 1 + sizeof_len((m).get_size()),
            usp_record::mod_Record::OneOfrecord_type::uds_connect(ref m) => 1 + sizeof_len((m).get_size()),
            usp_record::mod_Record::OneOfrecord_type::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.version != "" { w.write_with_tag(10, |w| w.write_string(&**&self.version))?; }
        if self.to_id != "" { w.write_with_tag(18, |w| w.write_string(&**&self.to_id))?; }
        if self.from_id != "" { w.write_with_tag(26, |w| w.write_string(&**&self.from_id))?; }
        if self.payload_security != usp_record::mod_Record::PayloadSecurity::PLAINTEXT { w.write_with_tag(32, |w| w.write_enum(*&self.payload_security as i32))?; }
        if self.mac_signature != Cow::Borrowed(b"") { w.write_with_tag(42, |w| w.write_bytes(&**&self.mac_signature))?; }
        if self.sender_cert != Cow::Borrowed(b"") { w.write_with_tag(50, |w| w.write_bytes(&**&self.sender_cert))?; }
        match self.record_type {            usp_record::mod_Record::OneOfrecord_type::no_session_context(ref m) => { w.write_with_tag(58, |w| w.write_message(m))? },
            usp_record::mod_Record::OneOfrecord_type::session_context(ref m) => { w.write_with_tag(66, |w| w.write_message(m))? },
            usp_record::mod_Record::OneOfrecord_type::websocket_connect(ref m) => { w.write_with_tag(74, |w| w.write_message(m))? },
            usp_record::mod_Record::OneOfrecord_type::mqtt_connect(ref m) => { w.write_with_tag(82, |w| w.write_message(m))? },
            usp_record::mod_Record::OneOfrecord_type::stomp_connect(ref m) => { w.write_with_tag(90, |w| w.write_message(m))? },
            usp_record::mod_Record::OneOfrecord_type::disconnect(ref m) => { w.write_with_tag(98, |w| w.write_message(m))? },
            usp_record::mod_Record::OneOfrecord_type::uds_connect(ref m) => { w.write_with_tag(106, |w| w.write_message(m))? },
            usp_record::mod_Record::OneOfrecord_type::None => {},
    }        Ok(())
    }
}

pub mod mod_Record {

use super::usp_record;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PayloadSecurity {
    PLAINTEXT = 0,
    TLS12 = 1,
}

impl Default for PayloadSecurity {
    fn default() -> Self {
        Self::PLAINTEXT
    }
}

impl From<i32> for PayloadSecurity {
    fn from(i: i32) -> Self {
        match i {
            0 => Self::PLAINTEXT,
            1 => Self::TLS12,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for PayloadSecurity {
    fn from(s: &'a str) -> Self {
        match s {
            "PLAINTEXT" => Self::PLAINTEXT,
            "TLS12" => Self::TLS12,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfrecord_type<'a> {
    no_session_context(usp_record::NoSessionContextRecord<'a>),
    session_context(usp_record::SessionContextRecord<'a>),
    websocket_connect(usp_record::WebSocketConnectRecord),
    mqtt_connect(usp_record::MQTTConnectRecord<'a>),
    stomp_connect(usp_record::STOMPConnectRecord<'a>),
    disconnect(usp_record::DisconnectRecord<'a>),
    uds_connect(usp_record::UDSConnectRecord),
    None,
}

impl<'a> Default for OneOfrecord_type<'a> {
    fn default() -> Self {
        OneOfrecord_type::None
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct NoSessionContextRecord<'a> {
    pub payload: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for NoSessionContextRecord<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(18) => msg.payload = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for NoSessionContextRecord<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.payload == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.payload).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.payload != Cow::Borrowed(b"") { w.write_with_tag(18, |w| w.write_bytes(&**&self.payload))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct SessionContextRecord<'a> {
    pub session_id: u64,
    pub sequence_id: u64,
    pub expected_id: u64,
    pub retransmit_id: u64,
    pub payload_sar_state: usp_record::mod_SessionContextRecord::PayloadSARState,
    pub payloadrec_sar_state: usp_record::mod_SessionContextRecord::PayloadSARState,
    pub payload: Vec<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for SessionContextRecord<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.session_id = r.read_uint64(bytes)?,
                Ok(16) => msg.sequence_id = r.read_uint64(bytes)?,
                Ok(24) => msg.expected_id = r.read_uint64(bytes)?,
                Ok(32) => msg.retransmit_id = r.read_uint64(bytes)?,
                Ok(40) => msg.payload_sar_state = r.read_enum(bytes)?,
                Ok(48) => msg.payloadrec_sar_state = r.read_enum(bytes)?,
                Ok(58) => msg.payload.push(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for SessionContextRecord<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.session_id == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.session_id) as u64) }
        + if self.sequence_id == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.sequence_id) as u64) }
        + if self.expected_id == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.expected_id) as u64) }
        + if self.retransmit_id == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.retransmit_id) as u64) }
        + if self.payload_sar_state == usp_record::mod_SessionContextRecord::PayloadSARState::NONE { 0 } else { 1 + sizeof_varint(*(&self.payload_sar_state) as u64) }
        + if self.payloadrec_sar_state == usp_record::mod_SessionContextRecord::PayloadSARState::NONE { 0 } else { 1 + sizeof_varint(*(&self.payloadrec_sar_state) as u64) }
        + self.payload.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.session_id != 0u64 { w.write_with_tag(8, |w| w.write_uint64(*&self.session_id))?; }
        if self.sequence_id != 0u64 { w.write_with_tag(16, |w| w.write_uint64(*&self.sequence_id))?; }
        if self.expected_id != 0u64 { w.write_with_tag(24, |w| w.write_uint64(*&self.expected_id))?; }
        if self.retransmit_id != 0u64 { w.write_with_tag(32, |w| w.write_uint64(*&self.retransmit_id))?; }
        if self.payload_sar_state != usp_record::mod_SessionContextRecord::PayloadSARState::NONE { w.write_with_tag(40, |w| w.write_enum(*&self.payload_sar_state as i32))?; }
        if self.payloadrec_sar_state != usp_record::mod_SessionContextRecord::PayloadSARState::NONE { w.write_with_tag(48, |w| w.write_enum(*&self.payloadrec_sar_state as i32))?; }
        for s in &self.payload { w.write_with_tag(58, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

pub mod mod_SessionContextRecord {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PayloadSARState {
    NONE = 0,
    BEGIN = 1,
    INPROCESS = 2,
    COMPLETE = 3,
}

impl Default for PayloadSARState {
    fn default() -> Self {
        Self::NONE
    }
}

impl From<i32> for PayloadSARState {
    fn from(i: i32) -> Self {
        match i {
            0 => Self::NONE,
            1 => Self::BEGIN,
            2 => Self::INPROCESS,
            3 => Self::COMPLETE,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for PayloadSARState {
    fn from(s: &'a str) -> Self {
        match s {
            "NONE" => Self::NONE,
            "BEGIN" => Self::BEGIN,
            "INPROCESS" => Self::INPROCESS,
            "COMPLETE" => Self::COMPLETE,
            _ => Self::default(),
        }
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct WebSocketConnectRecord { }

impl<'a> MessageRead<'a> for WebSocketConnectRecord {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for WebSocketConnectRecord { }

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct MQTTConnectRecord<'a> {
    pub version: usp_record::mod_MQTTConnectRecord::MQTTVersion,
    pub subscribed_topic: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for MQTTConnectRecord<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.version = r.read_enum(bytes)?,
                Ok(18) => msg.subscribed_topic = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for MQTTConnectRecord<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.version == usp_record::mod_MQTTConnectRecord::MQTTVersion::V3_1_1 { 0 } else { 1 + sizeof_varint(*(&self.version) as u64) }
        + if self.subscribed_topic == "" { 0 } else { 1 + sizeof_len((&self.subscribed_topic).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.version != usp_record::mod_MQTTConnectRecord::MQTTVersion::V3_1_1 { w.write_with_tag(8, |w| w.write_enum(*&self.version as i32))?; }
        if self.subscribed_topic != "" { w.write_with_tag(18, |w| w.write_string(&**&self.subscribed_topic))?; }
        Ok(())
    }
}

pub mod mod_MQTTConnectRecord {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MQTTVersion {
    V3_1_1 = 0,
    V5 = 1,
}

impl Default for MQTTVersion {
    fn default() -> Self {
        Self::V3_1_1
    }
}

impl From<i32> for MQTTVersion {
    fn from(i: i32) -> Self {
        match i {
            0 => Self::V3_1_1,
            1 => Self::V5,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for MQTTVersion {
    fn from(s: &'a str) -> Self {
        match s {
            "V3_1_1" => Self::V3_1_1,
            "V5" => Self::V5,
            _ => Self::default(),
        }
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct STOMPConnectRecord<'a> {
    pub version: usp_record::mod_STOMPConnectRecord::STOMPVersion,
    pub subscribed_destination: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for STOMPConnectRecord<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.version = r.read_enum(bytes)?,
                Ok(18) => msg.subscribed_destination = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for STOMPConnectRecord<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.version == usp_record::mod_STOMPConnectRecord::STOMPVersion::V1_2 { 0 } else { 1 + sizeof_varint(*(&self.version) as u64) }
        + if self.subscribed_destination == "" { 0 } else { 1 + sizeof_len((&self.subscribed_destination).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.version != usp_record::mod_STOMPConnectRecord::STOMPVersion::V1_2 { w.write_with_tag(8, |w| w.write_enum(*&self.version as i32))?; }
        if self.subscribed_destination != "" { w.write_with_tag(18, |w| w.write_string(&**&self.subscribed_destination))?; }
        Ok(())
    }
}

pub mod mod_STOMPConnectRecord {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum STOMPVersion {
    V1_2 = 0,
}

impl Default for STOMPVersion {
    fn default() -> Self {
        Self::V1_2
    }
}

impl From<i32> for STOMPVersion {
    fn from(i: i32) -> Self {
        match i {
            0 => Self::V1_2,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for STOMPVersion {
    fn from(s: &'a str) -> Self {
        match s {
            "V1_2" => Self::V1_2,
            _ => Self::default(),
        }
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct UDSConnectRecord { }

impl<'a> MessageRead<'a> for UDSConnectRecord {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for UDSConnectRecord { }

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct DisconnectRecord<'a> {
    pub reason: Cow<'a, str>,
    pub reason_code: u32,
}

impl<'a> MessageRead<'a> for DisconnectRecord<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.reason = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(21) => msg.reason_code = r.read_fixed32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for DisconnectRecord<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.reason == "" { 0 } else { 1 + sizeof_len((&self.reason).len()) }
        + if self.reason_code == 0u32 { 0 } else { 1 + 4 }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.reason != "" { w.write_with_tag(10, |w| w.write_string(&**&self.reason))?; }
        if self.reason_code != 0u32 { w.write_with_tag(21, |w| w.write_fixed32(*&self.reason_code))?; }
        Ok(())
    }
}

