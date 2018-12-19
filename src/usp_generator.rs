use std::borrow::Cow;

use crate::usp::{Body, Get, GetResp, Header, Msg, Request, Response};

use crate::usp::mod_Body::OneOfmsg_body::*;
use crate::usp::mod_Request::OneOfreq_type::*;
use crate::usp::mod_Response::OneOfresp_type::*;

/// Wraps the body of a USP Msg into a USP Msg with the specified message ID
///
/// # Arguments
///
/// * `msg_id` - The message ID to put into the USP Msg
/// * `body` - The message body USP Msg
///
/// # Example
///
/// ```
/// use rusp::usp_generator::{usp_msg, usp_get_request};
/// let newmsg = usp_msg(
///     &"fancymsgid",
///     usp_get_request(vec!["Device.", "Device.DeviceInfo."]),
/// );
/// ```
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

/// Wraps the body of a USP Msg with a USP Get request
///
/// # Arguments
///
/// * `params` - A vector of parameter/object names to put into the Get request
///
/// # Example
///
/// ```
/// use rusp::usp_generator::usp_get_request;
/// let req = usp_get_request(vec!["Device.", "Device.DeviceInfo."]);
/// ```
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

/// Wraps the body of a USP Msg with a USP GetResp response
///
/// # Arguments
///
/// * `result` - A vector of Result tuples to put into the GetResp response
///
/// # Example
///
/// ```
/// use rusp::usp_generator::usp_get_response;
/// let resp = usp_get_response(vec![
///         ("Device.", Ok(vec![("Device.", vec![("Foo", "Bar")])])),
///         ("Dev.", Err((7000, "Message failed"))),
///     ]);
/// ```
pub fn usp_get_response<'a>(
    result: Vec<(
        &'a str,
        Result<Vec<(&'a str, Vec<(&'a str, &'a str)>)>, (u32, &'a str)>,
    )>,
) -> Body<'a> {
    use crate::usp::mod_GetResp::{RequestedPathResult, ResolvedPathResult};
    use std::collections::HashMap;

    Body {
        msg_body: response({
            Response {
                resp_type: get_resp({
                    let mut getr = GetResp::default();
                    for (path, state) in result {
                        getr.req_path_results.push(match state {
                            Ok(success) => {
                                let mut respaths = Vec::default();
                                for (path, params) in success {
                                    respaths.push(ResolvedPathResult {
                                        resolved_path: Some(Cow::Borrowed(path)),
                                        result_params: params
                                            .into_iter()
                                            .map(|(k, v)| (Cow::Borrowed(k), Cow::Borrowed(v)))
                                            .collect::<HashMap<_, _>>(),
                                    });
                                }

                                RequestedPathResult {
                                    requested_path: Some(Cow::Borrowed(path)),
                                    err_code: Option::None,
                                    err_msg: Option::None,
                                    resolved_path_results: respaths,
                                }
                            }
                            Err(failure) => RequestedPathResult {
                                requested_path: Some(Cow::Borrowed(path)),
                                err_code: Some(failure.0),
                                err_msg: Some(Cow::Borrowed(failure.1)),
                                resolved_path_results: Vec::default(),
                            },
                        });
                    }
                    /*
                    for path in requested_paths {
                        getr.req_path_results.push(Cow::Borrowed(path));
                    }*/
                    getr
                }),
            }
        }),
    }
}
