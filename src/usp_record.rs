#![allow(nonstandard_style)]

use super::*;
use quick_protobuf::sizeofs::*;
use quick_protobuf::{BytesReader, MessageRead, MessageWrite, Result, Writer};
use std::borrow::Cow;
use std::io::Write;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Record<'a> {
    pub version: Option<Cow<'a, str>>,
    pub to_id: Option<Cow<'a, str>>,
    pub from_id: Option<Cow<'a, str>>,
    pub payload_security: Option<usp_record::mod_Record::PayloadSecurity>,
    pub mac_signature: Option<Cow<'a, [u8]>>,
    pub sender_cert: Option<Cow<'a, [u8]>>,
    pub record_type: usp_record::mod_Record::OneOfrecord_type<'a>,
}

impl<'a> MessageRead<'a> for Record<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.version = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.to_id = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.from_id = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(32) => msg.payload_security = Some(r.read_enum(bytes)?),
                Ok(42) => msg.mac_signature = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(50) => msg.sender_cert = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(58) => {
                    msg.record_type = usp_record::mod_Record::OneOfrecord_type::no_session_context(
                        r.read_message::<usp_record::NoSessionContextRecord>(bytes)?,
                    )
                }
                Ok(66) => {
                    msg.record_type = usp_record::mod_Record::OneOfrecord_type::session_context(
                        r.read_message::<usp_record::SessionContextRecord>(bytes)?,
                    )
                }
                Ok(t) => {
                    r.read_unknown(bytes, t)?;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Record<'a> {
    fn get_size(&self) -> usize {
        0 + self
            .version
            .as_ref()
            .map_or(0, |m| 1 + sizeof_len((m).len()))
            + self.to_id.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
            + self
                .from_id
                .as_ref()
                .map_or(0, |m| 1 + sizeof_len((m).len()))
            + self
                .payload_security
                .as_ref()
                .map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
            + self
                .mac_signature
                .as_ref()
                .map_or(0, |m| 1 + sizeof_len((m).len()))
            + self
                .sender_cert
                .as_ref()
                .map_or(0, |m| 1 + sizeof_len((m).len()))
            + match self.record_type {
                usp_record::mod_Record::OneOfrecord_type::no_session_context(ref m) => {
                    1 + sizeof_len((m).get_size())
                }
                usp_record::mod_Record::OneOfrecord_type::session_context(ref m) => {
                    1 + sizeof_len((m).get_size())
                }
                usp_record::mod_Record::OneOfrecord_type::None => 0,
            }
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.version {
            w.write_with_tag(10, |w| w.write_string(&**s))?;
        }
        if let Some(ref s) = self.to_id {
            w.write_with_tag(18, |w| w.write_string(&**s))?;
        }
        if let Some(ref s) = self.from_id {
            w.write_with_tag(26, |w| w.write_string(&**s))?;
        }
        if let Some(ref s) = self.payload_security {
            w.write_with_tag(32, |w| w.write_enum(*s as i32))?;
        }
        if let Some(ref s) = self.mac_signature {
            w.write_with_tag(42, |w| w.write_bytes(&**s))?;
        }
        if let Some(ref s) = self.sender_cert {
            w.write_with_tag(50, |w| w.write_bytes(&**s))?;
        }
        match self.record_type {
            usp_record::mod_Record::OneOfrecord_type::no_session_context(ref m) => {
                w.write_with_tag(58, |w| w.write_message(m))?
            }
            usp_record::mod_Record::OneOfrecord_type::session_context(ref m) => {
                w.write_with_tag(66, |w| w.write_message(m))?
            }
            usp_record::mod_Record::OneOfrecord_type::None => {}
        }
        Ok(())
    }
}

pub mod mod_Record {

    use super::*;

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum PayloadSecurity {
        PLAINTEXT = 0,
        TLS12 = 1,
    }

    impl Default for PayloadSecurity {
        fn default() -> Self {
            PayloadSecurity::PLAINTEXT
        }
    }

    impl From<i32> for PayloadSecurity {
        fn from(i: i32) -> Self {
            match i {
                0 => PayloadSecurity::PLAINTEXT,
                1 => PayloadSecurity::TLS12,
                _ => Self::default(),
            }
        }
    }

