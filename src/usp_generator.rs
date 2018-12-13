use std::borrow::Cow;

use crate::usp::{Body, Get, Header, Msg, Request};

use crate::usp::mod_Body::OneOfmsg_body::*;
use crate::usp::mod_Request::OneOfreq_type::*;
use crate::usp::mod_Response::OneOfresp_type::*;


pub fn usp_msg<'a>(msg_id: &'a str, body: Body<'a>) -> Msg<'a> {
    use crate::usp::mod_Header::MsgType::*;

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
            _ => unreachable!(),
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
            _ => unreachable!(),
        },
        error(_) => ERROR,
        _ => ERROR,
    };

    Msg {
        header: Some(Header {
            msg_id: Some(std::borrow::Cow::Borrowed(msg_id)),
            msg_type: Some(msg_type),
        }),
        body: Some(body),
    }
}

pub fn usp_get_request(params: Vec<&str>) -> Body {
    Body {
        msg_body: request({
            Request {
                req_type: get({
                    let mut getr = Get::default();
                    for path in params {
                        getr.param_paths.push(Cow::Borrowed(path));
                    }
                    getr
                }),
            }
        }),
    }
}
