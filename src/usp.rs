#![allow(nonstandard_style)]

use super::*;
use quick_protobuf::sizeofs::*;
use quick_protobuf::{BytesReader, MessageRead, MessageWrite, Result, Writer};
use std::borrow::Cow;
use std::collections::HashMap;
use std::io::Write;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Msg<'a> {
    pub header: Option<usp::Header<'a>>,
    pub body: Option<usp::Body<'a>>,
}

impl<'a> MessageRead<'a> for Msg<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.header = Some(r.read_message::<usp::Header>(bytes)?),
                Ok(18) => msg.body = Some(r.read_message::<usp::Body>(bytes)?),
                Ok(t) => {
                    r.read_unknown(bytes, t)?;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Msg<'a> {
    fn get_size(&self) -> usize {
        0 + self
            .header
            .as_ref()
            .map_or(0, |m| 1 + sizeof_len((m).get_size()))
            + self
                .body
                .as_ref()
                .map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.header {
            w.write_with_tag(10, |w| w.write_message(s))?;
        }
        if let Some(ref s) = self.body {
            w.write_with_tag(18, |w| w.write_message(s))?;
        }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Header<'a> {
    pub msg_id: Option<Cow<'a, str>>,
    pub msg_type: Option<usp::mod_Header::MsgType>,
}

impl<'a> MessageRead<'a> for Header<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.msg_id = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(16) => msg.msg_type = Some(r.read_enum(bytes)?),
                Ok(t) => {
                    r.read_unknown(bytes, t)?;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Header<'a> {
    fn get_size(&self) -> usize {
        0 + self
            .msg_id
            .as_ref()
            .map_or(0, |m| 1 + sizeof_len((m).len()))
            + self
                .msg_type
                .as_ref()
                .map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.msg_id {
            w.write_with_tag(10, |w| w.write_string(&**s))?;
        }
        if let Some(ref s) = self.msg_type {
            w.write_with_tag(16, |w| w.write_enum(*s as i32))?;
        }
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
                _ => Self::default(),
            }
        }
    }

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Body<'a> {
    pub msg_body: usp::mod_Body::OneOfmsg_body<'a>,
}

