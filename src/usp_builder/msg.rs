use crate::usp::mod_Body::OneOfmsg_body;
use crate::usp::mod_Request::OneOfreq_type;
use crate::usp::mod_Response::OneOfresp_type;
use crate::usp::Body;
use crate::usp::Header;
use crate::usp::Msg;

use anyhow::{Context, Result};

#[derive(Clone)]
pub struct MsgBuilder {
    msg_id: Option<String>,
    body: Option<Body>,
}

impl MsgBuilder {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            msg_id: None,
            body: None,
        }
    }

    #[must_use]
    pub fn with_msg_id(mut self, msg_id: String) -> Self {
        self.msg_id = Some(msg_id);
        self
    }

    #[must_use]
    pub fn with_body(mut self, body: Body) -> Self {
        self.body = Some(body);
        self
    }

    pub fn build(self) -> Result<Msg> {
        use crate::usp::mod_Body::OneOfmsg_body::{error, request, response};
        use crate::usp::mod_Header::MsgType::{
            ADD, ADD_RESP, DELETE, DELETE_RESP, DEREGISTER, DEREGISTER_RESP, ERROR, GET,
            GET_INSTANCES, GET_INSTANCES_RESP, GET_RESP, GET_SUPPORTED_DM, GET_SUPPORTED_DM_RESP,
            GET_SUPPORTED_PROTO, GET_SUPPORTED_PROTO_RESP, NOTIFY, NOTIFY_RESP, OPERATE,
            OPERATE_RESP, REGISTER, REGISTER_RESP, SET, SET_RESP,
        };
        use crate::usp::mod_Request::OneOfreq_type::{
            add, delete, deregister, get, get_instances, get_supported_dm, get_supported_protocol,
            notify, operate, register, set,
        };
        use crate::usp::mod_Response::OneOfresp_type::{
            add_resp, delete_resp, deregister_resp, get_instances_resp, get_resp,
            get_supported_dm_resp, get_supported_protocol_resp, notify_resp, operate_resp,
            register_resp, set_resp,
        };

        let msg_id = self
            .msg_id
            .with_context(|| "Cannot produce USP Msg without msg_id")?;
        let body = self
            .body
            .with_context(|| "Cannot produce USP Msg without msg_body")?;

        let msg_type = match &body.msg_body {
            request(ref req) => match &req.req_type {
                get(_) => GET,
                get_supported_dm(_) => GET_SUPPORTED_DM,
                get_instances(_) => GET_INSTANCES,
                set(_) => SET,
                add(_) => ADD,
                delete(_) => DELETE,
                operate(_) => OPERATE,
                notify(_) => NOTIFY,
                get_supported_protocol(_) => GET_SUPPORTED_PROTO,
                register(_) => REGISTER,
                deregister(_) => DEREGISTER,
                OneOfreq_type::None => ERROR,
            },
            response(ref resp) => match &resp.resp_type {
                get_resp(_) => GET_RESP,
                get_supported_dm_resp(_) => GET_SUPPORTED_DM_RESP,
                get_instances_resp(_) => GET_INSTANCES_RESP,
                set_resp(_) => SET_RESP,
                add_resp(_) => ADD_RESP,
                delete_resp(_) => DELETE_RESP,
                operate_resp(_) => OPERATE_RESP,
                notify_resp(_) => NOTIFY_RESP,
                get_supported_protocol_resp(_) => GET_SUPPORTED_PROTO_RESP,
                register_resp(_) => REGISTER_RESP,
                deregister_resp(_) => DEREGISTER_RESP,
                OneOfresp_type::None => ERROR,
            },
            error(_) | OneOfmsg_body::None => ERROR,
        };

        Ok(Msg {
            header: Some(Header { msg_id, msg_type }),
            body: Some(body),
        })
    }
}