    impl<'a> From<&'a str> for PayloadSecurity {
        fn from(s: &'a str) -> Self {
            match s {
                "PLAINTEXT" => PayloadSecurity::PLAINTEXT,
                "TLS12" => PayloadSecurity::TLS12,
                _ => Self::default(),
            }
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum OneOfrecord_type<'a> {
        no_session_context(usp_record::NoSessionContextRecord<'a>),
        session_context(usp_record::SessionContextRecord<'a>),
        None,
    }

    impl<'a> Default for OneOfrecord_type<'a> {
        fn default() -> Self {
            OneOfrecord_type::None
        }
    }

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct NoSessionContextRecord<'a> {
    pub payload: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for NoSessionContextRecord<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(18) => msg.payload = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => {
                    r.read_unknown(bytes, t)?;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for NoSessionContextRecord<'a> {
    fn get_size(&self) -> usize {
        0 + self
            .payload
            .as_ref()
            .map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.payload {
            w.write_with_tag(18, |w| w.write_bytes(&**s))?;
        }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SessionContextRecord<'a> {
    pub session_id: Option<u64>,
    pub sequence_id: Option<u64>,
    pub expected_id: Option<u64>,
    pub retransmit_id: Option<u64>,
    pub payload_sar_state: Option<usp_record::mod_SessionContextRecord::PayloadSARState>,
    pub payloadrec_sar_state: Option<usp_record::mod_SessionContextRecord::PayloadSARState>,
    pub payload: Vec<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for SessionContextRecord<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.session_id = Some(r.read_uint64(bytes)?),
                Ok(16) => msg.sequence_id = Some(r.read_uint64(bytes)?),
                Ok(24) => msg.expected_id = Some(r.read_uint64(bytes)?),
                Ok(32) => msg.retransmit_id = Some(r.read_uint64(bytes)?),
                Ok(40) => msg.payload_sar_state = Some(r.read_enum(bytes)?),
                Ok(48) => msg.payloadrec_sar_state = Some(r.read_enum(bytes)?),
                Ok(58) => msg.payload.push(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => {
                    r.read_unknown(bytes, t)?;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for SessionContextRecord<'a> {
    fn get_size(&self) -> usize {
        0 + self
            .session_id
            .as_ref()
            .map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
            + self
                .sequence_id
                .as_ref()
                .map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
            + self
                .expected_id
                .as_ref()
                .map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
            + self
                .retransmit_id
                .as_ref()
                .map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
            + self
                .payload_sar_state
                .as_ref()
                .map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
            + self
                .payloadrec_sar_state
                .as_ref()
                .map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
            + self
                .payload
                .iter()
                .map(|s| 1 + sizeof_len((s).len()))
                .sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.session_id {
            w.write_with_tag(8, |w| w.write_uint64(*s))?;
        }
        if let Some(ref s) = self.sequence_id {
            w.write_with_tag(16, |w| w.write_uint64(*s))?;
        }
        if let Some(ref s) = self.expected_id {
            w.write_with_tag(24, |w| w.write_uint64(*s))?;
        }
        if let Some(ref s) = self.retransmit_id {
            w.write_with_tag(32, |w| w.write_uint64(*s))?;
        }
        if let Some(ref s) = self.payload_sar_state {
            w.write_with_tag(40, |w| w.write_enum(*s as i32))?;
        }
        if let Some(ref s) = self.payloadrec_sar_state {
            w.write_with_tag(48, |w| w.write_enum(*s as i32))?;
        }
        for s in &self.payload {
            w.write_with_tag(58, |w| w.write_bytes(&**s))?;
        }
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
            PayloadSARState::NONE
        }
    }

    impl From<i32> for PayloadSARState {
        fn from(i: i32) -> Self {
            match i {
                0 => PayloadSARState::NONE,
                1 => PayloadSARState::BEGIN,
                2 => PayloadSARState::INPROCESS,
                3 => PayloadSARState::COMPLETE,
                _ => Self::default(),
            }
        }
    }

    impl<'a> From<&'a str> for PayloadSARState {
        fn from(s: &'a str) -> Self {
            match s {
                "NONE" => PayloadSARState::NONE,
                "BEGIN" => PayloadSARState::BEGIN,
                "INPROCESS" => PayloadSARState::INPROCESS,
                "COMPLETE" => PayloadSARState::COMPLETE,
                _ => Self::default(),
            }
        }
    }

}