impl<'a> MessageRead<'a> for Body<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => {
                    msg.msg_body = usp::mod_Body::OneOfmsg_body::request(
                        r.read_message::<usp::Request>(bytes)?,
                    )
                }
                Ok(18) => {
                    msg.msg_body = usp::mod_Body::OneOfmsg_body::response(
                        r.read_message::<usp::Response>(bytes)?,
                    )
                }
                Ok(26) => {
                    msg.msg_body =
                        usp::mod_Body::OneOfmsg_body::error(r.read_message::<usp::Error>(bytes)?)
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

impl<'a> MessageWrite for Body<'a> {
    fn get_size(&self) -> usize {
        0 + match self.msg_body {
            usp::mod_Body::OneOfmsg_body::request(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_Body::OneOfmsg_body::response(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_Body::OneOfmsg_body::error(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_Body::OneOfmsg_body::None => 0,
        }
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        match self.msg_body {
            usp::mod_Body::OneOfmsg_body::request(ref m) => {
                w.write_with_tag(10, |w| w.write_message(m))?
            }
            usp::mod_Body::OneOfmsg_body::response(ref m) => {
                w.write_with_tag(18, |w| w.write_message(m))?
            }
            usp::mod_Body::OneOfmsg_body::error(ref m) => {
                w.write_with_tag(26, |w| w.write_message(m))?
            }
            usp::mod_Body::OneOfmsg_body::None => {}
        }
        Ok(())
    }
}

pub mod mod_Body {

    use super::*;

    #[derive(Debug, PartialEq, Clone)]
    pub enum OneOfmsg_body<'a> {
        request(usp::Request<'a>),
        response(usp::Response<'a>),
        error(usp::Error<'a>),
        None,
    }

    impl<'a> Default for OneOfmsg_body<'a> {
        fn default() -> Self {
            OneOfmsg_body::None
        }
    }

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Request<'a> {
    pub req_type: usp::mod_Request::OneOfreq_type<'a>,
}

impl<'a> MessageRead<'a> for Request<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => {
                    msg.req_type =
                        usp::mod_Request::OneOfreq_type::get(r.read_message::<usp::Get>(bytes)?)
                }
                Ok(18) => {
                    msg.req_type = usp::mod_Request::OneOfreq_type::get_supported_dm(
                        r.read_message::<usp::GetSupportedDM>(bytes)?,
                    )
                }
                Ok(26) => {
                    msg.req_type = usp::mod_Request::OneOfreq_type::get_instances(
                        r.read_message::<usp::GetInstances>(bytes)?,
                    )
                }
                Ok(34) => {
                    msg.req_type =
                        usp::mod_Request::OneOfreq_type::set(r.read_message::<usp::Set>(bytes)?)
                }
                Ok(42) => {
                    msg.req_type =
                        usp::mod_Request::OneOfreq_type::add(r.read_message::<usp::Add>(bytes)?)
                }
                Ok(50) => {
                    msg.req_type = usp::mod_Request::OneOfreq_type::delete(
                        r.read_message::<usp::Delete>(bytes)?,
                    )
                }
                Ok(58) => {
                    msg.req_type = usp::mod_Request::OneOfreq_type::operate(
                        r.read_message::<usp::Operate>(bytes)?,
                    )
                }
                Ok(66) => {
                    msg.req_type = usp::mod_Request::OneOfreq_type::notify(
                        r.read_message::<usp::Notify>(bytes)?,
                    )
                }
                Ok(74) => {
                    msg.req_type = usp::mod_Request::OneOfreq_type::get_supported_protocol(
                        r.read_message::<usp::GetSupportedProtocol>(bytes)?,
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

impl<'a> MessageWrite for Request<'a> {
    fn get_size(&self) -> usize {
        0 + match self.req_type {
            usp::mod_Request::OneOfreq_type::get(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_Request::OneOfreq_type::get_supported_dm(ref m) => {
                1 + sizeof_len((m).get_size())
            }
            usp::mod_Request::OneOfreq_type::get_instances(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_Request::OneOfreq_type::set(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_Request::OneOfreq_type::add(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_Request::OneOfreq_type::delete(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_Request::OneOfreq_type::operate(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_Request::OneOfreq_type::notify(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_Request::OneOfreq_type::get_supported_protocol(ref m) => {
                1 + sizeof_len((m).get_size())
            }
            usp::mod_Request::OneOfreq_type::None => 0,
        }
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        match self.req_type {
            usp::mod_Request::OneOfreq_type::get(ref m) => {
                w.write_with_tag(10, |w| w.write_message(m))?
            }
            usp::mod_Request::OneOfreq_type::get_supported_dm(ref m) => {
                w.write_with_tag(18, |w| w.write_message(m))?
            }
            usp::mod_Request::OneOfreq_type::get_instances(ref m) => {
                w.write_with_tag(26, |w| w.write_message(m))?
            }
            usp::mod_Request::OneOfreq_type::set(ref m) => {
                w.write_with_tag(34, |w| w.write_message(m))?
            }
            usp::mod_Request::OneOfreq_type::add(ref m) => {
                w.write_with_tag(42, |w| w.write_message(m))?
            }
            usp::mod_Request::OneOfreq_type::delete(ref m) => {
                w.write_with_tag(50, |w| w.write_message(m))?
            }
            usp::mod_Request::OneOfreq_type::operate(ref m) => {
                w.write_with_tag(58, |w| w.write_message(m))?
            }
            usp::mod_Request::OneOfreq_type::notify(ref m) => {
                w.write_with_tag(66, |w| w.write_message(m))?
            }
            usp::mod_Request::OneOfreq_type::get_supported_protocol(ref m) => {
                w.write_with_tag(74, |w| w.write_message(m))?
            }
            usp::mod_Request::OneOfreq_type::None => {}
        }
        Ok(())
    }
}

pub mod mod_Request {

    use super::*;

    #[derive(Debug, PartialEq, Clone)]
    pub enum OneOfreq_type<'a> {
        get(usp::Get<'a>),
        get_supported_dm(usp::GetSupportedDM<'a>),
        get_instances(usp::GetInstances<'a>),
        set(usp::Set<'a>),
        add(usp::Add<'a>),
        delete(usp::Delete<'a>),
        operate(usp::Operate<'a>),
        notify(usp::Notify<'a>),
        get_supported_protocol(usp::GetSupportedProtocol<'a>),
        None,
    }

    impl<'a> Default for OneOfreq_type<'a> {
        fn default() -> Self {
            OneOfreq_type::None
        }
    }

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Response<'a> {
    pub resp_type: usp::mod_Response::OneOfresp_type<'a>,
}

impl<'a> MessageRead<'a> for Response<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => {
                    msg.resp_type = usp::mod_Response::OneOfresp_type::get_resp(
                        r.read_message::<usp::GetResp>(bytes)?,
                    )
                }
                Ok(18) => {
                    msg.resp_type = usp::mod_Response::OneOfresp_type::get_supported_dm_resp(
                        r.read_message::<usp::GetSupportedDMResp>(bytes)?,
                    )
                }
                Ok(26) => {
                    msg.resp_type = usp::mod_Response::OneOfresp_type::get_instances_resp(
                        r.read_message::<usp::GetInstancesResp>(bytes)?,
                    )
                }
                Ok(34) => {
                    msg.resp_type = usp::mod_Response::OneOfresp_type::set_resp(
                        r.read_message::<usp::SetResp>(bytes)?,
                    )
                }
                Ok(42) => {
                    msg.resp_type = usp::mod_Response::OneOfresp_type::add_resp(
                        r.read_message::<usp::AddResp>(bytes)?,
                    )
                }
                Ok(50) => {
                    msg.resp_type = usp::mod_Response::OneOfresp_type::delete_resp(
                        r.read_message::<usp::DeleteResp>(bytes)?,
                    )
                }
                Ok(58) => {
                    msg.resp_type = usp::mod_Response::OneOfresp_type::operate_resp(
                        r.read_message::<usp::OperateResp>(bytes)?,
                    )
                }
                Ok(66) => {
                    msg.resp_type = usp::mod_Response::OneOfresp_type::notify_resp(
                        r.read_message::<usp::NotifyResp>(bytes)?,
                    )
                }
                Ok(74) => {
                    msg.resp_type = usp::mod_Response::OneOfresp_type::get_supported_protocol_resp(
                        r.read_message::<usp::GetSupportedProtocolResp>(bytes)?,
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

impl<'a> MessageWrite for Response<'a> {
    fn get_size(&self) -> usize {
        0 + match self.resp_type {
            usp::mod_Response::OneOfresp_type::get_resp(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_Response::OneOfresp_type::get_supported_dm_resp(ref m) => {
                1 + sizeof_len((m).get_size())
            }
            usp::mod_Response::OneOfresp_type::get_instances_resp(ref m) => {
                1 + sizeof_len((m).get_size())
            }
            usp::mod_Response::OneOfresp_type::set_resp(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_Response::OneOfresp_type::add_resp(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_Response::OneOfresp_type::delete_resp(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_Response::OneOfresp_type::operate_resp(ref m) => {
                1 + sizeof_len((m).get_size())
            }
            usp::mod_Response::OneOfresp_type::notify_resp(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_Response::OneOfresp_type::get_supported_protocol_resp(ref m) => {
                1 + sizeof_len((m).get_size())
            }
            usp::mod_Response::OneOfresp_type::None => 0,
        }
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        match self.resp_type {
            usp::mod_Response::OneOfresp_type::get_resp(ref m) => {
                w.write_with_tag(10, |w| w.write_message(m))?
            }
            usp::mod_Response::OneOfresp_type::get_supported_dm_resp(ref m) => {
                w.write_with_tag(18, |w| w.write_message(m))?
            }
            usp::mod_Response::OneOfresp_type::get_instances_resp(ref m) => {
                w.write_with_tag(26, |w| w.write_message(m))?
            }
            usp::mod_Response::OneOfresp_type::set_resp(ref m) => {
                w.write_with_tag(34, |w| w.write_message(m))?
            }
            usp::mod_Response::OneOfresp_type::add_resp(ref m) => {
                w.write_with_tag(42, |w| w.write_message(m))?
            }
            usp::mod_Response::OneOfresp_type::delete_resp(ref m) => {
                w.write_with_tag(50, |w| w.write_message(m))?
            }
            usp::mod_Response::OneOfresp_type::operate_resp(ref m) => {
                w.write_with_tag(58, |w| w.write_message(m))?
            }
            usp::mod_Response::OneOfresp_type::notify_resp(ref m) => {
                w.write_with_tag(66, |w| w.write_message(m))?
            }
            usp::mod_Response::OneOfresp_type::get_supported_protocol_resp(ref m) => {
                w.write_with_tag(74, |w| w.write_message(m))?
            }
            usp::mod_Response::OneOfresp_type::None => {}
        }
        Ok(())
    }
}

pub mod mod_Response {

    use super::*;

    #[derive(Debug, PartialEq, Clone)]
    pub enum OneOfresp_type<'a> {
        get_resp(usp::GetResp<'a>),
        get_supported_dm_resp(usp::GetSupportedDMResp<'a>),
        get_instances_resp(usp::GetInstancesResp<'a>),
        set_resp(usp::SetResp<'a>),
        add_resp(usp::AddResp<'a>),
        delete_resp(usp::DeleteResp<'a>),
        operate_resp(usp::OperateResp<'a>),
        notify_resp(usp::NotifyResp<'a>),
        get_supported_protocol_resp(usp::GetSupportedProtocolResp<'a>),
        None,
    }

    impl<'a> Default for OneOfresp_type<'a> {
        fn default() -> Self {
            OneOfresp_type::None
        }
    }

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Error<'a> {
    pub err_code: Option<u32>,
    pub err_msg: Option<Cow<'a, str>>,
    pub param_errs: Vec<usp::mod_Error::ParamError<'a>>,
}

impl<'a> MessageRead<'a> for Error<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(13) => msg.err_code = Some(r.read_fixed32(bytes)?),
                Ok(18) => msg.err_msg = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg
                    .param_errs
                    .push(r.read_message::<usp::mod_Error::ParamError>(bytes)?),
                Ok(t) => {
                    r.read_unknown(bytes, t)?;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Error<'a> {
    fn get_size(&self) -> usize {
        0 + self.err_code.as_ref().map_or(0, |_| 1 + 4)
            + self
                .err_msg
                .as_ref()
                .map_or(0, |m| 1 + sizeof_len((m).len()))
            + self
                .param_errs
                .iter()
                .map(|s| 1 + sizeof_len((s).get_size()))
                .sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.err_code {
            w.write_with_tag(13, |w| w.write_fixed32(*s))?;
        }
        if let Some(ref s) = self.err_msg {
            w.write_with_tag(18, |w| w.write_string(&**s))?;
        }
        for s in &self.param_errs {
            w.write_with_tag(26, |w| w.write_message(s))?;
        }
        Ok(())
    }
}

pub mod mod_Error {

    use super::*;
    use std::borrow::Cow;

    #[derive(Debug, Default, PartialEq, Clone)]
    pub struct ParamError<'a> {
        pub param_path: Option<Cow<'a, str>>,
        pub err_code: Option<u32>,
        pub err_msg: Option<Cow<'a, str>>,
    }

    impl<'a> MessageRead<'a> for ParamError<'a> {
        fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
            let mut msg = Self::default();
            while !r.is_eof() {
                match r.next_tag(bytes) {
                    Ok(10) => msg.param_path = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                    Ok(21) => msg.err_code = Some(r.read_fixed32(bytes)?),
                    Ok(26) => msg.err_msg = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                    Ok(t) => {
                        r.read_unknown(bytes, t)?;
                    }
                    Err(e) => return Err(e),
                }
            }
            Ok(msg)
        }
    }

    impl<'a> MessageWrite for ParamError<'a> {
        fn get_size(&self) -> usize {
            0 + self
                .param_path
                .as_ref()
                .map_or(0, |m| 1 + sizeof_len((m).len()))
                + self.err_code.as_ref().map_or(0, |_| 1 + 4)
                + self
                    .err_msg
                    .as_ref()
                    .map_or(0, |m| 1 + sizeof_len((m).len()))
        }

        fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
            if let Some(ref s) = self.param_path {
                w.write_with_tag(10, |w| w.write_string(&**s))?;
            }
            if let Some(ref s) = self.err_code {
                w.write_with_tag(21, |w| w.write_fixed32(*s))?;
            }
            if let Some(ref s) = self.err_msg {
                w.write_with_tag(26, |w| w.write_string(&**s))?;
            }
            Ok(())
        }
    }

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Get<'a> {
    pub param_paths: Vec<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for Get<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg
                    .param_paths
                    .push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => {
                    r.read_unknown(bytes, t)?;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Get<'a> {
    fn get_size(&self) -> usize {
        0 + self
            .param_paths
            .iter()
            .map(|s| 1 + sizeof_len((s).len()))
            .sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.param_paths {
            w.write_with_tag(10, |w| w.write_string(&**s))?;
        }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GetResp<'a> {
    pub req_path_results: Vec<usp::mod_GetResp::RequestedPathResult<'a>>,
}

impl<'a> MessageRead<'a> for GetResp<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg
                    .req_path_results
                    .push(r.read_message::<usp::mod_GetResp::RequestedPathResult>(bytes)?),
                Ok(t) => {
                    r.read_unknown(bytes, t)?;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for GetResp<'a> {
    fn get_size(&self) -> usize {
        0 + self
            .req_path_results
            .iter()
            .map(|s| 1 + sizeof_len((s).get_size()))
            .sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.req_path_results {
            w.write_with_tag(10, |w| w.write_message(s))?;
        }
        Ok(())
    }
}

pub mod mod_GetResp {

    use super::*;
    use std::borrow::Cow;
    use std::collections::HashMap;

    #[derive(Debug, Default, PartialEq, Clone)]
    pub struct RequestedPathResult<'a> {
        pub requested_path: Option<Cow<'a, str>>,
        pub err_code: Option<u32>,
        pub err_msg: Option<Cow<'a, str>>,
        pub resolved_path_results: Vec<usp::mod_GetResp::ResolvedPathResult<'a>>,
    }

    impl<'a> MessageRead<'a> for RequestedPathResult<'a> {
        fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
            let mut msg = Self::default();
            while !r.is_eof() {
                match r.next_tag(bytes) {
                    Ok(10) => msg.requested_path = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                    Ok(21) => msg.err_code = Some(r.read_fixed32(bytes)?),
                    Ok(26) => msg.err_msg = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                    Ok(34) => msg
                        .resolved_path_results
                        .push(r.read_message::<usp::mod_GetResp::ResolvedPathResult>(bytes)?),
                    Ok(t) => {
                        r.read_unknown(bytes, t)?;
                    }
                    Err(e) => return Err(e),
                }
            }
            Ok(msg)
        }
    }

    impl<'a> MessageWrite for RequestedPathResult<'a> {
        fn get_size(&self) -> usize {
            0 + self
                .requested_path
                .as_ref()
                .map_or(0, |m| 1 + sizeof_len((m).len()))
                + self.err_code.as_ref().map_or(0, |_| 1 + 4)
                + self
                    .err_msg
                    .as_ref()
                    .map_or(0, |m| 1 + sizeof_len((m).len()))
                + self
                    .resolved_path_results
                    .iter()
                    .map(|s| 1 + sizeof_len((s).get_size()))
                    .sum::<usize>()
        }

        fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
            if let Some(ref s) = self.requested_path {
                w.write_with_tag(10, |w| w.write_string(&**s))?;
            }
            if let Some(ref s) = self.err_code {
                w.write_with_tag(21, |w| w.write_fixed32(*s))?;
            }
            if let Some(ref s) = self.err_msg {
                w.write_with_tag(26, |w| w.write_string(&**s))?;
            }
            for s in &self.resolved_path_results {
                w.write_with_tag(34, |w| w.write_message(s))?;
            }
            Ok(())
        }
    }

    #[derive(Debug, Default, PartialEq, Clone)]
    pub struct ResolvedPathResult<'a> {
        pub resolved_path: Option<Cow<'a, str>>,
        pub result_params: HashMap<Cow<'a, str>, Cow<'a, str>>,
    }

    impl<'a> MessageRead<'a> for ResolvedPathResult<'a> {
        fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
            let mut msg = Self::default();
            while !r.is_eof() {
                match r.next_tag(bytes) {
                    Ok(10) => msg.resolved_path = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                    Ok(18) => {
                        let (key, value) = r.read_map(
                            bytes,
                            |r, bytes| r.read_string(bytes).map(Cow::Borrowed),
                            |r, bytes| r.read_string(bytes).map(Cow::Borrowed),
                        )?;
                        msg.result_params.insert(key, value);
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

    impl<'a> MessageWrite for ResolvedPathResult<'a> {
        fn get_size(&self) -> usize {
            0 + self
                .resolved_path
                .as_ref()
                .map_or(0, |m| 1 + sizeof_len((m).len()))
                + self
                    .result_params
                    .iter()
                    .map(|(k, v)| 1 + sizeof_len(2 + sizeof_len((k).len()) + sizeof_len((v).len())))
                    .sum::<usize>()
        }

        fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
            if let Some(ref s) = self.resolved_path {
                w.write_with_tag(10, |w| w.write_string(&**s))?;
            }
            for (k, v) in self.result_params.iter() {
                w.write_with_tag(18, |w| {
                    w.write_map(
                        2 + sizeof_len((k).len()) + sizeof_len((v).len()),
                        10,
                        |w| w.write_string(&**k),
                        18,
                        |w| w.write_string(&**v),
                    )
                })?;
            }
            Ok(())
        }
    }

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GetSupportedDM<'a> {
    pub obj_paths: Vec<Cow<'a, str>>,
    pub first_level_only: Option<bool>,
    pub return_commands: Option<bool>,
    pub return_events: Option<bool>,
    pub return_params: Option<bool>,
}

impl<'a> MessageRead<'a> for GetSupportedDM<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.obj_paths.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(16) => msg.first_level_only = Some(r.read_bool(bytes)?),
                Ok(24) => msg.return_commands = Some(r.read_bool(bytes)?),
                Ok(32) => msg.return_events = Some(r.read_bool(bytes)?),
                Ok(40) => msg.return_params = Some(r.read_bool(bytes)?),
                Ok(t) => {
                    r.read_unknown(bytes, t)?;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for GetSupportedDM<'a> {
    fn get_size(&self) -> usize {
        0 + self
            .obj_paths
            .iter()
            .map(|s| 1 + sizeof_len((s).len()))
            .sum::<usize>()
            + self
                .first_level_only
                .as_ref()
                .map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
            + self
                .return_commands
                .as_ref()
                .map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
            + self
                .return_events
                .as_ref()
                .map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
            + self
                .return_params
                .as_ref()
                .map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.obj_paths {
            w.write_with_tag(10, |w| w.write_string(&**s))?;
        }
        if let Some(ref s) = self.first_level_only {
            w.write_with_tag(16, |w| w.write_bool(*s))?;
        }
        if let Some(ref s) = self.return_commands {
            w.write_with_tag(24, |w| w.write_bool(*s))?;
        }
        if let Some(ref s) = self.return_events {
            w.write_with_tag(32, |w| w.write_bool(*s))?;
        }
        if let Some(ref s) = self.return_params {
            w.write_with_tag(40, |w| w.write_bool(*s))?;
        }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GetSupportedDMResp<'a> {
    pub req_obj_results: Vec<usp::mod_GetSupportedDMResp::RequestedObjectResult<'a>>,
}

impl<'a> MessageRead<'a> for GetSupportedDMResp<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.req_obj_results.push(
                    r.read_message::<usp::mod_GetSupportedDMResp::RequestedObjectResult>(bytes)?,
                ),
                Ok(t) => {
                    r.read_unknown(bytes, t)?;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for GetSupportedDMResp<'a> {
    fn get_size(&self) -> usize {
        0 + self
            .req_obj_results
            .iter()
            .map(|s| 1 + sizeof_len((s).get_size()))
            .sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.req_obj_results {
            w.write_with_tag(10, |w| w.write_message(s))?;
        }
        Ok(())
    }
}

pub mod mod_GetSupportedDMResp {

    use super::*;
    use std::borrow::Cow;

    #[derive(Debug, Default, PartialEq, Clone)]
    pub struct RequestedObjectResult<'a> {
        pub req_obj_path: Option<Cow<'a, str>>,
        pub err_code: Option<u32>,
        pub err_msg: Option<Cow<'a, str>>,
        pub data_model_inst_uri: Option<Cow<'a, str>>,
        pub supported_objs: Vec<usp::mod_GetSupportedDMResp::SupportedObjectResult<'a>>,
    }

    impl<'a> MessageRead<'a> for RequestedObjectResult<'a> {
        fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
            let mut msg = Self::default();
            while !r.is_eof() {
                match r.next_tag(bytes) {
                    Ok(10) => msg.req_obj_path = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                    Ok(21) => msg.err_code = Some(r.read_fixed32(bytes)?),
                    Ok(26) => msg.err_msg = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                    Ok(34) => {
                        msg.data_model_inst_uri = Some(r.read_string(bytes).map(Cow::Borrowed)?)
                    }
                    Ok(42) => msg.supported_objs.push(
                        r.read_message::<usp::mod_GetSupportedDMResp::SupportedObjectResult>(
                            bytes,
                        )?,
                    ),
                    Ok(t) => {
                        r.read_unknown(bytes, t)?;
                    }
                    Err(e) => return Err(e),
                }
            }
            Ok(msg)
        }
    }

    impl<'a> MessageWrite for RequestedObjectResult<'a> {
        fn get_size(&self) -> usize {
            0 + self
                .req_obj_path
                .as_ref()
                .map_or(0, |m| 1 + sizeof_len((m).len()))
                + self.err_code.as_ref().map_or(0, |_| 1 + 4)
                + self
                    .err_msg
                    .as_ref()
                    .map_or(0, |m| 1 + sizeof_len((m).len()))
                + self
                    .data_model_inst_uri
                    .as_ref()
                    .map_or(0, |m| 1 + sizeof_len((m).len()))
                + self
                    .supported_objs
                    .iter()
                    .map(|s| 1 + sizeof_len((s).get_size()))
                    .sum::<usize>()
        }

        fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
            if let Some(ref s) = self.req_obj_path {
                w.write_with_tag(10, |w| w.write_string(&**s))?;
            }
            if let Some(ref s) = self.err_code {
                w.write_with_tag(21, |w| w.write_fixed32(*s))?;
            }
            if let Some(ref s) = self.err_msg {
                w.write_with_tag(26, |w| w.write_string(&**s))?;
            }
            if let Some(ref s) = self.data_model_inst_uri {
                w.write_with_tag(34, |w| w.write_string(&**s))?;
            }
            for s in &self.supported_objs {
                w.write_with_tag(42, |w| w.write_message(s))?;
            }
            Ok(())
        }
    }

    #[derive(Debug, Default, PartialEq, Clone)]
    pub struct SupportedObjectResult<'a> {
        pub supported_obj_path: Option<Cow<'a, str>>,
        pub access: Option<usp::mod_GetSupportedDMResp::ObjAccessType>,
        pub is_multi_instance: Option<bool>,
        pub supported_commands: Vec<usp::mod_GetSupportedDMResp::SupportedCommandResult<'a>>,
        pub supported_events: Vec<usp::mod_GetSupportedDMResp::SupportedEventResult<'a>>,
        pub supported_params: Vec<usp::mod_GetSupportedDMResp::SupportedParamResult<'a>>,
    }

    impl<'a> MessageRead<'a> for SupportedObjectResult<'a> {
        fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
            let mut msg = Self::default();
            while !r.is_eof() {
                match r.next_tag(bytes) {
                    Ok(10) => {
                        msg.supported_obj_path = Some(r.read_string(bytes).map(Cow::Borrowed)?)
                    }
                    Ok(16) => msg.access = Some(r.read_enum(bytes)?),
                    Ok(24) => msg.is_multi_instance = Some(r.read_bool(bytes)?),
                    Ok(34) => msg.supported_commands.push(
                        r.read_message::<usp::mod_GetSupportedDMResp::SupportedCommandResult>(
                            bytes,
                        )?,
                    ),
                    Ok(42) => msg.supported_events.push(
                        r.read_message::<usp::mod_GetSupportedDMResp::SupportedEventResult>(bytes)?,
                    ),
                    Ok(50) => msg.supported_params.push(
                        r.read_message::<usp::mod_GetSupportedDMResp::SupportedParamResult>(bytes)?,
                    ),
                    Ok(t) => {
                        r.read_unknown(bytes, t)?;
                    }
                    Err(e) => return Err(e),
                }
            }
            Ok(msg)
        }
    }

    impl<'a> MessageWrite for SupportedObjectResult<'a> {
        fn get_size(&self) -> usize {
            0 + self
                .supported_obj_path
                .as_ref()
                .map_or(0, |m| 1 + sizeof_len((m).len()))
                + self
                    .access
                    .as_ref()
                    .map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
                + self
                    .is_multi_instance
                    .as_ref()
                    .map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
                + self
                    .supported_commands
                    .iter()
                    .map(|s| 1 + sizeof_len((s).get_size()))
                    .sum::<usize>()
                + self
                    .supported_events
                    .iter()
                    .map(|s| 1 + sizeof_len((s).get_size()))
                    .sum::<usize>()
                + self
                    .supported_params
                    .iter()
                    .map(|s| 1 + sizeof_len((s).get_size()))
                    .sum::<usize>()
        }

        fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
            if let Some(ref s) = self.supported_obj_path {
                w.write_with_tag(10, |w| w.write_string(&**s))?;
            }
            if let Some(ref s) = self.access {
                w.write_with_tag(16, |w| w.write_enum(*s as i32))?;
            }
            if let Some(ref s) = self.is_multi_instance {
                w.write_with_tag(24, |w| w.write_bool(*s))?;
            }
            for s in &self.supported_commands {
                w.write_with_tag(34, |w| w.write_message(s))?;
            }
            for s in &self.supported_events {
                w.write_with_tag(42, |w| w.write_message(s))?;
            }
            for s in &self.supported_params {
                w.write_with_tag(50, |w| w.write_message(s))?;
            }
            Ok(())
        }
    }

    #[derive(Debug, Default, PartialEq, Clone)]
    pub struct SupportedParamResult<'a> {
        pub param_name: Option<Cow<'a, str>>,
        pub access: Option<usp::mod_GetSupportedDMResp::ParamAccessType>,
    }

    impl<'a> MessageRead<'a> for SupportedParamResult<'a> {
        fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
            let mut msg = Self::default();
            while !r.is_eof() {
                match r.next_tag(bytes) {
                    Ok(10) => msg.param_name = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                    Ok(16) => msg.access = Some(r.read_enum(bytes)?),
                    Ok(t) => {
                        r.read_unknown(bytes, t)?;
                    }
                    Err(e) => return Err(e),
                }
            }
            Ok(msg)
        }
    }

    impl<'a> MessageWrite for SupportedParamResult<'a> {
        fn get_size(&self) -> usize {
            0 + self
                .param_name
                .as_ref()
                .map_or(0, |m| 1 + sizeof_len((m).len()))
                + self
                    .access
                    .as_ref()
                    .map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        }

        fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
            if let Some(ref s) = self.param_name {
                w.write_with_tag(10, |w| w.write_string(&**s))?;
            }
            if let Some(ref s) = self.access {
                w.write_with_tag(16, |w| w.write_enum(*s as i32))?;
            }
            Ok(())
        }
    }

    #[derive(Debug, Default, PartialEq, Clone)]
    pub struct SupportedCommandResult<'a> {
        pub command_name: Option<Cow<'a, str>>,
        pub input_arg_names: Vec<Cow<'a, str>>,
        pub output_arg_names: Vec<Cow<'a, str>>,
    }

    impl<'a> MessageRead<'a> for SupportedCommandResult<'a> {
        fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
            let mut msg = Self::default();
            while !r.is_eof() {
                match r.next_tag(bytes) {
                    Ok(10) => msg.command_name = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                    Ok(18) => msg
                        .input_arg_names
                        .push(r.read_string(bytes).map(Cow::Borrowed)?),
                    Ok(26) => msg
                        .output_arg_names
                        .push(r.read_string(bytes).map(Cow::Borrowed)?),
                    Ok(t) => {
                        r.read_unknown(bytes, t)?;
                    }
                    Err(e) => return Err(e),
                }
            }
            Ok(msg)
        }
    }

    impl<'a> MessageWrite for SupportedCommandResult<'a> {
        fn get_size(&self) -> usize {
            0 + self
                .command_name
                .as_ref()
                .map_or(0, |m| 1 + sizeof_len((m).len()))
                + self
                    .input_arg_names
                    .iter()
                    .map(|s| 1 + sizeof_len((s).len()))
                    .sum::<usize>()
                + self
                    .output_arg_names
                    .iter()
                    .map(|s| 1 + sizeof_len((s).len()))
                    .sum::<usize>()
        }

        fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
            if let Some(ref s) = self.command_name {
                w.write_with_tag(10, |w| w.write_string(&**s))?;
            }
            for s in &self.input_arg_names {
                w.write_with_tag(18, |w| w.write_string(&**s))?;
            }
            for s in &self.output_arg_names {
                w.write_with_tag(26, |w| w.write_string(&**s))?;
            }
            Ok(())
        }
    }

    #[derive(Debug, Default, PartialEq, Clone)]
    pub struct SupportedEventResult<'a> {
        pub event_name: Option<Cow<'a, str>>,
        pub arg_names: Vec<Cow<'a, str>>,
    }

    impl<'a> MessageRead<'a> for SupportedEventResult<'a> {
        fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
            let mut msg = Self::default();
            while !r.is_eof() {
                match r.next_tag(bytes) {
                    Ok(10) => msg.event_name = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                    Ok(18) => msg.arg_names.push(r.read_string(bytes).map(Cow::Borrowed)?),
                    Ok(t) => {
                        r.read_unknown(bytes, t)?;
                    }
                    Err(e) => return Err(e),
                }
            }
            Ok(msg)
        }
    }

    impl<'a> MessageWrite for SupportedEventResult<'a> {
        fn get_size(&self) -> usize {
            0 + self
                .event_name
                .as_ref()
                .map_or(0, |m| 1 + sizeof_len((m).len()))
                + self
                    .arg_names
                    .iter()
                    .map(|s| 1 + sizeof_len((s).len()))
                    .sum::<usize>()
        }

        fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
            if let Some(ref s) = self.event_name {
                w.write_with_tag(10, |w| w.write_string(&**s))?;
            }
            for s in &self.arg_names {
                w.write_with_tag(18, |w| w.write_string(&**s))?;
            }
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

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GetInstances<'a> {
    pub obj_paths: Vec<Cow<'a, str>>,
    pub first_level_only: Option<bool>,
}

impl<'a> MessageRead<'a> for GetInstances<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.obj_paths.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(16) => msg.first_level_only = Some(r.read_bool(bytes)?),
                Ok(t) => {
                    r.read_unknown(bytes, t)?;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for GetInstances<'a> {
    fn get_size(&self) -> usize {
        0 + self
            .obj_paths
            .iter()
            .map(|s| 1 + sizeof_len((s).len()))
            .sum::<usize>()
            + self
                .first_level_only
                .as_ref()
                .map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.obj_paths {
            w.write_with_tag(10, |w| w.write_string(&**s))?;
        }
        if let Some(ref s) = self.first_level_only {
            w.write_with_tag(16, |w| w.write_bool(*s))?;
        }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GetInstancesResp<'a> {
    pub req_path_results: Vec<usp::mod_GetInstancesResp::RequestedPathResult<'a>>,
}

impl<'a> MessageRead<'a> for GetInstancesResp<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg
                    .req_path_results
                    .push(r.read_message::<usp::mod_GetInstancesResp::RequestedPathResult>(bytes)?),
                Ok(t) => {
                    r.read_unknown(bytes, t)?;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for GetInstancesResp<'a> {
    fn get_size(&self) -> usize {
        0 + self
            .req_path_results
            .iter()
            .map(|s| 1 + sizeof_len((s).get_size()))
            .sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.req_path_results {
            w.write_with_tag(10, |w| w.write_message(s))?;
        }
        Ok(())
    }
}

pub mod mod_GetInstancesResp {

    use super::*;
    use std::borrow::Cow;
    use std::collections::HashMap;

    #[derive(Debug, Default, PartialEq, Clone)]
    pub struct RequestedPathResult<'a> {
        pub requested_path: Option<Cow<'a, str>>,
        pub err_code: Option<u32>,
        pub err_msg: Option<Cow<'a, str>>,
        pub curr_insts: Vec<usp::mod_GetInstancesResp::CurrInstance<'a>>,
    }

    impl<'a> MessageRead<'a> for RequestedPathResult<'a> {
        fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
            let mut msg = Self::default();
            while !r.is_eof() {
                match r.next_tag(bytes) {
                    Ok(10) => msg.requested_path = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                    Ok(21) => msg.err_code = Some(r.read_fixed32(bytes)?),
                    Ok(26) => msg.err_msg = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                    Ok(34) => msg
                        .curr_insts
                        .push(r.read_message::<usp::mod_GetInstancesResp::CurrInstance>(bytes)?),
                    Ok(t) => {
                        r.read_unknown(bytes, t)?;
                    }
                    Err(e) => return Err(e),
                }
            }
            Ok(msg)
        }
    }

    impl<'a> MessageWrite for RequestedPathResult<'a> {
        fn get_size(&self) -> usize {
            0 + self
                .requested_path
                .as_ref()
                .map_or(0, |m| 1 + sizeof_len((m).len()))
                + self.err_code.as_ref().map_or(0, |_| 1 + 4)
                + self
                    .err_msg
                    .as_ref()
                    .map_or(0, |m| 1 + sizeof_len((m).len()))
                + self
                    .curr_insts
                    .iter()
                    .map(|s| 1 + sizeof_len((s).get_size()))
                    .sum::<usize>()
        }

        fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
            if let Some(ref s) = self.requested_path {
                w.write_with_tag(10, |w| w.write_string(&**s))?;
            }
            if let Some(ref s) = self.err_code {
                w.write_with_tag(21, |w| w.write_fixed32(*s))?;
            }
            if let Some(ref s) = self.err_msg {
                w.write_with_tag(26, |w| w.write_string(&**s))?;
            }
            for s in &self.curr_insts {
                w.write_with_tag(34, |w| w.write_message(s))?;
            }
            Ok(())
        }
    }

    #[derive(Debug, Default, PartialEq, Clone)]
    pub struct CurrInstance<'a> {
        pub instantiated_obj_path: Option<Cow<'a, str>>,
        pub unique_keys: HashMap<Cow<'a, str>, Cow<'a, str>>,
    }

    impl<'a> MessageRead<'a> for CurrInstance<'a> {
        fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
            let mut msg = Self::default();
            while !r.is_eof() {
                match r.next_tag(bytes) {
                    Ok(10) => {
                        msg.instantiated_obj_path = Some(r.read_string(bytes).map(Cow::Borrowed)?)
                    }
                    Ok(18) => {
                        let (key, value) = r.read_map(
                            bytes,
                            |r, bytes| r.read_string(bytes).map(Cow::Borrowed),
                            |r, bytes| r.read_string(bytes).map(Cow::Borrowed),
                        )?;
                        msg.unique_keys.insert(key, value);
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

    impl<'a> MessageWrite for CurrInstance<'a> {
        fn get_size(&self) -> usize {
            0 + self
                .instantiated_obj_path
                .as_ref()
                .map_or(0, |m| 1 + sizeof_len((m).len()))
                + self
                    .unique_keys
                    .iter()
                    .map(|(k, v)| 1 + sizeof_len(2 + sizeof_len((k).len()) + sizeof_len((v).len())))
                    .sum::<usize>()
        }

        fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
            if let Some(ref s) = self.instantiated_obj_path {
                w.write_with_tag(10, |w| w.write_string(&**s))?;
            }
            for (k, v) in self.unique_keys.iter() {
                w.write_with_tag(18, |w| {
                    w.write_map(
                        2 + sizeof_len((k).len()) + sizeof_len((v).len()),
                        10,
                        |w| w.write_string(&**k),
                        18,
                        |w| w.write_string(&**v),
                    )
                })?;
            }
            Ok(())
        }
    }

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GetSupportedProtocol<'a> {
    pub controller_supported_protocol_versions: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for GetSupportedProtocol<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => {
                    msg.controller_supported_protocol_versions =
                        Some(r.read_string(bytes).map(Cow::Borrowed)?)
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

impl<'a> MessageWrite for GetSupportedProtocol<'a> {
    fn get_size(&self) -> usize {
        0 + self
            .controller_supported_protocol_versions
            .as_ref()
            .map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.controller_supported_protocol_versions {
            w.write_with_tag(10, |w| w.write_string(&**s))?;
        }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GetSupportedProtocolResp<'a> {
    pub agent_supported_protocol_versions: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for GetSupportedProtocolResp<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => {
                    msg.agent_supported_protocol_versions =
                        Some(r.read_string(bytes).map(Cow::Borrowed)?)
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

impl<'a> MessageWrite for GetSupportedProtocolResp<'a> {
    fn get_size(&self) -> usize {
        0 + self
            .agent_supported_protocol_versions
            .as_ref()
            .map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.agent_supported_protocol_versions {
            w.write_with_tag(10, |w| w.write_string(&**s))?;
        }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Add<'a> {
    pub allow_partial: Option<bool>,
    pub create_objs: Vec<usp::mod_Add::CreateObject<'a>>,
}

impl<'a> MessageRead<'a> for Add<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.allow_partial = Some(r.read_bool(bytes)?),
                Ok(18) => msg
                    .create_objs
                    .push(r.read_message::<usp::mod_Add::CreateObject>(bytes)?),
                Ok(t) => {
                    r.read_unknown(bytes, t)?;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Add<'a> {
    fn get_size(&self) -> usize {
        0 + self
            .allow_partial
            .as_ref()
            .map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
            + self
                .create_objs
                .iter()
                .map(|s| 1 + sizeof_len((s).get_size()))
                .sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.allow_partial {
            w.write_with_tag(8, |w| w.write_bool(*s))?;
        }
        for s in &self.create_objs {
            w.write_with_tag(18, |w| w.write_message(s))?;
        }
        Ok(())
    }
}

pub mod mod_Add {

    use super::*;
    use std::borrow::Cow;

    #[derive(Debug, Default, PartialEq, Clone)]
    pub struct CreateObject<'a> {
        pub obj_path: Option<Cow<'a, str>>,
        pub param_settings: Vec<usp::mod_Add::CreateParamSetting<'a>>,
    }

    impl<'a> MessageRead<'a> for CreateObject<'a> {
        fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
            let mut msg = Self::default();
            while !r.is_eof() {
                match r.next_tag(bytes) {
                    Ok(10) => msg.obj_path = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                    Ok(18) => msg
                        .param_settings
                        .push(r.read_message::<usp::mod_Add::CreateParamSetting>(bytes)?),
                    Ok(t) => {
                        r.read_unknown(bytes, t)?;
                    }
                    Err(e) => return Err(e),
                }
            }
            Ok(msg)
        }
    }

    impl<'a> MessageWrite for CreateObject<'a> {
        fn get_size(&self) -> usize {
            0 + self
                .obj_path
                .as_ref()
                .map_or(0, |m| 1 + sizeof_len((m).len()))
                + self
                    .param_settings
                    .iter()
                    .map(|s| 1 + sizeof_len((s).get_size()))
                    .sum::<usize>()
        }

        fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
            if let Some(ref s) = self.obj_path {
                w.write_with_tag(10, |w| w.write_string(&**s))?;
            }
            for s in &self.param_settings {
                w.write_with_tag(18, |w| w.write_message(s))?;
            }
            Ok(())
        }
    }

    #[derive(Debug, Default, PartialEq, Clone)]
    pub struct CreateParamSetting<'a> {
        pub param: Option<Cow<'a, str>>,
        pub value: Option<Cow<'a, str>>,
        pub required: Option<bool>,
    }

    impl<'a> MessageRead<'a> for CreateParamSetting<'a> {
        fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
            let mut msg = Self::default();
            while !r.is_eof() {
                match r.next_tag(bytes) {
                    Ok(10) => msg.param = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                    Ok(18) => msg.value = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                    Ok(24) => msg.required = Some(r.read_bool(bytes)?),
                    Ok(t) => {
                        r.read_unknown(bytes, t)?;
                    }
                    Err(e) => return Err(e),
                }
            }
            Ok(msg)
        }
    }

    impl<'a> MessageWrite for CreateParamSetting<'a> {
        fn get_size(&self) -> usize {
            0 + self.param.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
                + self.value.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
                + self
                    .required
                    .as_ref()
                    .map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        }

        fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
            if let Some(ref s) = self.param {
                w.write_with_tag(10, |w| w.write_string(&**s))?;
            }
            if let Some(ref s) = self.value {
                w.write_with_tag(18, |w| w.write_string(&**s))?;
            }
            if let Some(ref s) = self.required {
                w.write_with_tag(24, |w| w.write_bool(*s))?;
            }
            Ok(())
        }
    }

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct AddResp<'a> {
    pub created_obj_results: Vec<usp::mod_AddResp::CreatedObjectResult<'a>>,
}

impl<'a> MessageRead<'a> for AddResp<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg
                    .created_obj_results
                    .push(r.read_message::<usp::mod_AddResp::CreatedObjectResult>(bytes)?),
                Ok(t) => {
                    r.read_unknown(bytes, t)?;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for AddResp<'a> {
    fn get_size(&self) -> usize {
        0 + self
            .created_obj_results
            .iter()
            .map(|s| 1 + sizeof_len((s).get_size()))
            .sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.created_obj_results {
            w.write_with_tag(10, |w| w.write_message(s))?;
        }
        Ok(())
    }
}

pub mod mod_AddResp {

    use super::*;
    use std::borrow::Cow;

    #[derive(Debug, Default, PartialEq, Clone)]
    pub struct CreatedObjectResult<'a> {
        pub requested_path: Option<Cow<'a, str>>,
        pub oper_status: Option<usp::mod_AddResp::mod_CreatedObjectResult::OperationStatus<'a>>,
    }

    impl<'a> MessageRead<'a> for CreatedObjectResult<'a> {
        fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
            let mut msg = Self::default();
            while !r.is_eof() {
                match r.next_tag(bytes) {
                Ok(10) => msg.requested_path = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.oper_status = Some(r.read_message::<usp::mod_AddResp::mod_CreatedObjectResult::OperationStatus>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
            }
            Ok(msg)
        }
    }

    impl<'a> MessageWrite for CreatedObjectResult<'a> {
        fn get_size(&self) -> usize {
            0 + self
                .requested_path
                .as_ref()
                .map_or(0, |m| 1 + sizeof_len((m).len()))
                + self
                    .oper_status
                    .as_ref()
                    .map_or(0, |m| 1 + sizeof_len((m).get_size()))
        }

        fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
            if let Some(ref s) = self.requested_path {
                w.write_with_tag(10, |w| w.write_string(&**s))?;
            }
            if let Some(ref s) = self.oper_status {
                w.write_with_tag(18, |w| w.write_message(s))?;
            }
            Ok(())
        }
    }

    pub mod mod_CreatedObjectResult {

        use super::*;

        #[derive(Debug, Default, PartialEq, Clone)]
        pub struct OperationStatus<'a> {
            pub oper_status:
                usp::mod_AddResp::mod_CreatedObjectResult::mod_OperationStatus::OneOfoper_status<
                    'a,
                >,
        }

        impl<'a> MessageRead<'a> for OperationStatus<'a> {
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

        impl<'a> MessageWrite for OperationStatus<'a> {
            fn get_size(&self) -> usize {
                0
        + match self.oper_status {
            usp::mod_AddResp::mod_CreatedObjectResult::mod_OperationStatus::OneOfoper_status::oper_failure(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_AddResp::mod_CreatedObjectResult::mod_OperationStatus::OneOfoper_status::oper_success(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_AddResp::mod_CreatedObjectResult::mod_OperationStatus::OneOfoper_status::None => 0,
    }
            }

            fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
                match self.oper_status {            usp::mod_AddResp::mod_CreatedObjectResult::mod_OperationStatus::OneOfoper_status::oper_failure(ref m) => { w.write_with_tag(10, |w| w.write_message(m))? },
            usp::mod_AddResp::mod_CreatedObjectResult::mod_OperationStatus::OneOfoper_status::oper_success(ref m) => { w.write_with_tag(18, |w| w.write_message(m))? },
            usp::mod_AddResp::mod_CreatedObjectResult::mod_OperationStatus::OneOfoper_status::None => {},
    }
                Ok(())
            }
        }

        pub mod mod_OperationStatus {

            use super::*;
            use std::borrow::Cow;
            use std::collections::HashMap;

            #[derive(Debug, Default, PartialEq, Clone)]
            pub struct OperationFailure<'a> {
                pub err_code: Option<u32>,
                pub err_msg: Option<Cow<'a, str>>,
            }

            impl<'a> MessageRead<'a> for OperationFailure<'a> {
                fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
                    let mut msg = Self::default();
                    while !r.is_eof() {
                        match r.next_tag(bytes) {
                            Ok(13) => msg.err_code = Some(r.read_fixed32(bytes)?),
                            Ok(18) => msg.err_msg = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                            Ok(t) => {
                                r.read_unknown(bytes, t)?;
                            }
                            Err(e) => return Err(e),
                        }
                    }
                    Ok(msg)
                }
            }

            impl<'a> MessageWrite for OperationFailure<'a> {
                fn get_size(&self) -> usize {
                    0 + self.err_code.as_ref().map_or(0, |_| 1 + 4) + self
                        .err_msg
                        .as_ref()
                        .map_or(0, |m| 1 + sizeof_len((m).len()))
                }

                fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
                    if let Some(ref s) = self.err_code {
                        w.write_with_tag(13, |w| w.write_fixed32(*s))?;
                    }
                    if let Some(ref s) = self.err_msg {
                        w.write_with_tag(18, |w| w.write_string(&**s))?;
                    }
                    Ok(())
                }
            }

            #[derive(Debug, Default, PartialEq, Clone)]
            pub struct OperationSuccess<'a> {
                pub instantiated_path: Option<Cow<'a, str>>,
                pub param_errs: Vec<
                    usp::mod_AddResp::mod_CreatedObjectResult::mod_OperationStatus::ParameterError<
                        'a,
                    >,
                >,
                pub unique_keys: HashMap<Cow<'a, str>, Cow<'a, str>>,
            }

            impl<'a> MessageRead<'a> for OperationSuccess<'a> {
                fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
                    let mut msg = Self::default();
                    while !r.is_eof() {
                        match r.next_tag(bytes) {
                Ok(10) => msg.instantiated_path = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.param_errs.push(r.read_message::<usp::mod_AddResp::mod_CreatedObjectResult::mod_OperationStatus::ParameterError>(bytes)?),
                Ok(26) => {
                    let (key, value) = r.read_map(bytes, |r, bytes| r.read_string(bytes).map(Cow::Borrowed), |r, bytes| r.read_string(bytes).map(Cow::Borrowed))?;
                    msg.unique_keys.insert(key, value);
                }
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
                    }
                    Ok(msg)
                }
            }

            impl<'a> MessageWrite for OperationSuccess<'a> {
                fn get_size(&self) -> usize {
                    0 + self
                        .instantiated_path
                        .as_ref()
                        .map_or(0, |m| 1 + sizeof_len((m).len()))
                        + self
                            .param_errs
                            .iter()
                            .map(|s| 1 + sizeof_len((s).get_size()))
                            .sum::<usize>()
                        + self
                            .unique_keys
                            .iter()
                            .map(|(k, v)| {
                                1 + sizeof_len(2 + sizeof_len((k).len()) + sizeof_len((v).len()))
                            })
                            .sum::<usize>()
                }

                fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
                    if let Some(ref s) = self.instantiated_path {
                        w.write_with_tag(10, |w| w.write_string(&**s))?;
                    }
                    for s in &self.param_errs {
                        w.write_with_tag(18, |w| w.write_message(s))?;
                    }
                    for (k, v) in self.unique_keys.iter() {
                        w.write_with_tag(26, |w| {
                            w.write_map(
                                2 + sizeof_len((k).len()) + sizeof_len((v).len()),
                                10,
                                |w| w.write_string(&**k),
                                18,
                                |w| w.write_string(&**v),
                            )
                        })?;
                    }
                    Ok(())
                }
            }

            #[derive(Debug, Default, PartialEq, Clone)]
            pub struct ParameterError<'a> {
                pub param: Option<Cow<'a, str>>,
                pub err_code: Option<u32>,
                pub err_msg: Option<Cow<'a, str>>,
            }

            impl<'a> MessageRead<'a> for ParameterError<'a> {
                fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
                    let mut msg = Self::default();
                    while !r.is_eof() {
                        match r.next_tag(bytes) {
                            Ok(10) => msg.param = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                            Ok(21) => msg.err_code = Some(r.read_fixed32(bytes)?),
                            Ok(26) => msg.err_msg = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                            Ok(t) => {
                                r.read_unknown(bytes, t)?;
                            }
                            Err(e) => return Err(e),
                        }
                    }
                    Ok(msg)
                }
            }

            impl<'a> MessageWrite for ParameterError<'a> {
                fn get_size(&self) -> usize {
                    0 + self.param.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
                        + self.err_code.as_ref().map_or(0, |_| 1 + 4)
                        + self
                            .err_msg
                            .as_ref()
                            .map_or(0, |m| 1 + sizeof_len((m).len()))
                }

                fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
                    if let Some(ref s) = self.param {
                        w.write_with_tag(10, |w| w.write_string(&**s))?;
                    }
                    if let Some(ref s) = self.err_code {
                        w.write_with_tag(21, |w| w.write_fixed32(*s))?;
                    }
                    if let Some(ref s) = self.err_msg {
                        w.write_with_tag(26, |w| w.write_string(&**s))?;
                    }
                    Ok(())
                }
            }

            #[derive(Debug, PartialEq, Clone)]
            pub enum OneOfoper_status<'a> {
                
                
                
                
    oper_failure(usp::mod_AddResp::mod_CreatedObjectResult::mod_OperationStatus::OperationFailure<'a>),
    oper_success(usp::mod_AddResp::mod_CreatedObjectResult::mod_OperationStatus::OperationSuccess<'a>),
    None,
}

            impl<'a> Default for OneOfoper_status<'a> {
                fn default() -> Self {
                    OneOfoper_status::None
                }
            }

        }

    }

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Delete<'a> {
    pub allow_partial: Option<bool>,
    pub obj_paths: Vec<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for Delete<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.allow_partial = Some(r.read_bool(bytes)?),
                Ok(18) => msg.obj_paths.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => {
                    r.read_unknown(bytes, t)?;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Delete<'a> {
    fn get_size(&self) -> usize {
        0 + self
            .allow_partial
            .as_ref()
            .map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
            + self
                .obj_paths
                .iter()
                .map(|s| 1 + sizeof_len((s).len()))
                .sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.allow_partial {
            w.write_with_tag(8, |w| w.write_bool(*s))?;
        }
        for s in &self.obj_paths {
            w.write_with_tag(18, |w| w.write_string(&**s))?;
        }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DeleteResp<'a> {
    pub deleted_obj_results: Vec<usp::mod_DeleteResp::DeletedObjectResult<'a>>,
}

impl<'a> MessageRead<'a> for DeleteResp<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg
                    .deleted_obj_results
                    .push(r.read_message::<usp::mod_DeleteResp::DeletedObjectResult>(bytes)?),
                Ok(t) => {
                    r.read_unknown(bytes, t)?;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for DeleteResp<'a> {
    fn get_size(&self) -> usize {
        0 + self
            .deleted_obj_results
            .iter()
            .map(|s| 1 + sizeof_len((s).get_size()))
            .sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.deleted_obj_results {
            w.write_with_tag(10, |w| w.write_message(s))?;
        }
        Ok(())
    }
}

pub mod mod_DeleteResp {

    use super::*;
    use std::borrow::Cow;

    #[derive(Debug, Default, PartialEq, Clone)]
    pub struct DeletedObjectResult<'a> {
        pub requested_path: Option<Cow<'a, str>>,
        pub oper_status: Option<usp::mod_DeleteResp::mod_DeletedObjectResult::OperationStatus<'a>>,
    }

    impl<'a> MessageRead<'a> for DeletedObjectResult<'a> {
        fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
            let mut msg = Self::default();
            while !r.is_eof() {
                match r.next_tag(bytes) {
                Ok(10) => msg.requested_path = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.oper_status = Some(r.read_message::<usp::mod_DeleteResp::mod_DeletedObjectResult::OperationStatus>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
            }
            Ok(msg)
        }
    }

    impl<'a> MessageWrite for DeletedObjectResult<'a> {
        fn get_size(&self) -> usize {
            0 + self
                .requested_path
                .as_ref()
                .map_or(0, |m| 1 + sizeof_len((m).len()))
                + self
                    .oper_status
                    .as_ref()
                    .map_or(0, |m| 1 + sizeof_len((m).get_size()))
        }

        fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
            if let Some(ref s) = self.requested_path {
                w.write_with_tag(10, |w| w.write_string(&**s))?;
            }
            if let Some(ref s) = self.oper_status {
                w.write_with_tag(18, |w| w.write_message(s))?;
            }
            Ok(())
        }
    }

    pub mod mod_DeletedObjectResult {

        use super::*;

        #[derive(Debug, Default, PartialEq, Clone)]
        pub struct OperationStatus<'a> {
            pub oper_status:
                usp::mod_DeleteResp::mod_DeletedObjectResult::mod_OperationStatus::OneOfoper_status<
                    'a,
                >,
        }

        impl<'a> MessageRead<'a> for OperationStatus<'a> {
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

        impl<'a> MessageWrite for OperationStatus<'a> {
            fn get_size(&self) -> usize {
                0
        + match self.oper_status {
            usp::mod_DeleteResp::mod_DeletedObjectResult::mod_OperationStatus::OneOfoper_status::oper_failure(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_DeleteResp::mod_DeletedObjectResult::mod_OperationStatus::OneOfoper_status::oper_success(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_DeleteResp::mod_DeletedObjectResult::mod_OperationStatus::OneOfoper_status::None => 0,
    }
            }

            fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
                match self.oper_status {            usp::mod_DeleteResp::mod_DeletedObjectResult::mod_OperationStatus::OneOfoper_status::oper_failure(ref m) => { w.write_with_tag(10, |w| w.write_message(m))? },
            usp::mod_DeleteResp::mod_DeletedObjectResult::mod_OperationStatus::OneOfoper_status::oper_success(ref m) => { w.write_with_tag(18, |w| w.write_message(m))? },
            usp::mod_DeleteResp::mod_DeletedObjectResult::mod_OperationStatus::OneOfoper_status::None => {},
    }
                Ok(())
            }
        }

        pub mod mod_OperationStatus {

            use super::*;
            use std::borrow::Cow;

            #[derive(Debug, Default, PartialEq, Clone)]
            pub struct OperationFailure<'a> {
                pub err_code: Option<u32>,
                pub err_msg: Option<Cow<'a, str>>,
            }

            impl<'a> MessageRead<'a> for OperationFailure<'a> {
                fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
                    let mut msg = Self::default();
                    while !r.is_eof() {
                        match r.next_tag(bytes) {
                            Ok(13) => msg.err_code = Some(r.read_fixed32(bytes)?),
                            Ok(18) => msg.err_msg = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                            Ok(t) => {
                                r.read_unknown(bytes, t)?;
                            }
                            Err(e) => return Err(e),
                        }
                    }
                    Ok(msg)
                }
            }

            impl<'a> MessageWrite for OperationFailure<'a> {
                fn get_size(&self) -> usize {
                    0 + self.err_code.as_ref().map_or(0, |_| 1 + 4) + self
                        .err_msg
                        .as_ref()
                        .map_or(0, |m| 1 + sizeof_len((m).len()))
                }

                fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
                    if let Some(ref s) = self.err_code {
                        w.write_with_tag(13, |w| w.write_fixed32(*s))?;
                    }
                    if let Some(ref s) = self.err_msg {
                        w.write_with_tag(18, |w| w.write_string(&**s))?;
                    }
                    Ok(())
                }
            }

            #[derive(Debug, Default, PartialEq, Clone)]
            pub struct OperationSuccess<'a> {
    pub affected_paths: Vec<Cow<'a, str>>,
    pub unaffected_path_errs: Vec<usp::mod_DeleteResp::mod_DeletedObjectResult::mod_OperationStatus::UnaffectedPathError<'a>>,
}

            impl<'a> MessageRead<'a> for OperationSuccess<'a> {
                fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
                    let mut msg = Self::default();
                    while !r.is_eof() {
                        match r.next_tag(bytes) {
                Ok(10) => msg.affected_paths.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.unaffected_path_errs.push(r.read_message::<usp::mod_DeleteResp::mod_DeletedObjectResult::mod_OperationStatus::UnaffectedPathError>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
                    }
                    Ok(msg)
                }
            }

            impl<'a> MessageWrite for OperationSuccess<'a> {
                fn get_size(&self) -> usize {
                    0 + self
                        .affected_paths
                        .iter()
                        .map(|s| 1 + sizeof_len((s).len()))
                        .sum::<usize>()
                        + self
                            .unaffected_path_errs
                            .iter()
                            .map(|s| 1 + sizeof_len((s).get_size()))
                            .sum::<usize>()
                }

                fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
                    for s in &self.affected_paths {
                        w.write_with_tag(10, |w| w.write_string(&**s))?;
                    }
                    for s in &self.unaffected_path_errs {
                        w.write_with_tag(18, |w| w.write_message(s))?;
                    }
                    Ok(())
                }
            }

            #[derive(Debug, Default, PartialEq, Clone)]
            pub struct UnaffectedPathError<'a> {
                pub unaffected_path: Option<Cow<'a, str>>,
                pub err_code: Option<u32>,
                pub err_msg: Option<Cow<'a, str>>,
            }

            impl<'a> MessageRead<'a> for UnaffectedPathError<'a> {
                fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
                    let mut msg = Self::default();
                    while !r.is_eof() {
                        match r.next_tag(bytes) {
                            Ok(10) => {
                                msg.unaffected_path = Some(r.read_string(bytes).map(Cow::Borrowed)?)
                            }
                            Ok(21) => msg.err_code = Some(r.read_fixed32(bytes)?),
                            Ok(26) => msg.err_msg = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                            Ok(t) => {
                                r.read_unknown(bytes, t)?;
                            }
                            Err(e) => return Err(e),
                        }
                    }
                    Ok(msg)
                }
            }

            impl<'a> MessageWrite for UnaffectedPathError<'a> {
                fn get_size(&self) -> usize {
                    0 + self
                        .unaffected_path
                        .as_ref()
                        .map_or(0, |m| 1 + sizeof_len((m).len()))
                        + self.err_code.as_ref().map_or(0, |_| 1 + 4)
                        + self
                            .err_msg
                            .as_ref()
                            .map_or(0, |m| 1 + sizeof_len((m).len()))
                }

                fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
                    if let Some(ref s) = self.unaffected_path {
                        w.write_with_tag(10, |w| w.write_string(&**s))?;
                    }
                    if let Some(ref s) = self.err_code {
                        w.write_with_tag(21, |w| w.write_fixed32(*s))?;
                    }
                    if let Some(ref s) = self.err_msg {
                        w.write_with_tag(26, |w| w.write_string(&**s))?;
                    }
                    Ok(())
                }
            }

            #[derive(Debug, PartialEq, Clone)]
            pub enum OneOfoper_status<'a> {
                
                
                
                
    oper_failure(usp::mod_DeleteResp::mod_DeletedObjectResult::mod_OperationStatus::OperationFailure<'a>),
    oper_success(usp::mod_DeleteResp::mod_DeletedObjectResult::mod_OperationStatus::OperationSuccess<'a>),
    None,
}

            impl<'a> Default for OneOfoper_status<'a> {
                fn default() -> Self {
                    OneOfoper_status::None
                }
            }

        }

    }

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Set<'a> {
    pub allow_partial: Option<bool>,
    pub update_objs: Vec<usp::mod_Set::UpdateObject<'a>>,
}

