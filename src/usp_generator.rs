use std::borrow::Cow;
use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};

use crate::usp::{Body, Error, Get, Header, Msg, Notify, Request, Response};
use crate::usp_record::Record;
use crate::usp_types::NotifyType;

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
    use crate::usp::mod_Body::OneOfmsg_body::*;
    use crate::usp::mod_Header::MsgType::*;
    use crate::usp::mod_Request::OneOfreq_type::*;
    use crate::usp::mod_Response::OneOfresp_type::*;

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

/// Creates a body for a USP Msg with an USP Error
///
/// # Arguments
///
/// * `code` - The USP error code, MUST be between 7000 and 7999
/// * `message` - A `Option<String>` containing the user readable messge. Will be automatically
///               filled in for standard error codes if not supplied
///
/// # Examples
///
/// ```
/// use rusp::usp_generator::usp_simple_error;
/// let err = usp_simple_error(7001, None);
/// ```
///
/// ```
/// use rusp::usp_generator::usp_simple_error;
/// let err = usp_simple_error(7803, Some("Funny custom vendor error".into()));
/// ```
///
/// ```should_panic
/// use rusp::usp_generator::usp_simple_error;
/// let err = usp_simple_error(8000, None);
/// ```
pub fn usp_simple_error<'a>(code: u32, message: Option<String>) -> Body<'a> {
    use crate::usp::mod_Body::OneOfmsg_body::*;

    let err_msg = message.unwrap_or_else(|| {
        match code {
            7000 => "Message failed",
            7001 => "Message not supported",
            7002 => "Request denied (no reason specified)",
            7003 => "Internal error",
            7004 => "Invalid arguments",
            7005 => "Resources exceeded",
            7006 => "Permission denied",
            7007 => "Invalid configuration",
            7008 => "Invalid path syntax",
            7009 => "Parameter action failed",
            7010 => "Unsupported parameter",
            7011 => "Invalid type",
            7012 => "Invalid value",
            7013 => "Attempt to update non-writeable parameter",
            7014 => "Value conflict",
            7015 => "Operation error",
            7016 => "Object does not exist",
            7017 => "Object could not be created",
            7018 => "Object is not a table",
            7019 => "Attempt to create non-creatable Object",
            7020 => "Object could not be updated",
            7021 => "Required parameter failed",
            7022 => "Command failure",
            7023 => "Command canceled",
            7024 => "Delete failure",
            7025 => "Object exists with duplicate key",
            7026 => "Invalid path",
            7027 => "Invalid Command Arguments",
            7800..=7999 => "Vendor specific",
            _ => unreachable!(),
        }
        .to_string()
    });

    Body {
        msg_body: error({
            Error {
                err_code: Some(code),
                err_msg: Some(err_msg.into()),
                param_errs: [].to_vec(),
            }
        }),
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
    use crate::usp::mod_Body::OneOfmsg_body::*;
    use crate::usp::mod_Request::OneOfreq_type::*;

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

/// Wraps the body of a USP Msg with a USP Notify request
///
/// # Arguments
///
/// * `sub_id` - The subscription_id for the Notify
/// * `send_resp` - Whether this requests expects a response to be sent
/// * `typ` - A filled out `NotifyType` structure
///
/// # Example
///
/// ```
/// use rusp::usp_types::NotifyType;
/// use rusp::usp_generator::usp_notify_request;
/// let req = usp_notify_request("", true, NotifyType::OnBoardRequest {
///     oui: "ABCABC".to_string(),
///     product_class: "PC".to_string(),
///     serial_number: "000000".to_string(),
///     agent_supported_protocol_versions: "1.0".to_string()
/// });
/// ```
pub fn usp_notify_request(sub_id: &'_ str, send_resp: bool, typ: NotifyType) -> Body<'_> {
    use crate::usp::mod_Body::OneOfmsg_body::*;
    use crate::usp::mod_Notify::OnBoardRequest;
    use crate::usp::mod_Notify::OneOfnotification::*;
    use crate::usp::mod_Request::OneOfreq_type::*;

    Body {
        msg_body: request({
            Request {
                req_type: notify({
                    let mut notr = Notify::default();
                    notr.subscription_id = Some(sub_id.into());
                    notr.send_resp = Some(send_resp);
                    notr.notification = match typ {
                        NotifyType::OnBoardRequest {
                            oui,
                            product_class,
                            serial_number,
                            agent_supported_protocol_versions,
                        } => on_board_req(OnBoardRequest {
                            agent_supported_protocol_versions: Some(
                                agent_supported_protocol_versions.into(),
                            ),
                            oui: Some(oui.into()),
                            product_class: Some(product_class.into()),
                            serial_number: Some(serial_number.into()),
                        }),
                    };
                    notr
                }),
            }
        }),
    }
}

/// Creates a body for a USP Msg with a USP GetResp response
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
    use crate::usp::mod_Body::OneOfmsg_body::*;
    use crate::usp::mod_GetResp::{RequestedPathResult, ResolvedPathResult};
    use crate::usp::mod_Response::OneOfresp_type::*;
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

/// Creates a body for a USP Msg with a USP GetResp response
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

/// Wraps a Usp Msg into an "no_session_context" USP Record with the specified record information
///
/// # Arguments
///
/// * `msg_id` - The message ID to put into the USP Msg
/// * `body` - The message body USP Msg
///
/// # Example
///
/// ```
/// use rusp::usp_generator::{usp_no_session_context_record};
/// let newrecord = usp_no_session_context_record(
///     None,
///     Some("proto::myfancyrecipient"),
///     Some("proto::anonymous"),
///     &[],
/// );
/// ```
pub fn usp_no_session_context_record<'a>(
    version: Option<&'a str>,
    to_id: Option<&'a str>,
    from_id: Option<&'a str>,
    msg: &'a [u8],
) -> Record<'a> {
    use crate::usp_record::mod_Record::OneOfrecord_type::no_session_context;
    use crate::usp_record::NoSessionContextRecord;

    let version = version.or(Some("1.0"));
    let to_id = to_id.or(Some("proto::to_rusp"));
    let from_id = from_id.or(Some("proto::from_rusp"));

    Record {
        version: version.map(|v| v.into()),
        to_id: to_id.map(|v| v.into()),
        from_id: from_id.map(|v| v.into()),
        sender_cert: None,
        mac_signature: None,
        payload_security: None,
        record_type: no_session_context(NoSessionContextRecord {
            payload: Some(msg.into()),
        }),
    }
}

/// Creates a body for a USP Msg with a USP NotifyResp response
///
/// # Arguments
///
/// * `result` - A vector of Result tuples to put into the NotifyResp response
///
/// # Example
///
/// ```
/// use rusp::usp_generator::usp_notify_response;
/// let resp = usp_notify_response("fancy_sub_id");
/// ```
pub fn usp_notify_response(subscription_id: &'_ str) -> Body<'_> {
    use crate::usp::mod_Body::OneOfmsg_body::*;
    use crate::usp::mod_Response::OneOfresp_type::*;
    use crate::usp::NotifyResp;

    Body {
        msg_body: response({
            Response {
                resp_type: notify_resp({
                    NotifyResp {
                        subscription_id: Some(Cow::Borrowed(subscription_id)),
                    }
                }),
            }
        }),
    }
}
