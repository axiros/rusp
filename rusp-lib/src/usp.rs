// Automatically generated rust module for 'usp-msg-1-4.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use std::collections::HashMap;
type KVMap<K, V> = HashMap<K, V>;
use quick_protobuf::{MessageInfo, MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result};
use quick_protobuf::sizeofs::*;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Msg {
    pub header: Option<usp::Header>,
    pub body: Option<usp::Body>,
}

impl<'a> MessageRead<'a> for Msg {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.header = Some(r.read_message::<usp::Header>(bytes)?),
                Ok(18) => msg.body = Some(r.read_message::<usp::Body>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Msg {
    fn get_size(&self) -> usize {
        0
        + self.header.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.body.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.header { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.body { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Header {
    pub msg_id: String,
    pub msg_type: usp::mod_Header::MsgType,
}

impl<'a> MessageRead<'a> for Header {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.msg_id = r.read_string(bytes)?.to_owned(),
                Ok(16) => msg.msg_type = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Header {
    fn get_size(&self) -> usize {
        0
        + if self.msg_id == String::default() { 0 } else { 1 + sizeof_len((&self.msg_id).len()) }
        + if self.msg_type == usp::mod_Header::MsgType::ERROR { 0 } else { 1 + sizeof_varint(*(&self.msg_type) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.msg_id != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.msg_id))?; }
        if self.msg_type != usp::mod_Header::MsgType::ERROR { w.write_with_tag(16, |w| w.write_enum(*&self.msg_type as i32))?; }
        Ok(())
    }
}

pub mod mod_Header {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MsgType {
    ERROR = 0,
    GET = 1,
    GET_RESP = 2,
    NOTIFY = 3,
    SET = 4,
    SET_RESP = 5,
    OPERATE = 6,
    OPERATE_RESP = 7,
    ADD = 8,
    ADD_RESP = 9,
    DELETE = 10,
    DELETE_RESP = 11,
    GET_SUPPORTED_DM = 12,
    GET_SUPPORTED_DM_RESP = 13,
    GET_INSTANCES = 14,
    GET_INSTANCES_RESP = 15,
    NOTIFY_RESP = 16,
    GET_SUPPORTED_PROTO = 17,
    GET_SUPPORTED_PROTO_RESP = 18,
    REGISTER = 19,
    REGISTER_RESP = 20,
    DEREGISTER = 21,
    DEREGISTER_RESP = 22,
}

impl Default for MsgType {
    fn default() -> Self {
        MsgType::ERROR
    }
}

impl From<i32> for MsgType {
    fn from(i: i32) -> Self {
        match i {
            0 => MsgType::ERROR,
            1 => MsgType::GET,
            2 => MsgType::GET_RESP,
            3 => MsgType::NOTIFY,
            4 => MsgType::SET,
            5 => MsgType::SET_RESP,
            6 => MsgType::OPERATE,
            7 => MsgType::OPERATE_RESP,
            8 => MsgType::ADD,
            9 => MsgType::ADD_RESP,
            10 => MsgType::DELETE,
            11 => MsgType::DELETE_RESP,
            12 => MsgType::GET_SUPPORTED_DM,
            13 => MsgType::GET_SUPPORTED_DM_RESP,
            14 => MsgType::GET_INSTANCES,
            15 => MsgType::GET_INSTANCES_RESP,
            16 => MsgType::NOTIFY_RESP,
            17 => MsgType::GET_SUPPORTED_PROTO,
            18 => MsgType::GET_SUPPORTED_PROTO_RESP,
            19 => MsgType::REGISTER,
            20 => MsgType::REGISTER_RESP,
            21 => MsgType::DEREGISTER,
            22 => MsgType::DEREGISTER_RESP,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for MsgType {
    fn from(s: &'a str) -> Self {
        match s {
            "ERROR" => MsgType::ERROR,
            "GET" => MsgType::GET,
            "GET_RESP" => MsgType::GET_RESP,
            "NOTIFY" => MsgType::NOTIFY,
            "SET" => MsgType::SET,
            "SET_RESP" => MsgType::SET_RESP,
            "OPERATE" => MsgType::OPERATE,
            "OPERATE_RESP" => MsgType::OPERATE_RESP,
            "ADD" => MsgType::ADD,
            "ADD_RESP" => MsgType::ADD_RESP,
            "DELETE" => MsgType::DELETE,
            "DELETE_RESP" => MsgType::DELETE_RESP,
            "GET_SUPPORTED_DM" => MsgType::GET_SUPPORTED_DM,
            "GET_SUPPORTED_DM_RESP" => MsgType::GET_SUPPORTED_DM_RESP,
            "GET_INSTANCES" => MsgType::GET_INSTANCES,
            "GET_INSTANCES_RESP" => MsgType::GET_INSTANCES_RESP,
            "NOTIFY_RESP" => MsgType::NOTIFY_RESP,
            "GET_SUPPORTED_PROTO" => MsgType::GET_SUPPORTED_PROTO,
            "GET_SUPPORTED_PROTO_RESP" => MsgType::GET_SUPPORTED_PROTO_RESP,
            "REGISTER" => MsgType::REGISTER,
            "REGISTER_RESP" => MsgType::REGISTER_RESP,
            "DEREGISTER" => MsgType::DEREGISTER,
            "DEREGISTER_RESP" => MsgType::DEREGISTER_RESP,
            _ => Self::default(),
        }
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Body {
    pub msg_body: usp::mod_Body::OneOfmsg_body,
}

impl<'a> MessageRead<'a> for Body {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.msg_body = usp::mod_Body::OneOfmsg_body::request(r.read_message::<usp::Request>(bytes)?),
                Ok(18) => msg.msg_body = usp::mod_Body::OneOfmsg_body::response(r.read_message::<usp::Response>(bytes)?),
                Ok(26) => msg.msg_body = usp::mod_Body::OneOfmsg_body::error(r.read_message::<usp::Error>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Body {
    fn get_size(&self) -> usize {
        0
        + match self.msg_body {
            usp::mod_Body::OneOfmsg_body::request(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_Body::OneOfmsg_body::response(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_Body::OneOfmsg_body::error(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_Body::OneOfmsg_body::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        match self.msg_body {            usp::mod_Body::OneOfmsg_body::request(ref m) => { w.write_with_tag(10, |w| w.write_message(m))? },
            usp::mod_Body::OneOfmsg_body::response(ref m) => { w.write_with_tag(18, |w| w.write_message(m))? },
            usp::mod_Body::OneOfmsg_body::error(ref m) => { w.write_with_tag(26, |w| w.write_message(m))? },
            usp::mod_Body::OneOfmsg_body::None => {},
    }        Ok(())
    }
}

pub mod mod_Body {

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfmsg_body {
    request(usp::Request),
    response(usp::Response),
    error(usp::Error),
    None,
}

impl Default for OneOfmsg_body {
    fn default() -> Self {
        OneOfmsg_body::None
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Request {
    pub req_type: usp::mod_Request::OneOfreq_type,
}

impl<'a> MessageRead<'a> for Request {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.req_type = usp::mod_Request::OneOfreq_type::get(r.read_message::<usp::Get>(bytes)?),
                Ok(18) => msg.req_type = usp::mod_Request::OneOfreq_type::get_supported_dm(r.read_message::<usp::GetSupportedDM>(bytes)?),
                Ok(26) => msg.req_type = usp::mod_Request::OneOfreq_type::get_instances(r.read_message::<usp::GetInstances>(bytes)?),
                Ok(34) => msg.req_type = usp::mod_Request::OneOfreq_type::set(r.read_message::<usp::Set>(bytes)?),
                Ok(42) => msg.req_type = usp::mod_Request::OneOfreq_type::add(r.read_message::<usp::Add>(bytes)?),
                Ok(50) => msg.req_type = usp::mod_Request::OneOfreq_type::delete(r.read_message::<usp::Delete>(bytes)?),
                Ok(58) => msg.req_type = usp::mod_Request::OneOfreq_type::operate(r.read_message::<usp::Operate>(bytes)?),
                Ok(66) => msg.req_type = usp::mod_Request::OneOfreq_type::notify(r.read_message::<usp::Notify>(bytes)?),
                Ok(74) => msg.req_type = usp::mod_Request::OneOfreq_type::get_supported_protocol(r.read_message::<usp::GetSupportedProtocol>(bytes)?),
                Ok(82) => msg.req_type = usp::mod_Request::OneOfreq_type::register(r.read_message::<usp::Register>(bytes)?),
                Ok(90) => msg.req_type = usp::mod_Request::OneOfreq_type::deregister(r.read_message::<usp::Deregister>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Request {
    fn get_size(&self) -> usize {
        0
        + match self.req_type {
            usp::mod_Request::OneOfreq_type::get(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_Request::OneOfreq_type::get_supported_dm(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_Request::OneOfreq_type::get_instances(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_Request::OneOfreq_type::set(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_Request::OneOfreq_type::add(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_Request::OneOfreq_type::delete(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_Request::OneOfreq_type::operate(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_Request::OneOfreq_type::notify(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_Request::OneOfreq_type::get_supported_protocol(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_Request::OneOfreq_type::register(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_Request::OneOfreq_type::deregister(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_Request::OneOfreq_type::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        match self.req_type {            usp::mod_Request::OneOfreq_type::get(ref m) => { w.write_with_tag(10, |w| w.write_message(m))? },
            usp::mod_Request::OneOfreq_type::get_supported_dm(ref m) => { w.write_with_tag(18, |w| w.write_message(m))? },
            usp::mod_Request::OneOfreq_type::get_instances(ref m) => { w.write_with_tag(26, |w| w.write_message(m))? },
            usp::mod_Request::OneOfreq_type::set(ref m) => { w.write_with_tag(34, |w| w.write_message(m))? },
            usp::mod_Request::OneOfreq_type::add(ref m) => { w.write_with_tag(42, |w| w.write_message(m))? },
            usp::mod_Request::OneOfreq_type::delete(ref m) => { w.write_with_tag(50, |w| w.write_message(m))? },
            usp::mod_Request::OneOfreq_type::operate(ref m) => { w.write_with_tag(58, |w| w.write_message(m))? },
            usp::mod_Request::OneOfreq_type::notify(ref m) => { w.write_with_tag(66, |w| w.write_message(m))? },
            usp::mod_Request::OneOfreq_type::get_supported_protocol(ref m) => { w.write_with_tag(74, |w| w.write_message(m))? },
            usp::mod_Request::OneOfreq_type::register(ref m) => { w.write_with_tag(82, |w| w.write_message(m))? },
            usp::mod_Request::OneOfreq_type::deregister(ref m) => { w.write_with_tag(90, |w| w.write_message(m))? },
            usp::mod_Request::OneOfreq_type::None => {},
    }        Ok(())
    }
}

pub mod mod_Request {

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfreq_type {
    get(usp::Get),
    get_supported_dm(usp::GetSupportedDM),
    get_instances(usp::GetInstances),
    set(usp::Set),
    add(usp::Add),
    delete(usp::Delete),
    operate(usp::Operate),
    notify(usp::Notify),
    get_supported_protocol(usp::GetSupportedProtocol),
    register(usp::Register),
    deregister(usp::Deregister),
    None,
}

impl Default for OneOfreq_type {
    fn default() -> Self {
        OneOfreq_type::None
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Response {
    pub resp_type: usp::mod_Response::OneOfresp_type,
}

impl<'a> MessageRead<'a> for Response {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.resp_type = usp::mod_Response::OneOfresp_type::get_resp(r.read_message::<usp::GetResp>(bytes)?),
                Ok(18) => msg.resp_type = usp::mod_Response::OneOfresp_type::get_supported_dm_resp(r.read_message::<usp::GetSupportedDMResp>(bytes)?),
                Ok(26) => msg.resp_type = usp::mod_Response::OneOfresp_type::get_instances_resp(r.read_message::<usp::GetInstancesResp>(bytes)?),
                Ok(34) => msg.resp_type = usp::mod_Response::OneOfresp_type::set_resp(r.read_message::<usp::SetResp>(bytes)?),
                Ok(42) => msg.resp_type = usp::mod_Response::OneOfresp_type::add_resp(r.read_message::<usp::AddResp>(bytes)?),
                Ok(50) => msg.resp_type = usp::mod_Response::OneOfresp_type::delete_resp(r.read_message::<usp::DeleteResp>(bytes)?),
                Ok(58) => msg.resp_type = usp::mod_Response::OneOfresp_type::operate_resp(r.read_message::<usp::OperateResp>(bytes)?),
                Ok(66) => msg.resp_type = usp::mod_Response::OneOfresp_type::notify_resp(r.read_message::<usp::NotifyResp>(bytes)?),
                Ok(74) => msg.resp_type = usp::mod_Response::OneOfresp_type::get_supported_protocol_resp(r.read_message::<usp::GetSupportedProtocolResp>(bytes)?),
                Ok(82) => msg.resp_type = usp::mod_Response::OneOfresp_type::register_resp(r.read_message::<usp::RegisterResp>(bytes)?),
                Ok(90) => msg.resp_type = usp::mod_Response::OneOfresp_type::deregister_resp(r.read_message::<usp::DeregisterResp>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Response {
    fn get_size(&self) -> usize {
        0
        + match self.resp_type {
            usp::mod_Response::OneOfresp_type::get_resp(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_Response::OneOfresp_type::get_supported_dm_resp(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_Response::OneOfresp_type::get_instances_resp(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_Response::OneOfresp_type::set_resp(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_Response::OneOfresp_type::add_resp(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_Response::OneOfresp_type::delete_resp(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_Response::OneOfresp_type::operate_resp(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_Response::OneOfresp_type::notify_resp(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_Response::OneOfresp_type::get_supported_protocol_resp(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_Response::OneOfresp_type::register_resp(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_Response::OneOfresp_type::deregister_resp(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_Response::OneOfresp_type::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        match self.resp_type {            usp::mod_Response::OneOfresp_type::get_resp(ref m) => { w.write_with_tag(10, |w| w.write_message(m))? },
            usp::mod_Response::OneOfresp_type::get_supported_dm_resp(ref m) => { w.write_with_tag(18, |w| w.write_message(m))? },
            usp::mod_Response::OneOfresp_type::get_instances_resp(ref m) => { w.write_with_tag(26, |w| w.write_message(m))? },
            usp::mod_Response::OneOfresp_type::set_resp(ref m) => { w.write_with_tag(34, |w| w.write_message(m))? },
            usp::mod_Response::OneOfresp_type::add_resp(ref m) => { w.write_with_tag(42, |w| w.write_message(m))? },
            usp::mod_Response::OneOfresp_type::delete_resp(ref m) => { w.write_with_tag(50, |w| w.write_message(m))? },
            usp::mod_Response::OneOfresp_type::operate_resp(ref m) => { w.write_with_tag(58, |w| w.write_message(m))? },
            usp::mod_Response::OneOfresp_type::notify_resp(ref m) => { w.write_with_tag(66, |w| w.write_message(m))? },
            usp::mod_Response::OneOfresp_type::get_supported_protocol_resp(ref m) => { w.write_with_tag(74, |w| w.write_message(m))? },
            usp::mod_Response::OneOfresp_type::register_resp(ref m) => { w.write_with_tag(82, |w| w.write_message(m))? },
            usp::mod_Response::OneOfresp_type::deregister_resp(ref m) => { w.write_with_tag(90, |w| w.write_message(m))? },
            usp::mod_Response::OneOfresp_type::None => {},
    }        Ok(())
    }
}

pub mod mod_Response {

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfresp_type {
    get_resp(usp::GetResp),
    get_supported_dm_resp(usp::GetSupportedDMResp),
    get_instances_resp(usp::GetInstancesResp),
    set_resp(usp::SetResp),
    add_resp(usp::AddResp),
    delete_resp(usp::DeleteResp),
    operate_resp(usp::OperateResp),
    notify_resp(usp::NotifyResp),
    get_supported_protocol_resp(usp::GetSupportedProtocolResp),
    register_resp(usp::RegisterResp),
    deregister_resp(usp::DeregisterResp),
    None,
}

impl Default for OneOfresp_type {
    fn default() -> Self {
        OneOfresp_type::None
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Error {
    pub err_code: u32,
    pub err_msg: String,
    pub param_errs: Vec<usp::mod_Error::ParamError>,
}

impl<'a> MessageRead<'a> for Error {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(13) => msg.err_code = r.read_fixed32(bytes)?,
                Ok(18) => msg.err_msg = r.read_string(bytes)?.to_owned(),
                Ok(26) => msg.param_errs.push(r.read_message::<usp::mod_Error::ParamError>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Error {
    fn get_size(&self) -> usize {
        0
        + if self.err_code == 0u32 { 0 } else { 1 + 4 }
        + if self.err_msg == String::default() { 0 } else { 1 + sizeof_len((&self.err_msg).len()) }
        + self.param_errs.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.err_code != 0u32 { w.write_with_tag(13, |w| w.write_fixed32(*&self.err_code))?; }
        if self.err_msg != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.err_msg))?; }
        for s in &self.param_errs { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_Error {

use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ParamError {
    pub param_path: String,
    pub err_code: u32,
    pub err_msg: String,
}

impl<'a> MessageRead<'a> for ParamError {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.param_path = r.read_string(bytes)?.to_owned(),
                Ok(21) => msg.err_code = r.read_fixed32(bytes)?,
                Ok(26) => msg.err_msg = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ParamError {
    fn get_size(&self) -> usize {
        0
        + if self.param_path == String::default() { 0 } else { 1 + sizeof_len((&self.param_path).len()) }
        + if self.err_code == 0u32 { 0 } else { 1 + 4 }
        + if self.err_msg == String::default() { 0 } else { 1 + sizeof_len((&self.err_msg).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.param_path != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.param_path))?; }
        if self.err_code != 0u32 { w.write_with_tag(21, |w| w.write_fixed32(*&self.err_code))?; }
        if self.err_msg != String::default() { w.write_with_tag(26, |w| w.write_string(&**&self.err_msg))?; }
        Ok(())
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Get {
    pub param_paths: Vec<String>,
    pub max_depth: u32,
}

impl<'a> MessageRead<'a> for Get {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.param_paths.push(r.read_string(bytes)?.to_owned()),
                Ok(21) => msg.max_depth = r.read_fixed32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Get {
    fn get_size(&self) -> usize {
        0
        + self.param_paths.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + if self.max_depth == 0u32 { 0 } else { 1 + 4 }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.param_paths { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if self.max_depth != 0u32 { w.write_with_tag(21, |w| w.write_fixed32(*&self.max_depth))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct GetResp {
    pub req_path_results: Vec<usp::mod_GetResp::RequestedPathResult>,
}

impl<'a> MessageRead<'a> for GetResp {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.req_path_results.push(r.read_message::<usp::mod_GetResp::RequestedPathResult>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for GetResp {
    fn get_size(&self) -> usize {
        0
        + self.req_path_results.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.req_path_results { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_GetResp {

use std::collections::HashMap;
type KVMap<K, V> = HashMap<K, V>;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct RequestedPathResult {
    pub requested_path: String,
    pub err_code: u32,
    pub err_msg: String,
    pub resolved_path_results: Vec<usp::mod_GetResp::ResolvedPathResult>,
}

impl<'a> MessageRead<'a> for RequestedPathResult {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.requested_path = r.read_string(bytes)?.to_owned(),
                Ok(21) => msg.err_code = r.read_fixed32(bytes)?,
                Ok(26) => msg.err_msg = r.read_string(bytes)?.to_owned(),
                Ok(34) => msg.resolved_path_results.push(r.read_message::<usp::mod_GetResp::ResolvedPathResult>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for RequestedPathResult {
    fn get_size(&self) -> usize {
        0
        + if self.requested_path == String::default() { 0 } else { 1 + sizeof_len((&self.requested_path).len()) }
        + if self.err_code == 0u32 { 0 } else { 1 + 4 }
        + if self.err_msg == String::default() { 0 } else { 1 + sizeof_len((&self.err_msg).len()) }
        + self.resolved_path_results.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.requested_path != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.requested_path))?; }
        if self.err_code != 0u32 { w.write_with_tag(21, |w| w.write_fixed32(*&self.err_code))?; }
        if self.err_msg != String::default() { w.write_with_tag(26, |w| w.write_string(&**&self.err_msg))?; }
        for s in &self.resolved_path_results { w.write_with_tag(34, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ResolvedPathResult {
    pub resolved_path: String,
    pub result_params: KVMap<String, String>,
}

impl<'a> MessageRead<'a> for ResolvedPathResult {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.resolved_path = r.read_string(bytes)?.to_owned(),
                Ok(18) => {
                    let (key, value) = r.read_map(bytes, |r, bytes| Ok(r.read_string(bytes)?.to_owned()), |r, bytes| Ok(r.read_string(bytes)?.to_owned()))?;
                    msg.result_params.insert(key, value);
                }
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ResolvedPathResult {
    fn get_size(&self) -> usize {
        0
        + if self.resolved_path == String::default() { 0 } else { 1 + sizeof_len((&self.resolved_path).len()) }
        + self.result_params.iter().map(|(k, v)| 1 + sizeof_len(2 + sizeof_len((k).len()) + sizeof_len((v).len()))).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.resolved_path != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.resolved_path))?; }
        for (k, v) in self.result_params.iter() { w.write_with_tag(18, |w| w.write_map(2 + sizeof_len((k).len()) + sizeof_len((v).len()), 10, |w| w.write_string(&**k), 18, |w| w.write_string(&**v)))?; }
        Ok(())
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct GetSupportedDM {
    pub obj_paths: Vec<String>,
    pub first_level_only: bool,
    pub return_commands: bool,
    pub return_events: bool,
    pub return_params: bool,
    pub return_unique_key_sets: bool,
}

impl<'a> MessageRead<'a> for GetSupportedDM {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.obj_paths.push(r.read_string(bytes)?.to_owned()),
                Ok(16) => msg.first_level_only = r.read_bool(bytes)?,
                Ok(24) => msg.return_commands = r.read_bool(bytes)?,
                Ok(32) => msg.return_events = r.read_bool(bytes)?,
                Ok(40) => msg.return_params = r.read_bool(bytes)?,
                Ok(48) => msg.return_unique_key_sets = r.read_bool(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for GetSupportedDM {
    fn get_size(&self) -> usize {
        0
        + self.obj_paths.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + if self.first_level_only == false { 0 } else { 1 + sizeof_varint(*(&self.first_level_only) as u64) }
        + if self.return_commands == false { 0 } else { 1 + sizeof_varint(*(&self.return_commands) as u64) }
        + if self.return_events == false { 0 } else { 1 + sizeof_varint(*(&self.return_events) as u64) }
        + if self.return_params == false { 0 } else { 1 + sizeof_varint(*(&self.return_params) as u64) }
        + if self.return_unique_key_sets == false { 0 } else { 1 + sizeof_varint(*(&self.return_unique_key_sets) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.obj_paths { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if self.first_level_only != false { w.write_with_tag(16, |w| w.write_bool(*&self.first_level_only))?; }
        if self.return_commands != false { w.write_with_tag(24, |w| w.write_bool(*&self.return_commands))?; }
        if self.return_events != false { w.write_with_tag(32, |w| w.write_bool(*&self.return_events))?; }
        if self.return_params != false { w.write_with_tag(40, |w| w.write_bool(*&self.return_params))?; }
        if self.return_unique_key_sets != false { w.write_with_tag(48, |w| w.write_bool(*&self.return_unique_key_sets))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct GetSupportedDMResp {
    pub req_obj_results: Vec<usp::mod_GetSupportedDMResp::RequestedObjectResult>,
}

impl<'a> MessageRead<'a> for GetSupportedDMResp {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.req_obj_results.push(r.read_message::<usp::mod_GetSupportedDMResp::RequestedObjectResult>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for GetSupportedDMResp {
    fn get_size(&self) -> usize {
        0
        + self.req_obj_results.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.req_obj_results { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_GetSupportedDMResp {

use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct RequestedObjectResult {
    pub req_obj_path: String,
    pub err_code: u32,
    pub err_msg: String,
    pub data_model_inst_uri: String,
    pub supported_objs: Vec<usp::mod_GetSupportedDMResp::SupportedObjectResult>,
}

impl<'a> MessageRead<'a> for RequestedObjectResult {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.req_obj_path = r.read_string(bytes)?.to_owned(),
                Ok(21) => msg.err_code = r.read_fixed32(bytes)?,
                Ok(26) => msg.err_msg = r.read_string(bytes)?.to_owned(),
                Ok(34) => msg.data_model_inst_uri = r.read_string(bytes)?.to_owned(),
                Ok(42) => msg.supported_objs.push(r.read_message::<usp::mod_GetSupportedDMResp::SupportedObjectResult>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for RequestedObjectResult {
    fn get_size(&self) -> usize {
        0
        + if self.req_obj_path == String::default() { 0 } else { 1 + sizeof_len((&self.req_obj_path).len()) }
        + if self.err_code == 0u32 { 0 } else { 1 + 4 }
        + if self.err_msg == String::default() { 0 } else { 1 + sizeof_len((&self.err_msg).len()) }
        + if self.data_model_inst_uri == String::default() { 0 } else { 1 + sizeof_len((&self.data_model_inst_uri).len()) }
        + self.supported_objs.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.req_obj_path != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.req_obj_path))?; }
        if self.err_code != 0u32 { w.write_with_tag(21, |w| w.write_fixed32(*&self.err_code))?; }
        if self.err_msg != String::default() { w.write_with_tag(26, |w| w.write_string(&**&self.err_msg))?; }
        if self.data_model_inst_uri != String::default() { w.write_with_tag(34, |w| w.write_string(&**&self.data_model_inst_uri))?; }
        for s in &self.supported_objs { w.write_with_tag(42, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct SupportedObjectResult {
    pub supported_obj_path: String,
    pub access: usp::mod_GetSupportedDMResp::ObjAccessType,
    pub is_multi_instance: bool,
    pub supported_commands: Vec<usp::mod_GetSupportedDMResp::SupportedCommandResult>,
    pub supported_events: Vec<usp::mod_GetSupportedDMResp::SupportedEventResult>,
    pub supported_params: Vec<usp::mod_GetSupportedDMResp::SupportedParamResult>,
    pub divergent_paths: Vec<String>,
    pub unique_key_sets: Vec<usp::mod_GetSupportedDMResp::SupportedUniqueKeySet>,
}

impl<'a> MessageRead<'a> for SupportedObjectResult {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.supported_obj_path = r.read_string(bytes)?.to_owned(),
                Ok(16) => msg.access = r.read_enum(bytes)?,
                Ok(24) => msg.is_multi_instance = r.read_bool(bytes)?,
                Ok(34) => msg.supported_commands.push(r.read_message::<usp::mod_GetSupportedDMResp::SupportedCommandResult>(bytes)?),
                Ok(42) => msg.supported_events.push(r.read_message::<usp::mod_GetSupportedDMResp::SupportedEventResult>(bytes)?),
                Ok(50) => msg.supported_params.push(r.read_message::<usp::mod_GetSupportedDMResp::SupportedParamResult>(bytes)?),
                Ok(58) => msg.divergent_paths.push(r.read_string(bytes)?.to_owned()),
                Ok(66) => msg.unique_key_sets.push(r.read_message::<usp::mod_GetSupportedDMResp::SupportedUniqueKeySet>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for SupportedObjectResult {
    fn get_size(&self) -> usize {
        0
        + if self.supported_obj_path == String::default() { 0 } else { 1 + sizeof_len((&self.supported_obj_path).len()) }
        + if self.access == usp::mod_GetSupportedDMResp::ObjAccessType::OBJ_READ_ONLY { 0 } else { 1 + sizeof_varint(*(&self.access) as u64) }
        + if self.is_multi_instance == false { 0 } else { 1 + sizeof_varint(*(&self.is_multi_instance) as u64) }
        + self.supported_commands.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.supported_events.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.supported_params.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.divergent_paths.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.unique_key_sets.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.supported_obj_path != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.supported_obj_path))?; }
        if self.access != usp::mod_GetSupportedDMResp::ObjAccessType::OBJ_READ_ONLY { w.write_with_tag(16, |w| w.write_enum(*&self.access as i32))?; }
        if self.is_multi_instance != false { w.write_with_tag(24, |w| w.write_bool(*&self.is_multi_instance))?; }
        for s in &self.supported_commands { w.write_with_tag(34, |w| w.write_message(s))?; }
        for s in &self.supported_events { w.write_with_tag(42, |w| w.write_message(s))?; }
        for s in &self.supported_params { w.write_with_tag(50, |w| w.write_message(s))?; }
        for s in &self.divergent_paths { w.write_with_tag(58, |w| w.write_string(&**s))?; }
        for s in &self.unique_key_sets { w.write_with_tag(66, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct SupportedParamResult {
    pub param_name: String,
    pub access: usp::mod_GetSupportedDMResp::ParamAccessType,
    pub value_type: usp::mod_GetSupportedDMResp::ParamValueType,
    pub value_change: usp::mod_GetSupportedDMResp::ValueChangeType,
}

impl<'a> MessageRead<'a> for SupportedParamResult {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.param_name = r.read_string(bytes)?.to_owned(),
                Ok(16) => msg.access = r.read_enum(bytes)?,
                Ok(24) => msg.value_type = r.read_enum(bytes)?,
                Ok(32) => msg.value_change = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for SupportedParamResult {
    fn get_size(&self) -> usize {
        0
        + if self.param_name == String::default() { 0 } else { 1 + sizeof_len((&self.param_name).len()) }
        + if self.access == usp::mod_GetSupportedDMResp::ParamAccessType::PARAM_READ_ONLY { 0 } else { 1 + sizeof_varint(*(&self.access) as u64) }
        + if self.value_type == usp::mod_GetSupportedDMResp::ParamValueType::PARAM_UNKNOWN { 0 } else { 1 + sizeof_varint(*(&self.value_type) as u64) }
        + if self.value_change == usp::mod_GetSupportedDMResp::ValueChangeType::VALUE_CHANGE_UNKNOWN { 0 } else { 1 + sizeof_varint(*(&self.value_change) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.param_name != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.param_name))?; }
        if self.access != usp::mod_GetSupportedDMResp::ParamAccessType::PARAM_READ_ONLY { w.write_with_tag(16, |w| w.write_enum(*&self.access as i32))?; }
        if self.value_type != usp::mod_GetSupportedDMResp::ParamValueType::PARAM_UNKNOWN { w.write_with_tag(24, |w| w.write_enum(*&self.value_type as i32))?; }
        if self.value_change != usp::mod_GetSupportedDMResp::ValueChangeType::VALUE_CHANGE_UNKNOWN { w.write_with_tag(32, |w| w.write_enum(*&self.value_change as i32))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct SupportedCommandResult {
    pub command_name: String,
    pub input_arg_names: Vec<String>,
    pub output_arg_names: Vec<String>,
    pub command_type: usp::mod_GetSupportedDMResp::CmdType,
}

impl<'a> MessageRead<'a> for SupportedCommandResult {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.command_name = r.read_string(bytes)?.to_owned(),
                Ok(18) => msg.input_arg_names.push(r.read_string(bytes)?.to_owned()),
                Ok(26) => msg.output_arg_names.push(r.read_string(bytes)?.to_owned()),
                Ok(32) => msg.command_type = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for SupportedCommandResult {
    fn get_size(&self) -> usize {
        0
        + if self.command_name == String::default() { 0 } else { 1 + sizeof_len((&self.command_name).len()) }
        + self.input_arg_names.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.output_arg_names.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + if self.command_type == usp::mod_GetSupportedDMResp::CmdType::CMD_UNKNOWN { 0 } else { 1 + sizeof_varint(*(&self.command_type) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.command_name != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.command_name))?; }
        for s in &self.input_arg_names { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        for s in &self.output_arg_names { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        if self.command_type != usp::mod_GetSupportedDMResp::CmdType::CMD_UNKNOWN { w.write_with_tag(32, |w| w.write_enum(*&self.command_type as i32))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct SupportedEventResult {
    pub event_name: String,
    pub arg_names: Vec<String>,
}

impl<'a> MessageRead<'a> for SupportedEventResult {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.event_name = r.read_string(bytes)?.to_owned(),
                Ok(18) => msg.arg_names.push(r.read_string(bytes)?.to_owned()),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for SupportedEventResult {
    fn get_size(&self) -> usize {
        0
        + if self.event_name == String::default() { 0 } else { 1 + sizeof_len((&self.event_name).len()) }
        + self.arg_names.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.event_name != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.event_name))?; }
        for s in &self.arg_names { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct SupportedUniqueKeySet {
    pub key_names: Vec<String>,
}

impl<'a> MessageRead<'a> for SupportedUniqueKeySet {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.key_names.push(r.read_string(bytes)?.to_owned()),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for SupportedUniqueKeySet {
    fn get_size(&self) -> usize {
        0
        + self.key_names.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.key_names { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ParamAccessType {
    PARAM_READ_ONLY = 0,
    PARAM_READ_WRITE = 1,
    PARAM_WRITE_ONLY = 2,
}

impl Default for ParamAccessType {
    fn default() -> Self {
        ParamAccessType::PARAM_READ_ONLY
    }
}

impl From<i32> for ParamAccessType {
    fn from(i: i32) -> Self {
        match i {
            0 => ParamAccessType::PARAM_READ_ONLY,
            1 => ParamAccessType::PARAM_READ_WRITE,
            2 => ParamAccessType::PARAM_WRITE_ONLY,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ParamAccessType {
    fn from(s: &'a str) -> Self {
        match s {
            "PARAM_READ_ONLY" => ParamAccessType::PARAM_READ_ONLY,
            "PARAM_READ_WRITE" => ParamAccessType::PARAM_READ_WRITE,
            "PARAM_WRITE_ONLY" => ParamAccessType::PARAM_WRITE_ONLY,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ObjAccessType {
    OBJ_READ_ONLY = 0,
    OBJ_ADD_DELETE = 1,
    OBJ_ADD_ONLY = 2,
    OBJ_DELETE_ONLY = 3,
}

impl Default for ObjAccessType {
    fn default() -> Self {
        ObjAccessType::OBJ_READ_ONLY
    }
}

impl From<i32> for ObjAccessType {
    fn from(i: i32) -> Self {
        match i {
            0 => ObjAccessType::OBJ_READ_ONLY,
            1 => ObjAccessType::OBJ_ADD_DELETE,
            2 => ObjAccessType::OBJ_ADD_ONLY,
            3 => ObjAccessType::OBJ_DELETE_ONLY,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ObjAccessType {
    fn from(s: &'a str) -> Self {
        match s {
            "OBJ_READ_ONLY" => ObjAccessType::OBJ_READ_ONLY,
            "OBJ_ADD_DELETE" => ObjAccessType::OBJ_ADD_DELETE,
            "OBJ_ADD_ONLY" => ObjAccessType::OBJ_ADD_ONLY,
            "OBJ_DELETE_ONLY" => ObjAccessType::OBJ_DELETE_ONLY,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ParamValueType {
    PARAM_UNKNOWN = 0,
    PARAM_BASE_64 = 1,
    PARAM_BOOLEAN = 2,
    PARAM_DATE_TIME = 3,
    PARAM_DECIMAL = 4,
    PARAM_HEX_BINARY = 5,
    PARAM_INT = 6,
    PARAM_LONG = 7,
    PARAM_STRING = 8,
    PARAM_UNSIGNED_INT = 9,
    PARAM_UNSIGNED_LONG = 10,
}

impl Default for ParamValueType {
    fn default() -> Self {
        ParamValueType::PARAM_UNKNOWN
    }
}

impl From<i32> for ParamValueType {
    fn from(i: i32) -> Self {
        match i {
            0 => ParamValueType::PARAM_UNKNOWN,
            1 => ParamValueType::PARAM_BASE_64,
            2 => ParamValueType::PARAM_BOOLEAN,
            3 => ParamValueType::PARAM_DATE_TIME,
            4 => ParamValueType::PARAM_DECIMAL,
            5 => ParamValueType::PARAM_HEX_BINARY,
            6 => ParamValueType::PARAM_INT,
            7 => ParamValueType::PARAM_LONG,
            8 => ParamValueType::PARAM_STRING,
            9 => ParamValueType::PARAM_UNSIGNED_INT,
            10 => ParamValueType::PARAM_UNSIGNED_LONG,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ParamValueType {
    fn from(s: &'a str) -> Self {
        match s {
            "PARAM_UNKNOWN" => ParamValueType::PARAM_UNKNOWN,
            "PARAM_BASE_64" => ParamValueType::PARAM_BASE_64,
            "PARAM_BOOLEAN" => ParamValueType::PARAM_BOOLEAN,
            "PARAM_DATE_TIME" => ParamValueType::PARAM_DATE_TIME,
            "PARAM_DECIMAL" => ParamValueType::PARAM_DECIMAL,
            "PARAM_HEX_BINARY" => ParamValueType::PARAM_HEX_BINARY,
            "PARAM_INT" => ParamValueType::PARAM_INT,
            "PARAM_LONG" => ParamValueType::PARAM_LONG,
            "PARAM_STRING" => ParamValueType::PARAM_STRING,
            "PARAM_UNSIGNED_INT" => ParamValueType::PARAM_UNSIGNED_INT,
            "PARAM_UNSIGNED_LONG" => ParamValueType::PARAM_UNSIGNED_LONG,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ValueChangeType {
    VALUE_CHANGE_UNKNOWN = 0,
    VALUE_CHANGE_ALLOWED = 1,
    VALUE_CHANGE_WILL_IGNORE = 2,
}

impl Default for ValueChangeType {
    fn default() -> Self {
        ValueChangeType::VALUE_CHANGE_UNKNOWN
    }
}

impl From<i32> for ValueChangeType {
    fn from(i: i32) -> Self {
        match i {
            0 => ValueChangeType::VALUE_CHANGE_UNKNOWN,
            1 => ValueChangeType::VALUE_CHANGE_ALLOWED,
            2 => ValueChangeType::VALUE_CHANGE_WILL_IGNORE,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ValueChangeType {
    fn from(s: &'a str) -> Self {
        match s {
            "VALUE_CHANGE_UNKNOWN" => ValueChangeType::VALUE_CHANGE_UNKNOWN,
            "VALUE_CHANGE_ALLOWED" => ValueChangeType::VALUE_CHANGE_ALLOWED,
            "VALUE_CHANGE_WILL_IGNORE" => ValueChangeType::VALUE_CHANGE_WILL_IGNORE,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CmdType {
    CMD_UNKNOWN = 0,
    CMD_SYNC = 1,
    CMD_ASYNC = 2,
}

impl Default for CmdType {
    fn default() -> Self {
        CmdType::CMD_UNKNOWN
    }
}

impl From<i32> for CmdType {
    fn from(i: i32) -> Self {
        match i {
            0 => CmdType::CMD_UNKNOWN,
            1 => CmdType::CMD_SYNC,
            2 => CmdType::CMD_ASYNC,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for CmdType {
    fn from(s: &'a str) -> Self {
        match s {
            "CMD_UNKNOWN" => CmdType::CMD_UNKNOWN,
            "CMD_SYNC" => CmdType::CMD_SYNC,
            "CMD_ASYNC" => CmdType::CMD_ASYNC,
            _ => Self::default(),
        }
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct GetInstances {
    pub obj_paths: Vec<String>,
    pub first_level_only: bool,
}

impl<'a> MessageRead<'a> for GetInstances {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.obj_paths.push(r.read_string(bytes)?.to_owned()),
                Ok(16) => msg.first_level_only = r.read_bool(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for GetInstances {
    fn get_size(&self) -> usize {
        0
        + self.obj_paths.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + if self.first_level_only == false { 0 } else { 1 + sizeof_varint(*(&self.first_level_only) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.obj_paths { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if self.first_level_only != false { w.write_with_tag(16, |w| w.write_bool(*&self.first_level_only))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct GetInstancesResp {
    pub req_path_results: Vec<usp::mod_GetInstancesResp::RequestedPathResult>,
}

impl<'a> MessageRead<'a> for GetInstancesResp {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.req_path_results.push(r.read_message::<usp::mod_GetInstancesResp::RequestedPathResult>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for GetInstancesResp {
    fn get_size(&self) -> usize {
        0
        + self.req_path_results.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.req_path_results { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_GetInstancesResp {

use std::collections::HashMap;
type KVMap<K, V> = HashMap<K, V>;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct RequestedPathResult {
    pub requested_path: String,
    pub err_code: u32,
    pub err_msg: String,
    pub curr_insts: Vec<usp::mod_GetInstancesResp::CurrInstance>,
}

impl<'a> MessageRead<'a> for RequestedPathResult {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.requested_path = r.read_string(bytes)?.to_owned(),
                Ok(21) => msg.err_code = r.read_fixed32(bytes)?,
                Ok(26) => msg.err_msg = r.read_string(bytes)?.to_owned(),
                Ok(34) => msg.curr_insts.push(r.read_message::<usp::mod_GetInstancesResp::CurrInstance>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for RequestedPathResult {
    fn get_size(&self) -> usize {
        0
        + if self.requested_path == String::default() { 0 } else { 1 + sizeof_len((&self.requested_path).len()) }
        + if self.err_code == 0u32 { 0 } else { 1 + 4 }
        + if self.err_msg == String::default() { 0 } else { 1 + sizeof_len((&self.err_msg).len()) }
        + self.curr_insts.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.requested_path != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.requested_path))?; }
        if self.err_code != 0u32 { w.write_with_tag(21, |w| w.write_fixed32(*&self.err_code))?; }
        if self.err_msg != String::default() { w.write_with_tag(26, |w| w.write_string(&**&self.err_msg))?; }
        for s in &self.curr_insts { w.write_with_tag(34, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct CurrInstance {
    pub instantiated_obj_path: String,
    pub unique_keys: KVMap<String, String>,
}

impl<'a> MessageRead<'a> for CurrInstance {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.instantiated_obj_path = r.read_string(bytes)?.to_owned(),
                Ok(18) => {
                    let (key, value) = r.read_map(bytes, |r, bytes| Ok(r.read_string(bytes)?.to_owned()), |r, bytes| Ok(r.read_string(bytes)?.to_owned()))?;
                    msg.unique_keys.insert(key, value);
                }
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for CurrInstance {
    fn get_size(&self) -> usize {
        0
        + if self.instantiated_obj_path == String::default() { 0 } else { 1 + sizeof_len((&self.instantiated_obj_path).len()) }
        + self.unique_keys.iter().map(|(k, v)| 1 + sizeof_len(2 + sizeof_len((k).len()) + sizeof_len((v).len()))).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.instantiated_obj_path != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.instantiated_obj_path))?; }
        for (k, v) in self.unique_keys.iter() { w.write_with_tag(18, |w| w.write_map(2 + sizeof_len((k).len()) + sizeof_len((v).len()), 10, |w| w.write_string(&**k), 18, |w| w.write_string(&**v)))?; }
        Ok(())
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct GetSupportedProtocol {
    pub controller_supported_protocol_versions: String,
}

impl<'a> MessageRead<'a> for GetSupportedProtocol {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.controller_supported_protocol_versions = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for GetSupportedProtocol {
    fn get_size(&self) -> usize {
        0
        + if self.controller_supported_protocol_versions == String::default() { 0 } else { 1 + sizeof_len((&self.controller_supported_protocol_versions).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.controller_supported_protocol_versions != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.controller_supported_protocol_versions))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct GetSupportedProtocolResp {
    pub agent_supported_protocol_versions: String,
}

impl<'a> MessageRead<'a> for GetSupportedProtocolResp {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.agent_supported_protocol_versions = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for GetSupportedProtocolResp {
    fn get_size(&self) -> usize {
        0
        + if self.agent_supported_protocol_versions == String::default() { 0 } else { 1 + sizeof_len((&self.agent_supported_protocol_versions).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.agent_supported_protocol_versions != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.agent_supported_protocol_versions))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Add {
    pub allow_partial: bool,
    pub create_objs: Vec<usp::mod_Add::CreateObject>,
}

impl<'a> MessageRead<'a> for Add {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.allow_partial = r.read_bool(bytes)?,
                Ok(18) => msg.create_objs.push(r.read_message::<usp::mod_Add::CreateObject>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Add {
    fn get_size(&self) -> usize {
        0
        + if self.allow_partial == false { 0 } else { 1 + sizeof_varint(*(&self.allow_partial) as u64) }
        + self.create_objs.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.allow_partial != false { w.write_with_tag(8, |w| w.write_bool(*&self.allow_partial))?; }
        for s in &self.create_objs { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_Add {

use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct CreateObject {
    pub obj_path: String,
    pub param_settings: Vec<usp::mod_Add::CreateParamSetting>,
}

impl<'a> MessageRead<'a> for CreateObject {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.obj_path = r.read_string(bytes)?.to_owned(),
                Ok(18) => msg.param_settings.push(r.read_message::<usp::mod_Add::CreateParamSetting>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for CreateObject {
    fn get_size(&self) -> usize {
        0
        + if self.obj_path == String::default() { 0 } else { 1 + sizeof_len((&self.obj_path).len()) }
        + self.param_settings.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.obj_path != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.obj_path))?; }
        for s in &self.param_settings { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct CreateParamSetting {
    pub param: String,
    pub value: String,
    pub required: bool,
}

impl<'a> MessageRead<'a> for CreateParamSetting {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.param = r.read_string(bytes)?.to_owned(),
                Ok(18) => msg.value = r.read_string(bytes)?.to_owned(),
                Ok(24) => msg.required = r.read_bool(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for CreateParamSetting {
    fn get_size(&self) -> usize {
        0
        + if self.param == String::default() { 0 } else { 1 + sizeof_len((&self.param).len()) }
        + if self.value == String::default() { 0 } else { 1 + sizeof_len((&self.value).len()) }
        + if self.required == false { 0 } else { 1 + sizeof_varint(*(&self.required) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.param != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.param))?; }
        if self.value != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.value))?; }
        if self.required != false { w.write_with_tag(24, |w| w.write_bool(*&self.required))?; }
        Ok(())
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct AddResp {
    pub created_obj_results: Vec<usp::mod_AddResp::CreatedObjectResult>,
}

impl<'a> MessageRead<'a> for AddResp {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.created_obj_results.push(r.read_message::<usp::mod_AddResp::CreatedObjectResult>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for AddResp {
    fn get_size(&self) -> usize {
        0
        + self.created_obj_results.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.created_obj_results { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_AddResp {

use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct CreatedObjectResult {
    pub requested_path: String,
    pub oper_status: Option<usp::mod_AddResp::mod_CreatedObjectResult::OperationStatus>,
}

impl<'a> MessageRead<'a> for CreatedObjectResult {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.requested_path = r.read_string(bytes)?.to_owned(),
                Ok(18) => msg.oper_status = Some(r.read_message::<usp::mod_AddResp::mod_CreatedObjectResult::OperationStatus>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for CreatedObjectResult {
    fn get_size(&self) -> usize {
        0
        + if self.requested_path == String::default() { 0 } else { 1 + sizeof_len((&self.requested_path).len()) }
        + self.oper_status.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.requested_path != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.requested_path))?; }
        if let Some(ref s) = self.oper_status { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_CreatedObjectResult {

use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct OperationStatus {
    pub oper_status: usp::mod_AddResp::mod_CreatedObjectResult::mod_OperationStatus::OneOfoper_status,
}

impl<'a> MessageRead<'a> for OperationStatus {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.oper_status = usp::mod_AddResp::mod_CreatedObjectResult::mod_OperationStatus::OneOfoper_status::oper_failure(r.read_message::<usp::mod_AddResp::mod_CreatedObjectResult::mod_OperationStatus::OperationFailure>(bytes)?),
                Ok(18) => msg.oper_status = usp::mod_AddResp::mod_CreatedObjectResult::mod_OperationStatus::OneOfoper_status::oper_success(r.read_message::<usp::mod_AddResp::mod_CreatedObjectResult::mod_OperationStatus::OperationSuccess>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for OperationStatus {
    fn get_size(&self) -> usize {
        0
        + match self.oper_status {
            usp::mod_AddResp::mod_CreatedObjectResult::mod_OperationStatus::OneOfoper_status::oper_failure(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_AddResp::mod_CreatedObjectResult::mod_OperationStatus::OneOfoper_status::oper_success(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_AddResp::mod_CreatedObjectResult::mod_OperationStatus::OneOfoper_status::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        match self.oper_status {            usp::mod_AddResp::mod_CreatedObjectResult::mod_OperationStatus::OneOfoper_status::oper_failure(ref m) => { w.write_with_tag(10, |w| w.write_message(m))? },
            usp::mod_AddResp::mod_CreatedObjectResult::mod_OperationStatus::OneOfoper_status::oper_success(ref m) => { w.write_with_tag(18, |w| w.write_message(m))? },
            usp::mod_AddResp::mod_CreatedObjectResult::mod_OperationStatus::OneOfoper_status::None => {},
    }        Ok(())
    }
}

pub mod mod_OperationStatus {

use std::collections::HashMap;
type KVMap<K, V> = HashMap<K, V>;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct OperationFailure {
    pub err_code: u32,
    pub err_msg: String,
}

impl<'a> MessageRead<'a> for OperationFailure {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(13) => msg.err_code = r.read_fixed32(bytes)?,
                Ok(18) => msg.err_msg = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for OperationFailure {
    fn get_size(&self) -> usize {
        0
        + if self.err_code == 0u32 { 0 } else { 1 + 4 }
        + if self.err_msg == String::default() { 0 } else { 1 + sizeof_len((&self.err_msg).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.err_code != 0u32 { w.write_with_tag(13, |w| w.write_fixed32(*&self.err_code))?; }
        if self.err_msg != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.err_msg))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct OperationSuccess {
    pub instantiated_path: String,
    pub param_errs: Vec<usp::mod_AddResp::ParameterError>,
    pub unique_keys: KVMap<String, String>,
}

impl<'a> MessageRead<'a> for OperationSuccess {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.instantiated_path = r.read_string(bytes)?.to_owned(),
                Ok(18) => msg.param_errs.push(r.read_message::<usp::mod_AddResp::ParameterError>(bytes)?),
                Ok(26) => {
                    let (key, value) = r.read_map(bytes, |r, bytes| Ok(r.read_string(bytes)?.to_owned()), |r, bytes| Ok(r.read_string(bytes)?.to_owned()))?;
                    msg.unique_keys.insert(key, value);
                }
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for OperationSuccess {
    fn get_size(&self) -> usize {
        0
        + if self.instantiated_path == String::default() { 0 } else { 1 + sizeof_len((&self.instantiated_path).len()) }
        + self.param_errs.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.unique_keys.iter().map(|(k, v)| 1 + sizeof_len(2 + sizeof_len((k).len()) + sizeof_len((v).len()))).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.instantiated_path != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.instantiated_path))?; }
        for s in &self.param_errs { w.write_with_tag(18, |w| w.write_message(s))?; }
        for (k, v) in self.unique_keys.iter() { w.write_with_tag(26, |w| w.write_map(2 + sizeof_len((k).len()) + sizeof_len((v).len()), 10, |w| w.write_string(&**k), 18, |w| w.write_string(&**v)))?; }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfoper_status {
    oper_failure(usp::mod_AddResp::mod_CreatedObjectResult::mod_OperationStatus::OperationFailure),
    oper_success(usp::mod_AddResp::mod_CreatedObjectResult::mod_OperationStatus::OperationSuccess),
    None,
}

impl Default for OneOfoper_status {
    fn default() -> Self {
        OneOfoper_status::None
    }
}

}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ParameterError {
    pub param: String,
    pub err_code: u32,
    pub err_msg: String,
}

impl<'a> MessageRead<'a> for ParameterError {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.param = r.read_string(bytes)?.to_owned(),
                Ok(21) => msg.err_code = r.read_fixed32(bytes)?,
                Ok(26) => msg.err_msg = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ParameterError {
    fn get_size(&self) -> usize {
        0
        + if self.param == String::default() { 0 } else { 1 + sizeof_len((&self.param).len()) }
        + if self.err_code == 0u32 { 0 } else { 1 + 4 }
        + if self.err_msg == String::default() { 0 } else { 1 + sizeof_len((&self.err_msg).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.param != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.param))?; }
        if self.err_code != 0u32 { w.write_with_tag(21, |w| w.write_fixed32(*&self.err_code))?; }
        if self.err_msg != String::default() { w.write_with_tag(26, |w| w.write_string(&**&self.err_msg))?; }
        Ok(())
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Delete {
    pub allow_partial: bool,
    pub obj_paths: Vec<String>,
}

impl<'a> MessageRead<'a> for Delete {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.allow_partial = r.read_bool(bytes)?,
                Ok(18) => msg.obj_paths.push(r.read_string(bytes)?.to_owned()),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Delete {
    fn get_size(&self) -> usize {
        0
        + if self.allow_partial == false { 0 } else { 1 + sizeof_varint(*(&self.allow_partial) as u64) }
        + self.obj_paths.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.allow_partial != false { w.write_with_tag(8, |w| w.write_bool(*&self.allow_partial))?; }
        for s in &self.obj_paths { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct DeleteResp {
    pub deleted_obj_results: Vec<usp::mod_DeleteResp::DeletedObjectResult>,
}

impl<'a> MessageRead<'a> for DeleteResp {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.deleted_obj_results.push(r.read_message::<usp::mod_DeleteResp::DeletedObjectResult>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for DeleteResp {
    fn get_size(&self) -> usize {
        0
        + self.deleted_obj_results.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.deleted_obj_results { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_DeleteResp {

use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct DeletedObjectResult {
    pub requested_path: String,
    pub oper_status: Option<usp::mod_DeleteResp::mod_DeletedObjectResult::OperationStatus>,
}

impl<'a> MessageRead<'a> for DeletedObjectResult {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.requested_path = r.read_string(bytes)?.to_owned(),
                Ok(18) => msg.oper_status = Some(r.read_message::<usp::mod_DeleteResp::mod_DeletedObjectResult::OperationStatus>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for DeletedObjectResult {
    fn get_size(&self) -> usize {
        0
        + if self.requested_path == String::default() { 0 } else { 1 + sizeof_len((&self.requested_path).len()) }
        + self.oper_status.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.requested_path != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.requested_path))?; }
        if let Some(ref s) = self.oper_status { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_DeletedObjectResult {

use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct OperationStatus {
    pub oper_status: usp::mod_DeleteResp::mod_DeletedObjectResult::mod_OperationStatus::OneOfoper_status,
}

impl<'a> MessageRead<'a> for OperationStatus {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.oper_status = usp::mod_DeleteResp::mod_DeletedObjectResult::mod_OperationStatus::OneOfoper_status::oper_failure(r.read_message::<usp::mod_DeleteResp::mod_DeletedObjectResult::mod_OperationStatus::OperationFailure>(bytes)?),
                Ok(18) => msg.oper_status = usp::mod_DeleteResp::mod_DeletedObjectResult::mod_OperationStatus::OneOfoper_status::oper_success(r.read_message::<usp::mod_DeleteResp::mod_DeletedObjectResult::mod_OperationStatus::OperationSuccess>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for OperationStatus {
    fn get_size(&self) -> usize {
        0
        + match self.oper_status {
            usp::mod_DeleteResp::mod_DeletedObjectResult::mod_OperationStatus::OneOfoper_status::oper_failure(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_DeleteResp::mod_DeletedObjectResult::mod_OperationStatus::OneOfoper_status::oper_success(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_DeleteResp::mod_DeletedObjectResult::mod_OperationStatus::OneOfoper_status::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        match self.oper_status {            usp::mod_DeleteResp::mod_DeletedObjectResult::mod_OperationStatus::OneOfoper_status::oper_failure(ref m) => { w.write_with_tag(10, |w| w.write_message(m))? },
            usp::mod_DeleteResp::mod_DeletedObjectResult::mod_OperationStatus::OneOfoper_status::oper_success(ref m) => { w.write_with_tag(18, |w| w.write_message(m))? },
            usp::mod_DeleteResp::mod_DeletedObjectResult::mod_OperationStatus::OneOfoper_status::None => {},
    }        Ok(())
    }
}

pub mod mod_OperationStatus {

use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct OperationFailure {
    pub err_code: u32,
    pub err_msg: String,
}

impl<'a> MessageRead<'a> for OperationFailure {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(13) => msg.err_code = r.read_fixed32(bytes)?,
                Ok(18) => msg.err_msg = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for OperationFailure {
    fn get_size(&self) -> usize {
        0
        + if self.err_code == 0u32 { 0 } else { 1 + 4 }
        + if self.err_msg == String::default() { 0 } else { 1 + sizeof_len((&self.err_msg).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.err_code != 0u32 { w.write_with_tag(13, |w| w.write_fixed32(*&self.err_code))?; }
        if self.err_msg != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.err_msg))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct OperationSuccess {
    pub affected_paths: Vec<String>,
    pub unaffected_path_errs: Vec<usp::mod_DeleteResp::UnaffectedPathError>,
}

impl<'a> MessageRead<'a> for OperationSuccess {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.affected_paths.push(r.read_string(bytes)?.to_owned()),
                Ok(18) => msg.unaffected_path_errs.push(r.read_message::<usp::mod_DeleteResp::UnaffectedPathError>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for OperationSuccess {
    fn get_size(&self) -> usize {
        0
        + self.affected_paths.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.unaffected_path_errs.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.affected_paths { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        for s in &self.unaffected_path_errs { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfoper_status {
    oper_failure(usp::mod_DeleteResp::mod_DeletedObjectResult::mod_OperationStatus::OperationFailure),
    oper_success(usp::mod_DeleteResp::mod_DeletedObjectResult::mod_OperationStatus::OperationSuccess),
    None,
}

impl Default for OneOfoper_status {
    fn default() -> Self {
        OneOfoper_status::None
    }
}

}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct UnaffectedPathError {
    pub unaffected_path: String,
    pub err_code: u32,
    pub err_msg: String,
}

impl<'a> MessageRead<'a> for UnaffectedPathError {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.unaffected_path = r.read_string(bytes)?.to_owned(),
                Ok(21) => msg.err_code = r.read_fixed32(bytes)?,
                Ok(26) => msg.err_msg = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for UnaffectedPathError {
    fn get_size(&self) -> usize {
        0
        + if self.unaffected_path == String::default() { 0 } else { 1 + sizeof_len((&self.unaffected_path).len()) }
        + if self.err_code == 0u32 { 0 } else { 1 + 4 }
        + if self.err_msg == String::default() { 0 } else { 1 + sizeof_len((&self.err_msg).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.unaffected_path != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.unaffected_path))?; }
        if self.err_code != 0u32 { w.write_with_tag(21, |w| w.write_fixed32(*&self.err_code))?; }
        if self.err_msg != String::default() { w.write_with_tag(26, |w| w.write_string(&**&self.err_msg))?; }
        Ok(())
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Set {
    pub allow_partial: bool,
    pub update_objs: Vec<usp::mod_Set::UpdateObject>,
}

impl<'a> MessageRead<'a> for Set {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.allow_partial = r.read_bool(bytes)?,
                Ok(18) => msg.update_objs.push(r.read_message::<usp::mod_Set::UpdateObject>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Set {
    fn get_size(&self) -> usize {
        0
        + if self.allow_partial == false { 0 } else { 1 + sizeof_varint(*(&self.allow_partial) as u64) }
        + self.update_objs.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.allow_partial != false { w.write_with_tag(8, |w| w.write_bool(*&self.allow_partial))?; }
        for s in &self.update_objs { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_Set {

use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct UpdateObject {
    pub obj_path: String,
    pub param_settings: Vec<usp::mod_Set::UpdateParamSetting>,
}

impl<'a> MessageRead<'a> for UpdateObject {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.obj_path = r.read_string(bytes)?.to_owned(),
                Ok(18) => msg.param_settings.push(r.read_message::<usp::mod_Set::UpdateParamSetting>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for UpdateObject {
    fn get_size(&self) -> usize {
        0
        + if self.obj_path == String::default() { 0 } else { 1 + sizeof_len((&self.obj_path).len()) }
        + self.param_settings.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.obj_path != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.obj_path))?; }
        for s in &self.param_settings { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct UpdateParamSetting {
    pub param: String,
    pub value: String,
    pub required: bool,
}

impl<'a> MessageRead<'a> for UpdateParamSetting {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.param = r.read_string(bytes)?.to_owned(),
                Ok(18) => msg.value = r.read_string(bytes)?.to_owned(),
                Ok(24) => msg.required = r.read_bool(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for UpdateParamSetting {
    fn get_size(&self) -> usize {
        0
        + if self.param == String::default() { 0 } else { 1 + sizeof_len((&self.param).len()) }
        + if self.value == String::default() { 0 } else { 1 + sizeof_len((&self.value).len()) }
        + if self.required == false { 0 } else { 1 + sizeof_varint(*(&self.required) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.param != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.param))?; }
        if self.value != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.value))?; }
        if self.required != false { w.write_with_tag(24, |w| w.write_bool(*&self.required))?; }
        Ok(())
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct SetResp {
    pub updated_obj_results: Vec<usp::mod_SetResp::UpdatedObjectResult>,
}

impl<'a> MessageRead<'a> for SetResp {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.updated_obj_results.push(r.read_message::<usp::mod_SetResp::UpdatedObjectResult>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for SetResp {
    fn get_size(&self) -> usize {
        0
        + self.updated_obj_results.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.updated_obj_results { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_SetResp {

use std::collections::HashMap;
type KVMap<K, V> = HashMap<K, V>;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct UpdatedObjectResult {
    pub requested_path: String,
    pub oper_status: Option<usp::mod_SetResp::mod_UpdatedObjectResult::OperationStatus>,
}

impl<'a> MessageRead<'a> for UpdatedObjectResult {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.requested_path = r.read_string(bytes)?.to_owned(),
                Ok(18) => msg.oper_status = Some(r.read_message::<usp::mod_SetResp::mod_UpdatedObjectResult::OperationStatus>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for UpdatedObjectResult {
    fn get_size(&self) -> usize {
        0
        + if self.requested_path == String::default() { 0 } else { 1 + sizeof_len((&self.requested_path).len()) }
        + self.oper_status.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.requested_path != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.requested_path))?; }
        if let Some(ref s) = self.oper_status { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_UpdatedObjectResult {

use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct OperationStatus {
    pub oper_status: usp::mod_SetResp::mod_UpdatedObjectResult::mod_OperationStatus::OneOfoper_status,
}

impl<'a> MessageRead<'a> for OperationStatus {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.oper_status = usp::mod_SetResp::mod_UpdatedObjectResult::mod_OperationStatus::OneOfoper_status::oper_failure(r.read_message::<usp::mod_SetResp::mod_UpdatedObjectResult::mod_OperationStatus::OperationFailure>(bytes)?),
                Ok(18) => msg.oper_status = usp::mod_SetResp::mod_UpdatedObjectResult::mod_OperationStatus::OneOfoper_status::oper_success(r.read_message::<usp::mod_SetResp::mod_UpdatedObjectResult::mod_OperationStatus::OperationSuccess>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for OperationStatus {
    fn get_size(&self) -> usize {
        0
        + match self.oper_status {
            usp::mod_SetResp::mod_UpdatedObjectResult::mod_OperationStatus::OneOfoper_status::oper_failure(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_SetResp::mod_UpdatedObjectResult::mod_OperationStatus::OneOfoper_status::oper_success(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_SetResp::mod_UpdatedObjectResult::mod_OperationStatus::OneOfoper_status::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        match self.oper_status {            usp::mod_SetResp::mod_UpdatedObjectResult::mod_OperationStatus::OneOfoper_status::oper_failure(ref m) => { w.write_with_tag(10, |w| w.write_message(m))? },
            usp::mod_SetResp::mod_UpdatedObjectResult::mod_OperationStatus::OneOfoper_status::oper_success(ref m) => { w.write_with_tag(18, |w| w.write_message(m))? },
            usp::mod_SetResp::mod_UpdatedObjectResult::mod_OperationStatus::OneOfoper_status::None => {},
    }        Ok(())
    }
}

pub mod mod_OperationStatus {

use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct OperationFailure {
    pub err_code: u32,
    pub err_msg: String,
    pub updated_inst_failures: Vec<usp::mod_SetResp::UpdatedInstanceFailure>,
}

impl<'a> MessageRead<'a> for OperationFailure {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(13) => msg.err_code = r.read_fixed32(bytes)?,
                Ok(18) => msg.err_msg = r.read_string(bytes)?.to_owned(),
                Ok(26) => msg.updated_inst_failures.push(r.read_message::<usp::mod_SetResp::UpdatedInstanceFailure>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for OperationFailure {
    fn get_size(&self) -> usize {
        0
        + if self.err_code == 0u32 { 0 } else { 1 + 4 }
        + if self.err_msg == String::default() { 0 } else { 1 + sizeof_len((&self.err_msg).len()) }
        + self.updated_inst_failures.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.err_code != 0u32 { w.write_with_tag(13, |w| w.write_fixed32(*&self.err_code))?; }
        if self.err_msg != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.err_msg))?; }
        for s in &self.updated_inst_failures { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct OperationSuccess {
    pub updated_inst_results: Vec<usp::mod_SetResp::UpdatedInstanceResult>,
}

impl<'a> MessageRead<'a> for OperationSuccess {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.updated_inst_results.push(r.read_message::<usp::mod_SetResp::UpdatedInstanceResult>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for OperationSuccess {
    fn get_size(&self) -> usize {
        0
        + self.updated_inst_results.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.updated_inst_results { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfoper_status {
    oper_failure(usp::mod_SetResp::mod_UpdatedObjectResult::mod_OperationStatus::OperationFailure),
    oper_success(usp::mod_SetResp::mod_UpdatedObjectResult::mod_OperationStatus::OperationSuccess),
    None,
}

impl Default for OneOfoper_status {
    fn default() -> Self {
        OneOfoper_status::None
    }
}

}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct UpdatedInstanceFailure {
    pub affected_path: String,
    pub param_errs: Vec<usp::mod_SetResp::ParameterError>,
}

impl<'a> MessageRead<'a> for UpdatedInstanceFailure {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.affected_path = r.read_string(bytes)?.to_owned(),
                Ok(18) => msg.param_errs.push(r.read_message::<usp::mod_SetResp::ParameterError>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for UpdatedInstanceFailure {
    fn get_size(&self) -> usize {
        0
        + if self.affected_path == String::default() { 0 } else { 1 + sizeof_len((&self.affected_path).len()) }
        + self.param_errs.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.affected_path != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.affected_path))?; }
        for s in &self.param_errs { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct UpdatedInstanceResult {
    pub affected_path: String,
    pub param_errs: Vec<usp::mod_SetResp::ParameterError>,
    pub updated_params: KVMap<String, String>,
}

impl<'a> MessageRead<'a> for UpdatedInstanceResult {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.affected_path = r.read_string(bytes)?.to_owned(),
                Ok(18) => msg.param_errs.push(r.read_message::<usp::mod_SetResp::ParameterError>(bytes)?),
                Ok(26) => {
                    let (key, value) = r.read_map(bytes, |r, bytes| Ok(r.read_string(bytes)?.to_owned()), |r, bytes| Ok(r.read_string(bytes)?.to_owned()))?;
                    msg.updated_params.insert(key, value);
                }
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for UpdatedInstanceResult {
    fn get_size(&self) -> usize {
        0
        + if self.affected_path == String::default() { 0 } else { 1 + sizeof_len((&self.affected_path).len()) }
        + self.param_errs.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.updated_params.iter().map(|(k, v)| 1 + sizeof_len(2 + sizeof_len((k).len()) + sizeof_len((v).len()))).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.affected_path != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.affected_path))?; }
        for s in &self.param_errs { w.write_with_tag(18, |w| w.write_message(s))?; }
        for (k, v) in self.updated_params.iter() { w.write_with_tag(26, |w| w.write_map(2 + sizeof_len((k).len()) + sizeof_len((v).len()), 10, |w| w.write_string(&**k), 18, |w| w.write_string(&**v)))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ParameterError {
    pub param: String,
    pub err_code: u32,
    pub err_msg: String,
}

impl<'a> MessageRead<'a> for ParameterError {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.param = r.read_string(bytes)?.to_owned(),
                Ok(21) => msg.err_code = r.read_fixed32(bytes)?,
                Ok(26) => msg.err_msg = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ParameterError {
    fn get_size(&self) -> usize {
        0
        + if self.param == String::default() { 0 } else { 1 + sizeof_len((&self.param).len()) }
        + if self.err_code == 0u32 { 0 } else { 1 + 4 }
        + if self.err_msg == String::default() { 0 } else { 1 + sizeof_len((&self.err_msg).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.param != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.param))?; }
        if self.err_code != 0u32 { w.write_with_tag(21, |w| w.write_fixed32(*&self.err_code))?; }
        if self.err_msg != String::default() { w.write_with_tag(26, |w| w.write_string(&**&self.err_msg))?; }
        Ok(())
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Operate {
    pub command: String,
    pub command_key: String,
    pub send_resp: bool,
    pub input_args: KVMap<String, String>,
}

impl<'a> MessageRead<'a> for Operate {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.command = r.read_string(bytes)?.to_owned(),
                Ok(18) => msg.command_key = r.read_string(bytes)?.to_owned(),
                Ok(24) => msg.send_resp = r.read_bool(bytes)?,
                Ok(34) => {
                    let (key, value) = r.read_map(bytes, |r, bytes| Ok(r.read_string(bytes)?.to_owned()), |r, bytes| Ok(r.read_string(bytes)?.to_owned()))?;
                    msg.input_args.insert(key, value);
                }
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Operate {
    fn get_size(&self) -> usize {
        0
        + if self.command == String::default() { 0 } else { 1 + sizeof_len((&self.command).len()) }
        + if self.command_key == String::default() { 0 } else { 1 + sizeof_len((&self.command_key).len()) }
        + if self.send_resp == false { 0 } else { 1 + sizeof_varint(*(&self.send_resp) as u64) }
        + self.input_args.iter().map(|(k, v)| 1 + sizeof_len(2 + sizeof_len((k).len()) + sizeof_len((v).len()))).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.command != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.command))?; }
        if self.command_key != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.command_key))?; }
        if self.send_resp != false { w.write_with_tag(24, |w| w.write_bool(*&self.send_resp))?; }
        for (k, v) in self.input_args.iter() { w.write_with_tag(34, |w| w.write_map(2 + sizeof_len((k).len()) + sizeof_len((v).len()), 10, |w| w.write_string(&**k), 18, |w| w.write_string(&**v)))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct OperateResp {
    pub operation_results: Vec<usp::mod_OperateResp::OperationResult>,
}

impl<'a> MessageRead<'a> for OperateResp {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.operation_results.push(r.read_message::<usp::mod_OperateResp::OperationResult>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for OperateResp {
    fn get_size(&self) -> usize {
        0
        + self.operation_results.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.operation_results { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_OperateResp {

use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct OperationResult {
    pub executed_command: String,
    pub operation_resp: usp::mod_OperateResp::mod_OperationResult::OneOfoperation_resp,
}

impl<'a> MessageRead<'a> for OperationResult {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.executed_command = r.read_string(bytes)?.to_owned(),
                Ok(18) => msg.operation_resp = usp::mod_OperateResp::mod_OperationResult::OneOfoperation_resp::req_obj_path(r.read_string(bytes)?.to_owned()),
                Ok(26) => msg.operation_resp = usp::mod_OperateResp::mod_OperationResult::OneOfoperation_resp::req_output_args(r.read_message::<usp::mod_OperateResp::mod_OperationResult::OutputArgs>(bytes)?),
                Ok(34) => msg.operation_resp = usp::mod_OperateResp::mod_OperationResult::OneOfoperation_resp::cmd_failure(r.read_message::<usp::mod_OperateResp::mod_OperationResult::CommandFailure>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for OperationResult {
    fn get_size(&self) -> usize {
        0
        + if self.executed_command == String::default() { 0 } else { 1 + sizeof_len((&self.executed_command).len()) }
        + match self.operation_resp {
            usp::mod_OperateResp::mod_OperationResult::OneOfoperation_resp::req_obj_path(ref m) => 1 + sizeof_len((m).len()),
            usp::mod_OperateResp::mod_OperationResult::OneOfoperation_resp::req_output_args(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_OperateResp::mod_OperationResult::OneOfoperation_resp::cmd_failure(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_OperateResp::mod_OperationResult::OneOfoperation_resp::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.executed_command != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.executed_command))?; }
        match self.operation_resp {            usp::mod_OperateResp::mod_OperationResult::OneOfoperation_resp::req_obj_path(ref m) => { w.write_with_tag(18, |w| w.write_string(&**m))? },
            usp::mod_OperateResp::mod_OperationResult::OneOfoperation_resp::req_output_args(ref m) => { w.write_with_tag(26, |w| w.write_message(m))? },
            usp::mod_OperateResp::mod_OperationResult::OneOfoperation_resp::cmd_failure(ref m) => { w.write_with_tag(34, |w| w.write_message(m))? },
            usp::mod_OperateResp::mod_OperationResult::OneOfoperation_resp::None => {},
    }        Ok(())
    }
}

pub mod mod_OperationResult {

use std::collections::HashMap;
type KVMap<K, V> = HashMap<K, V>;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct OutputArgs {
    pub output_args: KVMap<String, String>,
}

impl<'a> MessageRead<'a> for OutputArgs {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => {
                    let (key, value) = r.read_map(bytes, |r, bytes| Ok(r.read_string(bytes)?.to_owned()), |r, bytes| Ok(r.read_string(bytes)?.to_owned()))?;
                    msg.output_args.insert(key, value);
                }
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for OutputArgs {
    fn get_size(&self) -> usize {
        0
        + self.output_args.iter().map(|(k, v)| 1 + sizeof_len(2 + sizeof_len((k).len()) + sizeof_len((v).len()))).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for (k, v) in self.output_args.iter() { w.write_with_tag(10, |w| w.write_map(2 + sizeof_len((k).len()) + sizeof_len((v).len()), 10, |w| w.write_string(&**k), 18, |w| w.write_string(&**v)))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct CommandFailure {
    pub err_code: u32,
    pub err_msg: String,
}

impl<'a> MessageRead<'a> for CommandFailure {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(13) => msg.err_code = r.read_fixed32(bytes)?,
                Ok(18) => msg.err_msg = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for CommandFailure {
    fn get_size(&self) -> usize {
        0
        + if self.err_code == 0u32 { 0 } else { 1 + 4 }
        + if self.err_msg == String::default() { 0 } else { 1 + sizeof_len((&self.err_msg).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.err_code != 0u32 { w.write_with_tag(13, |w| w.write_fixed32(*&self.err_code))?; }
        if self.err_msg != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.err_msg))?; }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfoperation_resp {
    req_obj_path(String),
    req_output_args(usp::mod_OperateResp::mod_OperationResult::OutputArgs),
    cmd_failure(usp::mod_OperateResp::mod_OperationResult::CommandFailure),
    None,
}

impl Default for OneOfoperation_resp {
    fn default() -> Self {
        OneOfoperation_resp::None
    }
}

}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Notify {
    pub subscription_id: String,
    pub send_resp: bool,
    pub notification: usp::mod_Notify::OneOfnotification,
}

impl<'a> MessageRead<'a> for Notify {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.subscription_id = r.read_string(bytes)?.to_owned(),
                Ok(16) => msg.send_resp = r.read_bool(bytes)?,
                Ok(26) => msg.notification = usp::mod_Notify::OneOfnotification::event(r.read_message::<usp::mod_Notify::Event>(bytes)?),
                Ok(34) => msg.notification = usp::mod_Notify::OneOfnotification::value_change(r.read_message::<usp::mod_Notify::ValueChange>(bytes)?),
                Ok(42) => msg.notification = usp::mod_Notify::OneOfnotification::obj_creation(r.read_message::<usp::mod_Notify::ObjectCreation>(bytes)?),
                Ok(50) => msg.notification = usp::mod_Notify::OneOfnotification::obj_deletion(r.read_message::<usp::mod_Notify::ObjectDeletion>(bytes)?),
                Ok(58) => msg.notification = usp::mod_Notify::OneOfnotification::oper_complete(r.read_message::<usp::mod_Notify::OperationComplete>(bytes)?),
                Ok(66) => msg.notification = usp::mod_Notify::OneOfnotification::on_board_req(r.read_message::<usp::mod_Notify::OnBoardRequest>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Notify {
    fn get_size(&self) -> usize {
        0
        + if self.subscription_id == String::default() { 0 } else { 1 + sizeof_len((&self.subscription_id).len()) }
        + if self.send_resp == false { 0 } else { 1 + sizeof_varint(*(&self.send_resp) as u64) }
        + match self.notification {
            usp::mod_Notify::OneOfnotification::event(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_Notify::OneOfnotification::value_change(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_Notify::OneOfnotification::obj_creation(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_Notify::OneOfnotification::obj_deletion(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_Notify::OneOfnotification::oper_complete(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_Notify::OneOfnotification::on_board_req(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_Notify::OneOfnotification::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.subscription_id != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.subscription_id))?; }
        if self.send_resp != false { w.write_with_tag(16, |w| w.write_bool(*&self.send_resp))?; }
        match self.notification {            usp::mod_Notify::OneOfnotification::event(ref m) => { w.write_with_tag(26, |w| w.write_message(m))? },
            usp::mod_Notify::OneOfnotification::value_change(ref m) => { w.write_with_tag(34, |w| w.write_message(m))? },
            usp::mod_Notify::OneOfnotification::obj_creation(ref m) => { w.write_with_tag(42, |w| w.write_message(m))? },
            usp::mod_Notify::OneOfnotification::obj_deletion(ref m) => { w.write_with_tag(50, |w| w.write_message(m))? },
            usp::mod_Notify::OneOfnotification::oper_complete(ref m) => { w.write_with_tag(58, |w| w.write_message(m))? },
            usp::mod_Notify::OneOfnotification::on_board_req(ref m) => { w.write_with_tag(66, |w| w.write_message(m))? },
            usp::mod_Notify::OneOfnotification::None => {},
    }        Ok(())
    }
}

pub mod mod_Notify {

use std::collections::HashMap;
type KVMap<K, V> = HashMap<K, V>;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Event {
    pub obj_path: String,
    pub event_name: String,
    pub params: KVMap<String, String>,
}

impl<'a> MessageRead<'a> for Event {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.obj_path = r.read_string(bytes)?.to_owned(),
                Ok(18) => msg.event_name = r.read_string(bytes)?.to_owned(),
                Ok(26) => {
                    let (key, value) = r.read_map(bytes, |r, bytes| Ok(r.read_string(bytes)?.to_owned()), |r, bytes| Ok(r.read_string(bytes)?.to_owned()))?;
                    msg.params.insert(key, value);
                }
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Event {
    fn get_size(&self) -> usize {
        0
        + if self.obj_path == String::default() { 0 } else { 1 + sizeof_len((&self.obj_path).len()) }
        + if self.event_name == String::default() { 0 } else { 1 + sizeof_len((&self.event_name).len()) }
        + self.params.iter().map(|(k, v)| 1 + sizeof_len(2 + sizeof_len((k).len()) + sizeof_len((v).len()))).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.obj_path != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.obj_path))?; }
        if self.event_name != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.event_name))?; }
        for (k, v) in self.params.iter() { w.write_with_tag(26, |w| w.write_map(2 + sizeof_len((k).len()) + sizeof_len((v).len()), 10, |w| w.write_string(&**k), 18, |w| w.write_string(&**v)))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ValueChange {
    pub param_path: String,
    pub param_value: String,
}

impl<'a> MessageRead<'a> for ValueChange {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.param_path = r.read_string(bytes)?.to_owned(),
                Ok(18) => msg.param_value = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ValueChange {
    fn get_size(&self) -> usize {
        0
        + if self.param_path == String::default() { 0 } else { 1 + sizeof_len((&self.param_path).len()) }
        + if self.param_value == String::default() { 0 } else { 1 + sizeof_len((&self.param_value).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.param_path != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.param_path))?; }
        if self.param_value != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.param_value))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ObjectCreation {
    pub obj_path: String,
    pub unique_keys: KVMap<String, String>,
}

impl<'a> MessageRead<'a> for ObjectCreation {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.obj_path = r.read_string(bytes)?.to_owned(),
                Ok(18) => {
                    let (key, value) = r.read_map(bytes, |r, bytes| Ok(r.read_string(bytes)?.to_owned()), |r, bytes| Ok(r.read_string(bytes)?.to_owned()))?;
                    msg.unique_keys.insert(key, value);
                }
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ObjectCreation {
    fn get_size(&self) -> usize {
        0
        + if self.obj_path == String::default() { 0 } else { 1 + sizeof_len((&self.obj_path).len()) }
        + self.unique_keys.iter().map(|(k, v)| 1 + sizeof_len(2 + sizeof_len((k).len()) + sizeof_len((v).len()))).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.obj_path != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.obj_path))?; }
        for (k, v) in self.unique_keys.iter() { w.write_with_tag(18, |w| w.write_map(2 + sizeof_len((k).len()) + sizeof_len((v).len()), 10, |w| w.write_string(&**k), 18, |w| w.write_string(&**v)))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ObjectDeletion {
    pub obj_path: String,
}

impl<'a> MessageRead<'a> for ObjectDeletion {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.obj_path = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ObjectDeletion {
    fn get_size(&self) -> usize {
        0
        + if self.obj_path == String::default() { 0 } else { 1 + sizeof_len((&self.obj_path).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.obj_path != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.obj_path))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct OperationComplete {
    pub obj_path: String,
    pub command_name: String,
    pub command_key: String,
    pub operation_resp: usp::mod_Notify::mod_OperationComplete::OneOfoperation_resp,
}

impl<'a> MessageRead<'a> for OperationComplete {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.obj_path = r.read_string(bytes)?.to_owned(),
                Ok(18) => msg.command_name = r.read_string(bytes)?.to_owned(),
                Ok(26) => msg.command_key = r.read_string(bytes)?.to_owned(),
                Ok(34) => msg.operation_resp = usp::mod_Notify::mod_OperationComplete::OneOfoperation_resp::req_output_args(r.read_message::<usp::mod_Notify::mod_OperationComplete::OutputArgs>(bytes)?),
                Ok(42) => msg.operation_resp = usp::mod_Notify::mod_OperationComplete::OneOfoperation_resp::cmd_failure(r.read_message::<usp::mod_Notify::mod_OperationComplete::CommandFailure>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for OperationComplete {
    fn get_size(&self) -> usize {
        0
        + if self.obj_path == String::default() { 0 } else { 1 + sizeof_len((&self.obj_path).len()) }
        + if self.command_name == String::default() { 0 } else { 1 + sizeof_len((&self.command_name).len()) }
        + if self.command_key == String::default() { 0 } else { 1 + sizeof_len((&self.command_key).len()) }
        + match self.operation_resp {
            usp::mod_Notify::mod_OperationComplete::OneOfoperation_resp::req_output_args(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_Notify::mod_OperationComplete::OneOfoperation_resp::cmd_failure(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_Notify::mod_OperationComplete::OneOfoperation_resp::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.obj_path != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.obj_path))?; }
        if self.command_name != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.command_name))?; }
        if self.command_key != String::default() { w.write_with_tag(26, |w| w.write_string(&**&self.command_key))?; }
        match self.operation_resp {            usp::mod_Notify::mod_OperationComplete::OneOfoperation_resp::req_output_args(ref m) => { w.write_with_tag(34, |w| w.write_message(m))? },
            usp::mod_Notify::mod_OperationComplete::OneOfoperation_resp::cmd_failure(ref m) => { w.write_with_tag(42, |w| w.write_message(m))? },
            usp::mod_Notify::mod_OperationComplete::OneOfoperation_resp::None => {},
    }        Ok(())
    }
}

pub mod mod_OperationComplete {

use std::collections::HashMap;
type KVMap<K, V> = HashMap<K, V>;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct OutputArgs {
    pub output_args: KVMap<String, String>,
}

impl<'a> MessageRead<'a> for OutputArgs {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => {
                    let (key, value) = r.read_map(bytes, |r, bytes| Ok(r.read_string(bytes)?.to_owned()), |r, bytes| Ok(r.read_string(bytes)?.to_owned()))?;
                    msg.output_args.insert(key, value);
                }
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for OutputArgs {
    fn get_size(&self) -> usize {
        0
        + self.output_args.iter().map(|(k, v)| 1 + sizeof_len(2 + sizeof_len((k).len()) + sizeof_len((v).len()))).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for (k, v) in self.output_args.iter() { w.write_with_tag(10, |w| w.write_map(2 + sizeof_len((k).len()) + sizeof_len((v).len()), 10, |w| w.write_string(&**k), 18, |w| w.write_string(&**v)))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct CommandFailure {
    pub err_code: u32,
    pub err_msg: String,
}

impl<'a> MessageRead<'a> for CommandFailure {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(13) => msg.err_code = r.read_fixed32(bytes)?,
                Ok(18) => msg.err_msg = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for CommandFailure {
    fn get_size(&self) -> usize {
        0
        + if self.err_code == 0u32 { 0 } else { 1 + 4 }
        + if self.err_msg == String::default() { 0 } else { 1 + sizeof_len((&self.err_msg).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.err_code != 0u32 { w.write_with_tag(13, |w| w.write_fixed32(*&self.err_code))?; }
        if self.err_msg != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.err_msg))?; }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfoperation_resp {
    req_output_args(usp::mod_Notify::mod_OperationComplete::OutputArgs),
    cmd_failure(usp::mod_Notify::mod_OperationComplete::CommandFailure),
    None,
}

impl Default for OneOfoperation_resp {
    fn default() -> Self {
        OneOfoperation_resp::None
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct OnBoardRequest {
    pub oui: String,
    pub product_class: String,
    pub serial_number: String,
    pub agent_supported_protocol_versions: String,
}

impl<'a> MessageRead<'a> for OnBoardRequest {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.oui = r.read_string(bytes)?.to_owned(),
                Ok(18) => msg.product_class = r.read_string(bytes)?.to_owned(),
                Ok(26) => msg.serial_number = r.read_string(bytes)?.to_owned(),
                Ok(34) => msg.agent_supported_protocol_versions = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for OnBoardRequest {
    fn get_size(&self) -> usize {
        0
        + if self.oui == String::default() { 0 } else { 1 + sizeof_len((&self.oui).len()) }
        + if self.product_class == String::default() { 0 } else { 1 + sizeof_len((&self.product_class).len()) }
        + if self.serial_number == String::default() { 0 } else { 1 + sizeof_len((&self.serial_number).len()) }
        + if self.agent_supported_protocol_versions == String::default() { 0 } else { 1 + sizeof_len((&self.agent_supported_protocol_versions).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.oui != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.oui))?; }
        if self.product_class != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.product_class))?; }
        if self.serial_number != String::default() { w.write_with_tag(26, |w| w.write_string(&**&self.serial_number))?; }
        if self.agent_supported_protocol_versions != String::default() { w.write_with_tag(34, |w| w.write_string(&**&self.agent_supported_protocol_versions))?; }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfnotification {
    event(usp::mod_Notify::Event),
    value_change(usp::mod_Notify::ValueChange),
    obj_creation(usp::mod_Notify::ObjectCreation),
    obj_deletion(usp::mod_Notify::ObjectDeletion),
    oper_complete(usp::mod_Notify::OperationComplete),
    on_board_req(usp::mod_Notify::OnBoardRequest),
    None,
}

impl Default for OneOfnotification {
    fn default() -> Self {
        OneOfnotification::None
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct NotifyResp {
    pub subscription_id: String,
}

impl<'a> MessageRead<'a> for NotifyResp {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.subscription_id = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for NotifyResp {
    fn get_size(&self) -> usize {
        0
        + if self.subscription_id == String::default() { 0 } else { 1 + sizeof_len((&self.subscription_id).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.subscription_id != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.subscription_id))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Register {
    pub allow_partial: bool,
    pub reg_paths: Vec<usp::mod_Register::RegistrationPath>,
}

impl<'a> MessageRead<'a> for Register {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.allow_partial = r.read_bool(bytes)?,
                Ok(18) => msg.reg_paths.push(r.read_message::<usp::mod_Register::RegistrationPath>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Register {
    fn get_size(&self) -> usize {
        0
        + if self.allow_partial == false { 0 } else { 1 + sizeof_varint(*(&self.allow_partial) as u64) }
        + self.reg_paths.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.allow_partial != false { w.write_with_tag(8, |w| w.write_bool(*&self.allow_partial))?; }
        for s in &self.reg_paths { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_Register {

use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct RegistrationPath {
    pub path: String,
}

impl<'a> MessageRead<'a> for RegistrationPath {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.path = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for RegistrationPath {
    fn get_size(&self) -> usize {
        0
        + if self.path == String::default() { 0 } else { 1 + sizeof_len((&self.path).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.path != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.path))?; }
        Ok(())
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct RegisterResp {
    pub registered_path_results: Vec<usp::mod_RegisterResp::RegisteredPathResult>,
}

impl<'a> MessageRead<'a> for RegisterResp {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.registered_path_results.push(r.read_message::<usp::mod_RegisterResp::RegisteredPathResult>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for RegisterResp {
    fn get_size(&self) -> usize {
        0
        + self.registered_path_results.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.registered_path_results { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_RegisterResp {

use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct RegisteredPathResult {
    pub requested_path: String,
    pub oper_status: Option<usp::mod_RegisterResp::mod_RegisteredPathResult::OperationStatus>,
}

impl<'a> MessageRead<'a> for RegisteredPathResult {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.requested_path = r.read_string(bytes)?.to_owned(),
                Ok(18) => msg.oper_status = Some(r.read_message::<usp::mod_RegisterResp::mod_RegisteredPathResult::OperationStatus>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for RegisteredPathResult {
    fn get_size(&self) -> usize {
        0
        + if self.requested_path == String::default() { 0 } else { 1 + sizeof_len((&self.requested_path).len()) }
        + self.oper_status.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.requested_path != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.requested_path))?; }
        if let Some(ref s) = self.oper_status { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_RegisteredPathResult {

use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct OperationStatus {
    pub oper_status: usp::mod_RegisterResp::mod_RegisteredPathResult::mod_OperationStatus::OneOfoper_status,
}

impl<'a> MessageRead<'a> for OperationStatus {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.oper_status = usp::mod_RegisterResp::mod_RegisteredPathResult::mod_OperationStatus::OneOfoper_status::oper_failure(r.read_message::<usp::mod_RegisterResp::mod_RegisteredPathResult::mod_OperationStatus::OperationFailure>(bytes)?),
                Ok(18) => msg.oper_status = usp::mod_RegisterResp::mod_RegisteredPathResult::mod_OperationStatus::OneOfoper_status::oper_success(r.read_message::<usp::mod_RegisterResp::mod_RegisteredPathResult::mod_OperationStatus::OperationSuccess>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for OperationStatus {
    fn get_size(&self) -> usize {
        0
        + match self.oper_status {
            usp::mod_RegisterResp::mod_RegisteredPathResult::mod_OperationStatus::OneOfoper_status::oper_failure(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_RegisterResp::mod_RegisteredPathResult::mod_OperationStatus::OneOfoper_status::oper_success(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_RegisterResp::mod_RegisteredPathResult::mod_OperationStatus::OneOfoper_status::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        match self.oper_status {            usp::mod_RegisterResp::mod_RegisteredPathResult::mod_OperationStatus::OneOfoper_status::oper_failure(ref m) => { w.write_with_tag(10, |w| w.write_message(m))? },
            usp::mod_RegisterResp::mod_RegisteredPathResult::mod_OperationStatus::OneOfoper_status::oper_success(ref m) => { w.write_with_tag(18, |w| w.write_message(m))? },
            usp::mod_RegisterResp::mod_RegisteredPathResult::mod_OperationStatus::OneOfoper_status::None => {},
    }        Ok(())
    }
}

pub mod mod_OperationStatus {

use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct OperationFailure {
    pub err_code: u32,
    pub err_msg: String,
}

impl<'a> MessageRead<'a> for OperationFailure {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(13) => msg.err_code = r.read_fixed32(bytes)?,
                Ok(18) => msg.err_msg = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for OperationFailure {
    fn get_size(&self) -> usize {
        0
        + if self.err_code == 0u32 { 0 } else { 1 + 4 }
        + if self.err_msg == String::default() { 0 } else { 1 + sizeof_len((&self.err_msg).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.err_code != 0u32 { w.write_with_tag(13, |w| w.write_fixed32(*&self.err_code))?; }
        if self.err_msg != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.err_msg))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct OperationSuccess {
    pub registered_path: String,
}

impl<'a> MessageRead<'a> for OperationSuccess {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.registered_path = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for OperationSuccess {
    fn get_size(&self) -> usize {
        0
        + if self.registered_path == String::default() { 0 } else { 1 + sizeof_len((&self.registered_path).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.registered_path != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.registered_path))?; }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfoper_status {
    oper_failure(usp::mod_RegisterResp::mod_RegisteredPathResult::mod_OperationStatus::OperationFailure),
    oper_success(usp::mod_RegisterResp::mod_RegisteredPathResult::mod_OperationStatus::OperationSuccess),
    None,
}

impl Default for OneOfoper_status {
    fn default() -> Self {
        OneOfoper_status::None
    }
}

}

}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Deregister {
    pub paths: Vec<String>,
}

impl<'a> MessageRead<'a> for Deregister {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.paths.push(r.read_string(bytes)?.to_owned()),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Deregister {
    fn get_size(&self) -> usize {
        0
        + self.paths.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.paths { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct DeregisterResp {
    pub deregistered_path_results: Vec<usp::mod_DeregisterResp::DeregisteredPathResult>,
}

impl<'a> MessageRead<'a> for DeregisterResp {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.deregistered_path_results.push(r.read_message::<usp::mod_DeregisterResp::DeregisteredPathResult>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for DeregisterResp {
    fn get_size(&self) -> usize {
        0
        + self.deregistered_path_results.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.deregistered_path_results { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_DeregisterResp {

use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct DeregisteredPathResult {
    pub requested_path: String,
    pub oper_status: Option<usp::mod_DeregisterResp::mod_DeregisteredPathResult::OperationStatus>,
}

impl<'a> MessageRead<'a> for DeregisteredPathResult {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.requested_path = r.read_string(bytes)?.to_owned(),
                Ok(18) => msg.oper_status = Some(r.read_message::<usp::mod_DeregisterResp::mod_DeregisteredPathResult::OperationStatus>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for DeregisteredPathResult {
    fn get_size(&self) -> usize {
        0
        + if self.requested_path == String::default() { 0 } else { 1 + sizeof_len((&self.requested_path).len()) }
        + self.oper_status.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.requested_path != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.requested_path))?; }
        if let Some(ref s) = self.oper_status { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_DeregisteredPathResult {

use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct OperationStatus {
    pub oper_status: usp::mod_DeregisterResp::mod_DeregisteredPathResult::mod_OperationStatus::OneOfoper_status,
}

impl<'a> MessageRead<'a> for OperationStatus {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.oper_status = usp::mod_DeregisterResp::mod_DeregisteredPathResult::mod_OperationStatus::OneOfoper_status::oper_failure(r.read_message::<usp::mod_DeregisterResp::mod_DeregisteredPathResult::mod_OperationStatus::OperationFailure>(bytes)?),
                Ok(18) => msg.oper_status = usp::mod_DeregisterResp::mod_DeregisteredPathResult::mod_OperationStatus::OneOfoper_status::oper_success(r.read_message::<usp::mod_DeregisterResp::mod_DeregisteredPathResult::mod_OperationStatus::OperationSuccess>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for OperationStatus {
    fn get_size(&self) -> usize {
        0
        + match self.oper_status {
            usp::mod_DeregisterResp::mod_DeregisteredPathResult::mod_OperationStatus::OneOfoper_status::oper_failure(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_DeregisterResp::mod_DeregisteredPathResult::mod_OperationStatus::OneOfoper_status::oper_success(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_DeregisterResp::mod_DeregisteredPathResult::mod_OperationStatus::OneOfoper_status::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        match self.oper_status {            usp::mod_DeregisterResp::mod_DeregisteredPathResult::mod_OperationStatus::OneOfoper_status::oper_failure(ref m) => { w.write_with_tag(10, |w| w.write_message(m))? },
            usp::mod_DeregisterResp::mod_DeregisteredPathResult::mod_OperationStatus::OneOfoper_status::oper_success(ref m) => { w.write_with_tag(18, |w| w.write_message(m))? },
            usp::mod_DeregisterResp::mod_DeregisteredPathResult::mod_OperationStatus::OneOfoper_status::None => {},
    }        Ok(())
    }
}

pub mod mod_OperationStatus {

use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct OperationFailure {
    pub err_code: u32,
    pub err_msg: String,
}

impl<'a> MessageRead<'a> for OperationFailure {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(13) => msg.err_code = r.read_fixed32(bytes)?,
                Ok(18) => msg.err_msg = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for OperationFailure {
    fn get_size(&self) -> usize {
        0
        + if self.err_code == 0u32 { 0 } else { 1 + 4 }
        + if self.err_msg == String::default() { 0 } else { 1 + sizeof_len((&self.err_msg).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.err_code != 0u32 { w.write_with_tag(13, |w| w.write_fixed32(*&self.err_code))?; }
        if self.err_msg != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.err_msg))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct OperationSuccess {
    pub deregistered_path: Vec<String>,
}

impl<'a> MessageRead<'a> for OperationSuccess {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.deregistered_path.push(r.read_string(bytes)?.to_owned()),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for OperationSuccess {
    fn get_size(&self) -> usize {
        0
        + self.deregistered_path.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.deregistered_path { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfoper_status {
    oper_failure(usp::mod_DeregisterResp::mod_DeregisteredPathResult::mod_OperationStatus::OperationFailure),
    oper_success(usp::mod_DeregisterResp::mod_DeregisteredPathResult::mod_OperationStatus::OperationSuccess),
    None,
}

impl Default for OneOfoper_status {
    fn default() -> Self {
        OneOfoper_status::None
    }
}

}

}

}