impl<'a> MessageRead<'a> for Set<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.allow_partial = Some(r.read_bool(bytes)?),
                Ok(18) => msg
                    .update_objs
                    .push(r.read_message::<usp::mod_Set::UpdateObject>(bytes)?),
                Ok(t) => {
                    r.read_unknown(bytes, t)?;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Set<'a> {
    fn get_size(&self) -> usize {
        0 + self
            .allow_partial
            .as_ref()
            .map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
            + self
                .update_objs
                .iter()
                .map(|s| 1 + sizeof_len((s).get_size()))
                .sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.allow_partial {
            w.write_with_tag(8, |w| w.write_bool(*s))?;
        }
        for s in &self.update_objs {
            w.write_with_tag(18, |w| w.write_message(s))?;
        }
        Ok(())
    }
}

pub mod mod_Set {

    use super::*;
    use std::borrow::Cow;

    #[derive(Debug, Default, PartialEq, Clone)]
    pub struct UpdateObject<'a> {
        pub obj_path: Option<Cow<'a, str>>,
        pub param_settings: Vec<usp::mod_Set::UpdateParamSetting<'a>>,
    }

    impl<'a> MessageRead<'a> for UpdateObject<'a> {
        fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
            let mut msg = Self::default();
            while !r.is_eof() {
                match r.next_tag(bytes) {
                    Ok(10) => msg.obj_path = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                    Ok(18) => msg
                        .param_settings
                        .push(r.read_message::<usp::mod_Set::UpdateParamSetting>(bytes)?),
                    Ok(t) => {
                        r.read_unknown(bytes, t)?;
                    }
                    Err(e) => return Err(e),
                }
            }
            Ok(msg)
        }
    }

    impl<'a> MessageWrite for UpdateObject<'a> {
        fn get_size(&self) -> usize {
            0 + self
                .obj_path
                .as_ref()
                .map_or(0, |m| 1 + sizeof_len((m).len()))
                + self
                    .param_settings
                    .iter()
                    .map(|s| 1 + sizeof_len((s).get_size()))
                    .sum::<usize>()
        }

        fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
            if let Some(ref s) = self.obj_path {
                w.write_with_tag(10, |w| w.write_string(&**s))?;
            }
            for s in &self.param_settings {
                w.write_with_tag(18, |w| w.write_message(s))?;
            }
            Ok(())
        }
    }

    #[derive(Debug, Default, PartialEq, Clone)]
    pub struct UpdateParamSetting<'a> {
        pub param: Option<Cow<'a, str>>,
        pub value: Option<Cow<'a, str>>,
        pub required: Option<bool>,
    }

    impl<'a> MessageRead<'a> for UpdateParamSetting<'a> {
        fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
            let mut msg = Self::default();
            while !r.is_eof() {
                match r.next_tag(bytes) {
                    Ok(10) => msg.param = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                    Ok(18) => msg.value = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                    Ok(24) => msg.required = Some(r.read_bool(bytes)?),
                    Ok(t) => {
                        r.read_unknown(bytes, t)?;
                    }
                    Err(e) => return Err(e),
                }
            }
            Ok(msg)
        }
    }

    impl<'a> MessageWrite for UpdateParamSetting<'a> {
        fn get_size(&self) -> usize {
            0 + self.param.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
                + self.value.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
                + self
                    .required
                    .as_ref()
                    .map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        }

        fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
            if let Some(ref s) = self.param {
                w.write_with_tag(10, |w| w.write_string(&**s))?;
            }
            if let Some(ref s) = self.value {
                w.write_with_tag(18, |w| w.write_string(&**s))?;
            }
            if let Some(ref s) = self.required {
                w.write_with_tag(24, |w| w.write_bool(*s))?;
            }
            Ok(())
        }
    }

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SetResp<'a> {
    pub updated_obj_results: Vec<usp::mod_SetResp::UpdatedObjectResult<'a>>,
}

