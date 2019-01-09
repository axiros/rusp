use std::borrow::Cow;
use std::collections::HashMap;

use serde;
use serde_derive::{Deserialize, Serialize};

use crate::usp::{Body, Get, Header, Msg, Request, Response};

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
///     "fancymsgid".to_string(),
///     usp_get_request(&["Device.", "Device.DeviceInfo."]),
/// );
/// ```
pub fn usp_msg(msg_id: String, body: Body) -> Msg {
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
            msg_id: Some(std::borrow::Cow::from(msg_id)),
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
/// let req = usp_get_request(&["Device.", "Device.DeviceInfo."]);
/// ```
pub fn usp_get_request<'a>(params: &[&'a str]) -> Body<'a> {
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

/// Creates a body for USP Msg with a USP GetResp response
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
    use crate::usp::GetResp;
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
                    getr
                }),
            }
        }),
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct ResolvedPathResult<'a> {
    resolved_path: &'a str,
    result_params: HashMap<&'a str, &'a str>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestedPathResult<'a> {
    requested_path: &'a str,
    err_code: u32,
    err_msg: &'a str,
    resolved_path_results: Vec<ResolvedPathResult<'a>>,
}

pub type GetResp<'a> = Vec<RequestedPathResult<'a>>;

/// Creates a body for USP Msg with a USP GetResp response
///
/// # Arguments
///
/// * `result` - A vector of Result tuples to put into the GetResp response
///
/// # Example
///
/// ```
/// use rusp::usp_generator::{usp_get_response_from_json, GetResp};
/// let json = r#"[{"requested_path": "bar", "err_code" : 0, "err_msg" : "", "resolved_path_results" : [{"resolved_path": "Device.", "result_params": {"Device.Foo": "bar"}}]}]"#;
/// let deserialised : GetResp = serde_json::from_str(&json).unwrap();
/// let resp = usp_get_response_from_json(&deserialised);
/// ```
pub fn usp_get_response_from_json<'a>(getresp: &[RequestedPathResult<'a>]) -> Body<'a> {
    let mut d: Vec<(&str, Result<Vec<(&str, Vec<(&str, &str)>)>, (u32, &str)>)> =
        Default::default();
    for req_path_result in getresp {
        //let name = &req_path_result.requested_path;
        match req_path_result.err_code {
            0 => {
                let mut resolved_path_result: Vec<(&str, Vec<(&str, &str)>)> = Default::default();

                for res_path in &req_path_result.resolved_path_results {
                    resolved_path_result.push((
                        &res_path.resolved_path,
                        res_path
                            .result_params
                            .iter()
                            .map(|(k, v)| (*k, *v))
                            .collect(),
                    ));
                }
                d.push((&req_path_result.requested_path, Ok(resolved_path_result)))
            }

            _ => d.push((
                &req_path_result.requested_path,
                Err((req_path_result.err_code, &req_path_result.err_msg)),
            )),
        };
    }

    usp_get_response(d)
}