impl<'a> MessageRead<'a> for SetResp<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg
                    .updated_obj_results
                    .push(r.read_message::<usp::mod_SetResp::UpdatedObjectResult>(bytes)?),
                Ok(t) => {
                    r.read_unknown(bytes, t)?;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for SetResp<'a> {
    fn get_size(&self) -> usize {
        0 + self
            .updated_obj_results
            .iter()
            .map(|s| 1 + sizeof_len((s).get_size()))
            .sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.updated_obj_results {
            w.write_with_tag(10, |w| w.write_message(s))?;
        }
        Ok(())
    }
}

pub mod mod_SetResp {

    use super::*;
    use std::borrow::Cow;

    #[derive(Debug, Default, PartialEq, Clone)]
    pub struct UpdatedObjectResult<'a> {
        pub requested_path: Option<Cow<'a, str>>,
        pub oper_status: Option<usp::mod_SetResp::mod_UpdatedObjectResult::OperationStatus<'a>>,
    }

    impl<'a> MessageRead<'a> for UpdatedObjectResult<'a> {
        fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
            let mut msg = Self::default();
            while !r.is_eof() {
                match r.next_tag(bytes) {
                Ok(10) => msg.requested_path = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.oper_status = Some(r.read_message::<usp::mod_SetResp::mod_UpdatedObjectResult::OperationStatus>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
            }
            Ok(msg)
        }
    }

    impl<'a> MessageWrite for UpdatedObjectResult<'a> {
        fn get_size(&self) -> usize {
            0 + self
                .requested_path
                .as_ref()
                .map_or(0, |m| 1 + sizeof_len((m).len()))
                + self
                    .oper_status
                    .as_ref()
                    .map_or(0, |m| 1 + sizeof_len((m).get_size()))
        }

        fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
            if let Some(ref s) = self.requested_path {
                w.write_with_tag(10, |w| w.write_string(&**s))?;
            }
            if let Some(ref s) = self.oper_status {
                w.write_with_tag(18, |w| w.write_message(s))?;
            }
            Ok(())
        }
    }

    pub mod mod_UpdatedObjectResult {

        use super::*;

        #[derive(Debug, Default, PartialEq, Clone)]
        pub struct OperationStatus<'a> {
            pub oper_status:
                usp::mod_SetResp::mod_UpdatedObjectResult::mod_OperationStatus::OneOfoper_status<
                    'a,
                >,
        }

        impl<'a> MessageRead<'a> for OperationStatus<'a> {
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

        impl<'a> MessageWrite for OperationStatus<'a> {
            fn get_size(&self) -> usize {
                0
        + match self.oper_status {
            usp::mod_SetResp::mod_UpdatedObjectResult::mod_OperationStatus::OneOfoper_status::oper_failure(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_SetResp::mod_UpdatedObjectResult::mod_OperationStatus::OneOfoper_status::oper_success(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_SetResp::mod_UpdatedObjectResult::mod_OperationStatus::OneOfoper_status::None => 0,
    }
            }

            fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
                match self.oper_status {            usp::mod_SetResp::mod_UpdatedObjectResult::mod_OperationStatus::OneOfoper_status::oper_failure(ref m) => { w.write_with_tag(10, |w| w.write_message(m))? },
            usp::mod_SetResp::mod_UpdatedObjectResult::mod_OperationStatus::OneOfoper_status::oper_success(ref m) => { w.write_with_tag(18, |w| w.write_message(m))? },
            usp::mod_SetResp::mod_UpdatedObjectResult::mod_OperationStatus::OneOfoper_status::None => {},
    }
                Ok(())
            }
        }

        pub mod mod_OperationStatus {

            use super::*;
            use std::borrow::Cow;
            use std::collections::HashMap;

            #[derive(Debug, Default, PartialEq, Clone)]
            pub struct OperationFailure<'a> {
    pub err_code: Option<u32>,
    pub err_msg: Option<Cow<'a, str>>,
    pub updated_inst_failures: Vec<usp::mod_SetResp::mod_UpdatedObjectResult::mod_OperationStatus::UpdatedInstanceFailure<'a>>,
}

            impl<'a> MessageRead<'a> for OperationFailure<'a> {
                fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
                    let mut msg = Self::default();
                    while !r.is_eof() {
                        match r.next_tag(bytes) {
                Ok(13) => msg.err_code = Some(r.read_fixed32(bytes)?),
                Ok(18) => msg.err_msg = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.updated_inst_failures.push(r.read_message::<usp::mod_SetResp::mod_UpdatedObjectResult::mod_OperationStatus::UpdatedInstanceFailure>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
                    }
                    Ok(msg)
                }
            }

            impl<'a> MessageWrite for OperationFailure<'a> {
                fn get_size(&self) -> usize {
                    0 + self.err_code.as_ref().map_or(0, |_| 1 + 4)
                        + self
                            .err_msg
                            .as_ref()
                            .map_or(0, |m| 1 + sizeof_len((m).len()))
                        + self
                            .updated_inst_failures
                            .iter()
                            .map(|s| 1 + sizeof_len((s).get_size()))
                            .sum::<usize>()
                }

                fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
                    if let Some(ref s) = self.err_code {
                        w.write_with_tag(13, |w| w.write_fixed32(*s))?;
                    }
                    if let Some(ref s) = self.err_msg {
                        w.write_with_tag(18, |w| w.write_string(&**s))?;
                    }
                    for s in &self.updated_inst_failures {
                        w.write_with_tag(26, |w| w.write_message(s))?;
                    }
                    Ok(())
                }
            }

            #[derive(Debug, Default, PartialEq, Clone)]
            pub struct OperationSuccess<'a> {
    pub updated_inst_results: Vec<usp::mod_SetResp::mod_UpdatedObjectResult::mod_OperationStatus::UpdatedInstanceResult<'a>>,
}

            impl<'a> MessageRead<'a> for OperationSuccess<'a> {
                fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
                    let mut msg = Self::default();
                    while !r.is_eof() {
                        match r.next_tag(bytes) {
                Ok(10) => msg.updated_inst_results.push(r.read_message::<usp::mod_SetResp::mod_UpdatedObjectResult::mod_OperationStatus::UpdatedInstanceResult>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
                    }
                    Ok(msg)
                }
            }

            impl<'a> MessageWrite for OperationSuccess<'a> {
                fn get_size(&self) -> usize {
                    0 + self
                        .updated_inst_results
                        .iter()
                        .map(|s| 1 + sizeof_len((s).get_size()))
                        .sum::<usize>()
                }

                fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
                    for s in &self.updated_inst_results {
                        w.write_with_tag(10, |w| w.write_message(s))?;
                    }
                    Ok(())
                }
            }

            #[derive(Debug, Default, PartialEq, Clone)]
            pub struct UpdatedInstanceFailure<'a> {
                pub affected_path: Option<Cow<'a, str>>,
                pub param_errs: Vec<
                    usp::mod_SetResp::mod_UpdatedObjectResult::mod_OperationStatus::ParameterError<
                        'a,
                    >,
                >,
            }

            impl<'a> MessageRead<'a> for UpdatedInstanceFailure<'a> {
                fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
                    let mut msg = Self::default();
                    while !r.is_eof() {
                        match r.next_tag(bytes) {
                Ok(10) => msg.affected_path = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.param_errs.push(r.read_message::<usp::mod_SetResp::mod_UpdatedObjectResult::mod_OperationStatus::ParameterError>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
                    }
                    Ok(msg)
                }
            }

            impl<'a> MessageWrite for UpdatedInstanceFailure<'a> {
                fn get_size(&self) -> usize {
                    0 + self
                        .affected_path
                        .as_ref()
                        .map_or(0, |m| 1 + sizeof_len((m).len()))
                        + self
                            .param_errs
                            .iter()
                            .map(|s| 1 + sizeof_len((s).get_size()))
                            .sum::<usize>()
                }

                fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
                    if let Some(ref s) = self.affected_path {
                        w.write_with_tag(10, |w| w.write_string(&**s))?;
                    }
                    for s in &self.param_errs {
                        w.write_with_tag(18, |w| w.write_message(s))?;
                    }
                    Ok(())
                }
            }

            #[derive(Debug, Default, PartialEq, Clone)]
            pub struct UpdatedInstanceResult<'a> {
                pub affected_path: Option<Cow<'a, str>>,
                pub param_errs: Vec<
                    usp::mod_SetResp::mod_UpdatedObjectResult::mod_OperationStatus::ParameterError<
                        'a,
                    >,
                >,
                pub updated_params: HashMap<Cow<'a, str>, Cow<'a, str>>,
            }

            impl<'a> MessageRead<'a> for UpdatedInstanceResult<'a> {
                fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
                    let mut msg = Self::default();
                    while !r.is_eof() {
                        match r.next_tag(bytes) {
                Ok(10) => msg.affected_path = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.param_errs.push(r.read_message::<usp::mod_SetResp::mod_UpdatedObjectResult::mod_OperationStatus::ParameterError>(bytes)?),
                Ok(26) => {
                    let (key, value) = r.read_map(bytes, |r, bytes| r.read_string(bytes).map(Cow::Borrowed), |r, bytes| r.read_string(bytes).map(Cow::Borrowed))?;
                    msg.updated_params.insert(key, value);
                }
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
                    }
                    Ok(msg)
                }
            }

            impl<'a> MessageWrite for UpdatedInstanceResult<'a> {
                fn get_size(&self) -> usize {
                    0 + self
                        .affected_path
                        .as_ref()
                        .map_or(0, |m| 1 + sizeof_len((m).len()))
                        + self
                            .param_errs
                            .iter()
                            .map(|s| 1 + sizeof_len((s).get_size()))
                            .sum::<usize>()
                        + self
                            .updated_params
                            .iter()
                            .map(|(k, v)| {
                                1 + sizeof_len(2 + sizeof_len((k).len()) + sizeof_len((v).len()))
                            })
                            .sum::<usize>()
                }

                fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
                    if let Some(ref s) = self.affected_path {
                        w.write_with_tag(10, |w| w.write_string(&**s))?;
                    }
                    for s in &self.param_errs {
                        w.write_with_tag(18, |w| w.write_message(s))?;
                    }
                    for (k, v) in self.updated_params.iter() {
                        w.write_with_tag(26, |w| {
                            w.write_map(
                                2 + sizeof_len((k).len()) + sizeof_len((v).len()),
                                10,
                                |w| w.write_string(&**k),
                                18,
                                |w| w.write_string(&**v),
                            )
                        })?;
                    }
                    Ok(())
                }
            }

            #[derive(Debug, Default, PartialEq, Clone)]
            pub struct ParameterError<'a> {
                pub param: Option<Cow<'a, str>>,
                pub err_code: Option<u32>,
                pub err_msg: Option<Cow<'a, str>>,
            }

            impl<'a> MessageRead<'a> for ParameterError<'a> {
                fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
                    let mut msg = Self::default();
                    while !r.is_eof() {
                        match r.next_tag(bytes) {
                            Ok(10) => msg.param = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                            Ok(21) => msg.err_code = Some(r.read_fixed32(bytes)?),
                            Ok(26) => msg.err_msg = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                            Ok(t) => {
                                r.read_unknown(bytes, t)?;
                            }
                            Err(e) => return Err(e),
                        }
                    }
                    Ok(msg)
                }
            }

            impl<'a> MessageWrite for ParameterError<'a> {
                fn get_size(&self) -> usize {
                    0 + self.param.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
                        + self.err_code.as_ref().map_or(0, |_| 1 + 4)
                        + self
                            .err_msg
                            .as_ref()
                            .map_or(0, |m| 1 + sizeof_len((m).len()))
                }

                fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
                    if let Some(ref s) = self.param {
                        w.write_with_tag(10, |w| w.write_string(&**s))?;
                    }
                    if let Some(ref s) = self.err_code {
                        w.write_with_tag(21, |w| w.write_fixed32(*s))?;
                    }
                    if let Some(ref s) = self.err_msg {
                        w.write_with_tag(26, |w| w.write_string(&**s))?;
                    }
                    Ok(())
                }
            }

            #[derive(Debug, PartialEq, Clone)]
            pub enum OneOfoper_status<'a> {
                
                
                
                
    oper_failure(usp::mod_SetResp::mod_UpdatedObjectResult::mod_OperationStatus::OperationFailure<'a>),
    oper_success(usp::mod_SetResp::mod_UpdatedObjectResult::mod_OperationStatus::OperationSuccess<'a>),
    None,
}

            impl<'a> Default for OneOfoper_status<'a> {
                fn default() -> Self {
                    OneOfoper_status::None
                }
            }

        }

    }

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Operate<'a> {
    pub command: Option<Cow<'a, str>>,
    pub command_key: Option<Cow<'a, str>>,
    pub send_resp: Option<bool>,
    pub input_args: HashMap<Cow<'a, str>, Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for Operate<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.command = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.command_key = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(24) => msg.send_resp = Some(r.read_bool(bytes)?),
                Ok(34) => {
                    let (key, value) = r.read_map(
                        bytes,
                        |r, bytes| r.read_string(bytes).map(Cow::Borrowed),
                        |r, bytes| r.read_string(bytes).map(Cow::Borrowed),
                    )?;
                    msg.input_args.insert(key, value);
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

impl<'a> MessageWrite for Operate<'a> {
    fn get_size(&self) -> usize {
        0 + self
            .command
            .as_ref()
            .map_or(0, |m| 1 + sizeof_len((m).len()))
            + self
                .command_key
                .as_ref()
                .map_or(0, |m| 1 + sizeof_len((m).len()))
            + self
                .send_resp
                .as_ref()
                .map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
            + self
                .input_args
                .iter()
                .map(|(k, v)| 1 + sizeof_len(2 + sizeof_len((k).len()) + sizeof_len((v).len())))
                .sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.command {
            w.write_with_tag(10, |w| w.write_string(&**s))?;
        }
        if let Some(ref s) = self.command_key {
            w.write_with_tag(18, |w| w.write_string(&**s))?;
        }
        if let Some(ref s) = self.send_resp {
            w.write_with_tag(24, |w| w.write_bool(*s))?;
        }
        for (k, v) in self.input_args.iter() {
            w.write_with_tag(34, |w| {
                w.write_map(
                    2 + sizeof_len((k).len()) + sizeof_len((v).len()),
                    10,
                    |w| w.write_string(&**k),
                    18,
                    |w| w.write_string(&**v),
                )
            })?;
        }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct OperateResp<'a> {
    pub operation_results: Vec<usp::mod_OperateResp::OperationResult<'a>>,
}

impl<'a> MessageRead<'a> for OperateResp<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg
                    .operation_results
                    .push(r.read_message::<usp::mod_OperateResp::OperationResult>(bytes)?),
                Ok(t) => {
                    r.read_unknown(bytes, t)?;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for OperateResp<'a> {
    fn get_size(&self) -> usize {
        0 + self
            .operation_results
            .iter()
            .map(|s| 1 + sizeof_len((s).get_size()))
            .sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.operation_results {
            w.write_with_tag(10, |w| w.write_message(s))?;
        }
        Ok(())
    }
}

pub mod mod_OperateResp {

    use super::*;
    use std::borrow::Cow;

    #[derive(Debug, Default, PartialEq, Clone)]
    pub struct OperationResult<'a> {
        pub executed_command: Option<Cow<'a, str>>,
        pub operation_resp: usp::mod_OperateResp::mod_OperationResult::OneOfoperation_resp<'a>,
    }

    impl<'a> MessageRead<'a> for OperationResult<'a> {
        fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
            let mut msg = Self::default();
            while !r.is_eof() {
                match r.next_tag(bytes) {
                Ok(10) => msg.executed_command = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.operation_resp = usp::mod_OperateResp::mod_OperationResult::OneOfoperation_resp::req_obj_path(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.operation_resp = usp::mod_OperateResp::mod_OperationResult::OneOfoperation_resp::req_output_args(r.read_message::<usp::mod_OperateResp::mod_OperationResult::OutputArgs>(bytes)?),
                Ok(34) => msg.operation_resp = usp::mod_OperateResp::mod_OperationResult::OneOfoperation_resp::cmd_failure(r.read_message::<usp::mod_OperateResp::mod_OperationResult::CommandFailure>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
            }
            Ok(msg)
        }
    }

    impl<'a> MessageWrite for OperationResult<'a> {
        fn get_size(&self) -> usize {
            0 + self
                .executed_command
                .as_ref()
                .map_or(0, |m| 1 + sizeof_len((m).len())) + match self.operation_resp {
                usp::mod_OperateResp::mod_OperationResult::OneOfoperation_resp::req_obj_path(
                    ref m,
                ) => 1 + sizeof_len((m).len()),
                usp::mod_OperateResp::mod_OperationResult::OneOfoperation_resp::req_output_args(
                    ref m,
                ) => 1 + sizeof_len((m).get_size()),
                usp::mod_OperateResp::mod_OperationResult::OneOfoperation_resp::cmd_failure(
                    ref m,
                ) => 1 + sizeof_len((m).get_size()),
                usp::mod_OperateResp::mod_OperationResult::OneOfoperation_resp::None => 0,
            }
        }

        fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
            if let Some(ref s) = self.executed_command {
                w.write_with_tag(10, |w| w.write_string(&**s))?;
            }
            match self.operation_resp {
                usp::mod_OperateResp::mod_OperationResult::OneOfoperation_resp::req_obj_path(
                    ref m,
                ) => w.write_with_tag(18, |w| w.write_string(&**m))?,
                usp::mod_OperateResp::mod_OperationResult::OneOfoperation_resp::req_output_args(
                    ref m,
                ) => w.write_with_tag(26, |w| w.write_message(m))?,
                usp::mod_OperateResp::mod_OperationResult::OneOfoperation_resp::cmd_failure(
                    ref m,
                ) => w.write_with_tag(34, |w| w.write_message(m))?,
                usp::mod_OperateResp::mod_OperationResult::OneOfoperation_resp::None => {}
            }
            Ok(())
        }
    }

    pub mod mod_OperationResult {

        use super::*;
        use std::borrow::Cow;
        use std::collections::HashMap;

        #[derive(Debug, Default, PartialEq, Clone)]
        pub struct OutputArgs<'a> {
            pub output_args: HashMap<Cow<'a, str>, Cow<'a, str>>,
        }

        impl<'a> MessageRead<'a> for OutputArgs<'a> {
            fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
                let mut msg = Self::default();
                while !r.is_eof() {
                    match r.next_tag(bytes) {
                        Ok(10) => {
                            let (key, value) = r.read_map(
                                bytes,
                                |r, bytes| r.read_string(bytes).map(Cow::Borrowed),
                                |r, bytes| r.read_string(bytes).map(Cow::Borrowed),
                            )?;
                            msg.output_args.insert(key, value);
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

        impl<'a> MessageWrite for OutputArgs<'a> {
            fn get_size(&self) -> usize {
                0 + self
                    .output_args
                    .iter()
                    .map(|(k, v)| 1 + sizeof_len(2 + sizeof_len((k).len()) + sizeof_len((v).len())))
                    .sum::<usize>()
            }

            fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
                for (k, v) in self.output_args.iter() {
                    w.write_with_tag(10, |w| {
                        w.write_map(
                            2 + sizeof_len((k).len()) + sizeof_len((v).len()),
                            10,
                            |w| w.write_string(&**k),
                            18,
                            |w| w.write_string(&**v),
                        )
                    })?;
                }
                Ok(())
            }
        }

        #[derive(Debug, Default, PartialEq, Clone)]
        pub struct CommandFailure<'a> {
            pub err_code: Option<u32>,
            pub err_msg: Option<Cow<'a, str>>,
        }

        impl<'a> MessageRead<'a> for CommandFailure<'a> {
            fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
                let mut msg = Self::default();
                while !r.is_eof() {
                    match r.next_tag(bytes) {
                        Ok(13) => msg.err_code = Some(r.read_fixed32(bytes)?),
                        Ok(18) => msg.err_msg = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                        Ok(t) => {
                            r.read_unknown(bytes, t)?;
                        }
                        Err(e) => return Err(e),
                    }
                }
                Ok(msg)
            }
        }

        impl<'a> MessageWrite for CommandFailure<'a> {
            fn get_size(&self) -> usize {
                0 + self.err_code.as_ref().map_or(0, |_| 1 + 4) + self
                    .err_msg
                    .as_ref()
                    .map_or(0, |m| 1 + sizeof_len((m).len()))
            }

            fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
                if let Some(ref s) = self.err_code {
                    w.write_with_tag(13, |w| w.write_fixed32(*s))?;
                }
                if let Some(ref s) = self.err_msg {
                    w.write_with_tag(18, |w| w.write_string(&**s))?;
                }
                Ok(())
            }
        }

        #[derive(Debug, PartialEq, Clone)]
        pub enum OneOfoperation_resp<'a> {
            req_obj_path(Cow<'a, str>),
            req_output_args(usp::mod_OperateResp::mod_OperationResult::OutputArgs<'a>),
            cmd_failure(usp::mod_OperateResp::mod_OperationResult::CommandFailure<'a>),
            None,
        }

        impl<'a> Default for OneOfoperation_resp<'a> {
            fn default() -> Self {
                OneOfoperation_resp::None
            }
        }

    }

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Notify<'a> {
    pub subscription_id: Option<Cow<'a, str>>,
    pub send_resp: Option<bool>,
    pub notification: usp::mod_Notify::OneOfnotification<'a>,
}

impl<'a> MessageRead<'a> for Notify<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.subscription_id = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(16) => msg.send_resp = Some(r.read_bool(bytes)?),
                Ok(26) => {
                    msg.notification = usp::mod_Notify::OneOfnotification::event(
                        r.read_message::<usp::mod_Notify::Event>(bytes)?,
                    )
                }
                Ok(34) => {
                    msg.notification = usp::mod_Notify::OneOfnotification::value_change(
                        r.read_message::<usp::mod_Notify::ValueChange>(bytes)?,
                    )
                }
                Ok(42) => {
                    msg.notification = usp::mod_Notify::OneOfnotification::obj_creation(
                        r.read_message::<usp::mod_Notify::ObjectCreation>(bytes)?,
                    )
                }
                Ok(50) => {
                    msg.notification = usp::mod_Notify::OneOfnotification::obj_deletion(
                        r.read_message::<usp::mod_Notify::ObjectDeletion>(bytes)?,
                    )
                }
                Ok(58) => {
                    msg.notification = usp::mod_Notify::OneOfnotification::oper_complete(
                        r.read_message::<usp::mod_Notify::OperationComplete>(bytes)?,
                    )
                }
                Ok(66) => {
                    msg.notification = usp::mod_Notify::OneOfnotification::on_board_req(
                        r.read_message::<usp::mod_Notify::OnBoardRequest>(bytes)?,
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

impl<'a> MessageWrite for Notify<'a> {
    fn get_size(&self) -> usize {
        0 + self
            .subscription_id
            .as_ref()
            .map_or(0, |m| 1 + sizeof_len((m).len()))
            + self
                .send_resp
                .as_ref()
                .map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
            + match self.notification {
                usp::mod_Notify::OneOfnotification::event(ref m) => 1 + sizeof_len((m).get_size()),
                usp::mod_Notify::OneOfnotification::value_change(ref m) => {
                    1 + sizeof_len((m).get_size())
                }
                usp::mod_Notify::OneOfnotification::obj_creation(ref m) => {
                    1 + sizeof_len((m).get_size())
                }
                usp::mod_Notify::OneOfnotification::obj_deletion(ref m) => {
                    1 + sizeof_len((m).get_size())
                }
                usp::mod_Notify::OneOfnotification::oper_complete(ref m) => {
                    1 + sizeof_len((m).get_size())
                }
                usp::mod_Notify::OneOfnotification::on_board_req(ref m) => {
                    1 + sizeof_len((m).get_size())
                }
                usp::mod_Notify::OneOfnotification::None => 0,
            }
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.subscription_id {
            w.write_with_tag(10, |w| w.write_string(&**s))?;
        }
        if let Some(ref s) = self.send_resp {
            w.write_with_tag(16, |w| w.write_bool(*s))?;
        }
        match self.notification {
            usp::mod_Notify::OneOfnotification::event(ref m) => {
                w.write_with_tag(26, |w| w.write_message(m))?
            }
            usp::mod_Notify::OneOfnotification::value_change(ref m) => {
                w.write_with_tag(34, |w| w.write_message(m))?
            }
            usp::mod_Notify::OneOfnotification::obj_creation(ref m) => {
                w.write_with_tag(42, |w| w.write_message(m))?
            }
            usp::mod_Notify::OneOfnotification::obj_deletion(ref m) => {
                w.write_with_tag(50, |w| w.write_message(m))?
            }
            usp::mod_Notify::OneOfnotification::oper_complete(ref m) => {
                w.write_with_tag(58, |w| w.write_message(m))?
            }
            usp::mod_Notify::OneOfnotification::on_board_req(ref m) => {
                w.write_with_tag(66, |w| w.write_message(m))?
            }
            usp::mod_Notify::OneOfnotification::None => {}
        }
        Ok(())
    }
}

pub mod mod_Notify {

    use super::*;
    use std::borrow::Cow;
    use std::collections::HashMap;

    #[derive(Debug, Default, PartialEq, Clone)]
    pub struct Event<'a> {
        pub obj_path: Option<Cow<'a, str>>,
        pub event_name: Option<Cow<'a, str>>,
        pub params: HashMap<Cow<'a, str>, Cow<'a, str>>,
    }

    impl<'a> MessageRead<'a> for Event<'a> {
        fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
            let mut msg = Self::default();
            while !r.is_eof() {
                match r.next_tag(bytes) {
                    Ok(10) => msg.obj_path = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                    Ok(18) => msg.event_name = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                    Ok(26) => {
                        let (key, value) = r.read_map(
                            bytes,
                            |r, bytes| r.read_string(bytes).map(Cow::Borrowed),
                            |r, bytes| r.read_string(bytes).map(Cow::Borrowed),
                        )?;
                        msg.params.insert(key, value);
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

    impl<'a> MessageWrite for Event<'a> {
        fn get_size(&self) -> usize {
            0 + self
                .obj_path
                .as_ref()
                .map_or(0, |m| 1 + sizeof_len((m).len()))
                + self
                    .event_name
                    .as_ref()
                    .map_or(0, |m| 1 + sizeof_len((m).len()))
                + self
                    .params
                    .iter()
                    .map(|(k, v)| 1 + sizeof_len(2 + sizeof_len((k).len()) + sizeof_len((v).len())))
                    .sum::<usize>()
        }

        fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
            if let Some(ref s) = self.obj_path {
                w.write_with_tag(10, |w| w.write_string(&**s))?;
            }
            if let Some(ref s) = self.event_name {
                w.write_with_tag(18, |w| w.write_string(&**s))?;
            }
            for (k, v) in self.params.iter() {
                w.write_with_tag(26, |w| {
                    w.write_map(
                        2 + sizeof_len((k).len()) + sizeof_len((v).len()),
                        10,
                        |w| w.write_string(&**k),
                        18,
                        |w| w.write_string(&**v),
                    )
                })?;
            }
            Ok(())
        }
    }

    #[derive(Debug, Default, PartialEq, Clone)]
    pub struct ValueChange<'a> {
        pub param_path: Option<Cow<'a, str>>,
        pub param_value: Option<Cow<'a, str>>,
    }

    impl<'a> MessageRead<'a> for ValueChange<'a> {
        fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
            let mut msg = Self::default();
            while !r.is_eof() {
                match r.next_tag(bytes) {
                    Ok(10) => msg.param_path = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                    Ok(18) => msg.param_value = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                    Ok(t) => {
                        r.read_unknown(bytes, t)?;
                    }
                    Err(e) => return Err(e),
                }
            }
            Ok(msg)
        }
    }

    impl<'a> MessageWrite for ValueChange<'a> {
        fn get_size(&self) -> usize {
            0 + self
                .param_path
                .as_ref()
                .map_or(0, |m| 1 + sizeof_len((m).len()))
                + self
                    .param_value
                    .as_ref()
                    .map_or(0, |m| 1 + sizeof_len((m).len()))
        }

        fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
            if let Some(ref s) = self.param_path {
                w.write_with_tag(10, |w| w.write_string(&**s))?;
            }
            if let Some(ref s) = self.param_value {
                w.write_with_tag(18, |w| w.write_string(&**s))?;
            }
            Ok(())
        }
    }

    #[derive(Debug, Default, PartialEq, Clone)]
    pub struct ObjectCreation<'a> {
        pub obj_path: Option<Cow<'a, str>>,
        pub unique_keys: HashMap<Cow<'a, str>, Cow<'a, str>>,
    }

    impl<'a> MessageRead<'a> for ObjectCreation<'a> {
        fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
            let mut msg = Self::default();
            while !r.is_eof() {
                match r.next_tag(bytes) {
                    Ok(10) => msg.obj_path = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                    Ok(18) => {
                        let (key, value) = r.read_map(
                            bytes,
                            |r, bytes| r.read_string(bytes).map(Cow::Borrowed),
                            |r, bytes| r.read_string(bytes).map(Cow::Borrowed),
                        )?;
                        msg.unique_keys.insert(key, value);
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

    impl<'a> MessageWrite for ObjectCreation<'a> {
        fn get_size(&self) -> usize {
            0 + self
                .obj_path
                .as_ref()
                .map_or(0, |m| 1 + sizeof_len((m).len()))
                + self
                    .unique_keys
                    .iter()
                    .map(|(k, v)| 1 + sizeof_len(2 + sizeof_len((k).len()) + sizeof_len((v).len())))
                    .sum::<usize>()
        }

        fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
            if let Some(ref s) = self.obj_path {
                w.write_with_tag(10, |w| w.write_string(&**s))?;
            }
            for (k, v) in self.unique_keys.iter() {
                w.write_with_tag(18, |w| {
                    w.write_map(
                        2 + sizeof_len((k).len()) + sizeof_len((v).len()),
                        10,
                        |w| w.write_string(&**k),
                        18,
                        |w| w.write_string(&**v),
                    )
                })?;
            }
            Ok(())
        }
    }

    #[derive(Debug, Default, PartialEq, Clone)]
    pub struct ObjectDeletion<'a> {
        pub obj_path: Option<Cow<'a, str>>,
    }

    impl<'a> MessageRead<'a> for ObjectDeletion<'a> {
        fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
            let mut msg = Self::default();
            while !r.is_eof() {
                match r.next_tag(bytes) {
                    Ok(10) => msg.obj_path = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                    Ok(t) => {
                        r.read_unknown(bytes, t)?;
                    }
                    Err(e) => return Err(e),
                }
            }
            Ok(msg)
        }
    }

    impl<'a> MessageWrite for ObjectDeletion<'a> {
        fn get_size(&self) -> usize {
            0 + self
                .obj_path
                .as_ref()
                .map_or(0, |m| 1 + sizeof_len((m).len()))
        }

        fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
            if let Some(ref s) = self.obj_path {
                w.write_with_tag(10, |w| w.write_string(&**s))?;
            }
            Ok(())
        }
    }

    #[derive(Debug, Default, PartialEq, Clone)]
    pub struct OperationComplete<'a> {
        pub obj_path: Option<Cow<'a, str>>,
        pub command_name: Option<Cow<'a, str>>,
        pub command_key: Option<Cow<'a, str>>,
        pub operation_resp: usp::mod_Notify::mod_OperationComplete::OneOfoperation_resp<'a>,
    }

    impl<'a> MessageRead<'a> for OperationComplete<'a> {
        fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
            let mut msg = Self::default();
            while !r.is_eof() {
                match r.next_tag(bytes) {
                Ok(10) => msg.obj_path = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.command_name = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.command_key = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(34) => msg.operation_resp = usp::mod_Notify::mod_OperationComplete::OneOfoperation_resp::req_output_args(r.read_message::<usp::mod_Notify::mod_OperationComplete::OutputArgs>(bytes)?),
                Ok(42) => msg.operation_resp = usp::mod_Notify::mod_OperationComplete::OneOfoperation_resp::cmd_failure(r.read_message::<usp::mod_Notify::mod_OperationComplete::CommandFailure>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
            }
            Ok(msg)
        }
    }

    impl<'a> MessageWrite for OperationComplete<'a> {
        fn get_size(&self) -> usize {
            0
        + self.obj_path.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.command_name.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.command_key.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + match self.operation_resp {
            usp::mod_Notify::mod_OperationComplete::OneOfoperation_resp::req_output_args(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_Notify::mod_OperationComplete::OneOfoperation_resp::cmd_failure(ref m) => 1 + sizeof_len((m).get_size()),
            usp::mod_Notify::mod_OperationComplete::OneOfoperation_resp::None => 0,
    }
        }

        fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
            if let Some(ref s) = self.obj_path {
                w.write_with_tag(10, |w| w.write_string(&**s))?;
            }
            if let Some(ref s) = self.command_name {
                w.write_with_tag(18, |w| w.write_string(&**s))?;
            }
            if let Some(ref s) = self.command_key {
                w.write_with_tag(26, |w| w.write_string(&**s))?;
            }
            match self.operation_resp {
                usp::mod_Notify::mod_OperationComplete::OneOfoperation_resp::req_output_args(
                    ref m,
                ) => w.write_with_tag(34, |w| w.write_message(m))?,
                usp::mod_Notify::mod_OperationComplete::OneOfoperation_resp::cmd_failure(ref m) => {
                    w.write_with_tag(42, |w| w.write_message(m))?
                }
                usp::mod_Notify::mod_OperationComplete::OneOfoperation_resp::None => {}
            }
            Ok(())
        }
    }

    pub mod mod_OperationComplete {

        use super::*;
        use std::borrow::Cow;
        use std::collections::HashMap;

        #[derive(Debug, Default, PartialEq, Clone)]
        pub struct OutputArgs<'a> {
            pub output_args: HashMap<Cow<'a, str>, Cow<'a, str>>,
        }

        impl<'a> MessageRead<'a> for OutputArgs<'a> {
            fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
                let mut msg = Self::default();
                while !r.is_eof() {
                    match r.next_tag(bytes) {
                        Ok(10) => {
                            let (key, value) = r.read_map(
                                bytes,
                                |r, bytes| r.read_string(bytes).map(Cow::Borrowed),
                                |r, bytes| r.read_string(bytes).map(Cow::Borrowed),
                            )?;
                            msg.output_args.insert(key, value);
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

        impl<'a> MessageWrite for OutputArgs<'a> {
            fn get_size(&self) -> usize {
                0 + self
                    .output_args
                    .iter()
                    .map(|(k, v)| 1 + sizeof_len(2 + sizeof_len((k).len()) + sizeof_len((v).len())))
                    .sum::<usize>()
            }

            fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
                for (k, v) in self.output_args.iter() {
                    w.write_with_tag(10, |w| {
                        w.write_map(
                            2 + sizeof_len((k).len()) + sizeof_len((v).len()),
                            10,
                            |w| w.write_string(&**k),
                            18,
                            |w| w.write_string(&**v),
                        )
                    })?;
                }
                Ok(())
            }
        }

        #[derive(Debug, Default, PartialEq, Clone)]
        pub struct CommandFailure<'a> {
            pub err_code: Option<u32>,
            pub err_msg: Option<Cow<'a, str>>,
        }

        impl<'a> MessageRead<'a> for CommandFailure<'a> {
            fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
                let mut msg = Self::default();
                while !r.is_eof() {
                    match r.next_tag(bytes) {
                        Ok(13) => msg.err_code = Some(r.read_fixed32(bytes)?),
                        Ok(18) => msg.err_msg = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                        Ok(t) => {
                            r.read_unknown(bytes, t)?;
                        }
                        Err(e) => return Err(e),
                    }
                }
                Ok(msg)
            }
        }

        impl<'a> MessageWrite for CommandFailure<'a> {
            fn get_size(&self) -> usize {
                0 + self.err_code.as_ref().map_or(0, |_| 1 + 4) + self
                    .err_msg
                    .as_ref()
                    .map_or(0, |m| 1 + sizeof_len((m).len()))
            }

            fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
                if let Some(ref s) = self.err_code {
                    w.write_with_tag(13, |w| w.write_fixed32(*s))?;
                }
                if let Some(ref s) = self.err_msg {
                    w.write_with_tag(18, |w| w.write_string(&**s))?;
                }
                Ok(())
            }
        }

        #[derive(Debug, PartialEq, Clone)]
        pub enum OneOfoperation_resp<'a> {
            req_output_args(usp::mod_Notify::mod_OperationComplete::OutputArgs<'a>),
            cmd_failure(usp::mod_Notify::mod_OperationComplete::CommandFailure<'a>),
            None,
        }

        impl<'a> Default for OneOfoperation_resp<'a> {
            fn default() -> Self {
                OneOfoperation_resp::None
            }
        }

    }

    #[derive(Debug, Default, PartialEq, Clone)]
    pub struct OnBoardRequest<'a> {
        pub oui: Option<Cow<'a, str>>,
        pub product_class: Option<Cow<'a, str>>,
        pub serial_number: Option<Cow<'a, str>>,
        pub agent_supported_protocol_versions: Option<Cow<'a, str>>,
    }

    impl<'a> MessageRead<'a> for OnBoardRequest<'a> {
        fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
            let mut msg = Self::default();
            while !r.is_eof() {
                match r.next_tag(bytes) {
                    Ok(10) => msg.oui = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                    Ok(18) => msg.product_class = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                    Ok(26) => msg.serial_number = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                    Ok(34) => {
                        msg.agent_supported_protocol_versions =
                            Some(r.read_string(bytes).map(Cow::Borrowed)?)
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

    impl<'a> MessageWrite for OnBoardRequest<'a> {
        fn get_size(&self) -> usize {
            0 + self.oui.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
                + self
                    .product_class
                    .as_ref()
                    .map_or(0, |m| 1 + sizeof_len((m).len()))
                + self
                    .serial_number
                    .as_ref()
                    .map_or(0, |m| 1 + sizeof_len((m).len()))
                + self
                    .agent_supported_protocol_versions
                    .as_ref()
                    .map_or(0, |m| 1 + sizeof_len((m).len()))
        }

        fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
            if let Some(ref s) = self.oui {
                w.write_with_tag(10, |w| w.write_string(&**s))?;
            }
            if let Some(ref s) = self.product_class {
                w.write_with_tag(18, |w| w.write_string(&**s))?;
            }
            if let Some(ref s) = self.serial_number {
                w.write_with_tag(26, |w| w.write_string(&**s))?;
            }
            if let Some(ref s) = self.agent_supported_protocol_versions {
                w.write_with_tag(34, |w| w.write_string(&**s))?;
            }
            Ok(())
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum OneOfnotification<'a> {
        event(usp::mod_Notify::Event<'a>),
        value_change(usp::mod_Notify::ValueChange<'a>),
        obj_creation(usp::mod_Notify::ObjectCreation<'a>),
        obj_deletion(usp::mod_Notify::ObjectDeletion<'a>),
        oper_complete(usp::mod_Notify::OperationComplete<'a>),
        on_board_req(usp::mod_Notify::OnBoardRequest<'a>),
        None,
    }

    impl<'a> Default for OneOfnotification<'a> {
        fn default() -> Self {
            OneOfnotification::None
        }
    }

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct NotifyResp<'a> {
    pub subscription_id: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for NotifyResp<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.subscription_id = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => {
                    r.read_unknown(bytes, t)?;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for NotifyResp<'a> {
    fn get_size(&self) -> usize {
        0 + self
            .subscription_id
            .as_ref()
            .map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.subscription_id {
            w.write_with_tag(10, |w| w.write_string(&**s))?;
        }
        Ok(())
    }
}
