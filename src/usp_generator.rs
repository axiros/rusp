use std::borrow::Cow;
use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};

use crate::usp::{
    Add, Body, Delete, Error, Get, GetInstances, GetSupportedDM, GetSupportedProtocol, Header, Msg,
    Notify, Operate, Request, Response, Set,
};
use crate::usp_record::Record;
pub use crate::usp_types::{NotifyType, PayloadSARState, PayloadSecurity};

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
///     usp_get_request(&["Device.", "Device.DeviceInfo."], 0),
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
            msg_id: Cow::from(msg_id),
            msg_type,
        }),
        body: Some(body),
    }
}

/// Creates a body for a USP Msg with an USP Error
///
/// # Arguments
///
/// * `code` - The USP error code, MUST be between 7000 and 7999
/// * `message` - An `Option<&str>` containing the user readable message. Will be automatically
///               filled in for standard error codes if not supplied
///
/// # Panics
///
/// Panics if both `code` is an invalid USP error code and `message` is `None`
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
/// let err = usp_simple_error(7803, Some("Funny custom vendor error"));
/// ```
///
/// ```should_panic
/// use rusp::usp_generator::usp_simple_error;
/// let err = usp_simple_error(8000, None);
/// ```
pub fn usp_simple_error(code: u32, message: Option<&str>) -> Body {
    use crate::usp::mod_Body::OneOfmsg_body::*;

    let err_msg = message.unwrap_or_else(|| get_err_msg(code).expect("Invalid USP error code"));

    Body {
        msg_body: error({
            Error {
                err_code: code,
                err_msg: Cow::Borrowed(err_msg),
                param_errs: [].to_vec(),
            }
        }),
    }
}

/// Creates a body for a USP Msg with an USP Error
///
/// # Arguments
///
/// * `code` - The USP error code, MUST be between 7000 and 7999
/// * `message` - An `Option<&str>` containing the user readable message. Will be automatically
///               filled in for standard error codes if not supplied
/// * `param_errs` - A slice of a parameter path, error code and error message, can be empty
///
/// # Panics
///
/// Panics if both `code` is an invalid USP error code and `message` is `None`
///
/// # Examples
///
/// ```
/// use rusp::usp_generator::usp_error;
/// let err = usp_error(7001, None, &[]);
/// ```
///
/// ```
/// use rusp::usp_generator::usp_error;
/// let err = usp_error(
///     7803,
///     Some("Funny custom vendor error"),
///     &[("Some.Path", 7804, "Funny error related to path")],
/// );
/// ```
///
/// ```should_panic
/// use rusp::usp_generator::usp_error;
/// let err = usp_error(8000, None, &[]);
/// ```
pub fn usp_error<'a>(
    code: u32,
    message: Option<&'a str>,
    param_errs: &[(&'a str, u32, &'a str)],
) -> Body<'a> {
    use crate::usp::mod_Body::OneOfmsg_body::*;
    use crate::usp::mod_Error::ParamError;

    let err_msg = message.unwrap_or_else(|| get_err_msg(code).expect("Invalid USP error code"));

    Body {
        msg_body: error({
            Error {
                err_code: code,
                err_msg: Cow::Borrowed(err_msg),
                param_errs: param_errs
                    .iter()
                    .map(|(param_path, err_code, err_msg)| ParamError {
                        param_path: Cow::Borrowed(param_path),
                        err_code: *err_code,
                        err_msg: Cow::Borrowed(err_msg),
                    })
                    .collect(),
            }
        }),
    }
}

/// Gets an USP error message from the error code
pub fn get_err_msg(code: u32) -> Option<&'static str> {
    match code {
        7000 => Some("Message failed"),
        7001 => Some("Message not supported"),
        7002 => Some("Request denied (no reason specified)"),
        7003 => Some("Internal error"),
        7004 => Some("Invalid arguments"),
        7005 => Some("Resources exceeded"),
        7006 => Some("Permission denied"),
        7007 => Some("Invalid configuration"),
        7008 => Some("Invalid path syntax"),
        7009 => Some("Parameter action failed"),
        7010 => Some("Unsupported parameter"),
        7011 => Some("Invalid type"),
        7012 => Some("Invalid value"),
        7013 => Some("Attempt to update non-writeable parameter"),
        7014 => Some("Value conflict"),
        7015 => Some("Operation error"),
        7016 => Some("Object does not exist"),
        7017 => Some("Object could not be created"),
        7018 => Some("Object is not a table"),
        7019 => Some("Attempt to create non-creatable Object"),
        7020 => Some("Object could not be updated"),
        7021 => Some("Required parameter failed"),
        7022 => Some("Command failure"),
        7023 => Some("Command canceled"),
        7024 => Some("Delete failure"),
        7025 => Some("Object exists with duplicate key"),
        7026 => Some("Invalid path"),
        7027 => Some("Invalid Command Arguments"),
        7100 => Some("Record could not be parsed"),
        7101 => Some("Secure session required"),
        7102 => Some("Secure session not supported"),
        7103 => Some("Segmentation and reassembly not supported"),
        7104 => Some("Invalid Record value"),
        7028..=7099 | 7105..=7799 => Some(""),
        7800..=7999 => Some("Vendor specific"),
        _ => None,
    }
}

/// Generates a body of a USP Msg with a USP Get request
///
/// # Arguments
///
/// * `params` - An array of parameter/object names to put into the Get request
/// * `max_depth` - Used to limit the maximum tree depth of the results in the corresponding
///   Get Response. A zero value represents no limit.
///
/// # Example
///
/// ```
/// use rusp::usp_generator::usp_get_request;
/// let req = usp_get_request(&["Device.", "Device.DeviceInfo."], 0);
/// ```
pub fn usp_get_request<S: AsRef<str>>(params: &'_ [S], max_depth: u32) -> Body<'_> {
    use crate::usp::mod_Body::OneOfmsg_body::*;
    use crate::usp::mod_Request::OneOfreq_type::*;

    Body {
        msg_body: request({
            Request {
                req_type: get({
                    Get {
                        max_depth,
                        param_paths: params.iter().map(|p| Cow::Borrowed(p.as_ref())).collect(),
                    }
                }),
            }
        }),
    }
}

/// Generates a body of a USP Msg with a USP Add request
///
/// # Arguments
///
/// * `allow_partial` - A boolean indicating whether partial execution of the Set command is permitted
/// * `args` - An array of tuples consisting of an object path and a arrow of tuples consisting of parametername, value and required flag to put into the Set request
///
/// # Example
///
/// ```
/// use rusp::usp_generator::usp_add_request;
/// let req = usp_add_request(true, &[("Device.DeviceInfo.", &[("ProvisioningCode", "configured", true)])]);
/// ```
pub fn usp_add_request<S: AsRef<str>, V: AsRef<[(S, S, bool)]>>(
    allow_partial: bool,
    args: &'_ [(S, V)],
) -> Body<'_> {
    use crate::usp::mod_Body::OneOfmsg_body::*;
    use crate::usp::mod_Request::OneOfreq_type::*;

    Body {
        msg_body: request({
            Request {
                req_type: add({
                    let mut addr = Add::default();
                    addr.allow_partial = allow_partial;
                    for (dir, pars) in args {
                        let mut obj: crate::usp::mod_Add::CreateObject =
                            crate::usp::mod_Add::CreateObject::default();
                        obj.obj_path = Cow::Borrowed(dir.as_ref());
                        for par in pars.as_ref() {
                            obj.param_settings
                                .push(crate::usp::mod_Add::CreateParamSetting {
                                    param: Cow::Borrowed(par.0.as_ref()),
                                    value: Cow::Borrowed(par.1.as_ref()),
                                    required: par.2,
                                });
                        }
                        addr.create_objs.push(obj);
                    }
                    addr
                }),
            }
        }),
    }
}

/// Generates a body of a USP Msg with a USP Delete request
///
/// # Arguments
///
/// * `allow_partial` - A boolean indicating whether partial execution of the Set command is permitted
/// * `obj_paths` - An array of paths specifying the objects to delete
///
/// # Example
///
/// ```
/// use rusp::usp_generator::usp_delete_request;
/// let req = usp_delete_request(true, &["Device.XMPP.Connection.1."]);
/// ```
pub fn usp_delete_request<S: AsRef<str>>(allow_partial: bool, obj_paths: &'_ [S]) -> Body<'_> {
    use crate::usp::mod_Body::OneOfmsg_body::*;
    use crate::usp::mod_Request::OneOfreq_type::*;

    Body {
        msg_body: request({
            Request {
                req_type: delete({
                    Delete {
                        allow_partial,
                        obj_paths: obj_paths
                            .iter()
                            .map(|e| Cow::Borrowed(e.as_ref()))
                            .collect(),
                    }
                }),
            }
        }),
    }
}

/// Generates a body of a USP Msg with a USP Set request
///
/// # Arguments
///
/// * `allow_partial` - A boolean indicating whether partial execution of the Set command is permitted
/// * `args` - An array of tuples consisting of an object path and a arrow of tuples consisting of parametername, value and required flag to put into the Set request
///
/// # Example
///
/// ```
/// use rusp::usp_generator::usp_set_request;
/// let req = usp_set_request(true, &[("Device.DeviceInfo.", &[("ProvisioningCode", "configured", true)])]);
/// ```
pub fn usp_set_request<S: AsRef<str>, V: AsRef<[(S, S, bool)]>>(
    allow_partial: bool,
    args: &'_ [(S, V)],
) -> Body<'_> {
    use crate::usp::mod_Body::OneOfmsg_body::*;
    use crate::usp::mod_Request::OneOfreq_type::*;

    Body {
        msg_body: request({
            Request {
                req_type: set({
                    let mut setr = Set::default();
                    setr.allow_partial = allow_partial;
                    for (dir, pars) in args {
                        let mut obj: crate::usp::mod_Set::UpdateObject =
                            crate::usp::mod_Set::UpdateObject::default();
                        obj.obj_path = Cow::Borrowed(dir.as_ref());
                        for par in pars.as_ref() {
                            obj.param_settings
                                .push(crate::usp::mod_Set::UpdateParamSetting {
                                    param: Cow::Borrowed(par.0.as_ref()),
                                    value: Cow::Borrowed(par.1.as_ref()),
                                    required: par.2,
                                });
                        }
                        setr.update_objs.push(obj);
                    }
                    setr
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
/// let req = usp_notify_request("", true, &NotifyType::OnBoardRequest {
///     oui: "ABCABC".to_string(),
///     product_class: "PC".to_string(),
///     serial_number: "000000".to_string(),
///     agent_supported_protocol_versions: "1.0".to_string()
/// });
/// ```
pub fn usp_notify_request<'a>(sub_id: &'a str, send_resp: bool, typ: &'a NotifyType) -> Body<'a> {
    use crate::usp::mod_Body::OneOfmsg_body::*;
    use crate::usp::mod_Notify::mod_OperationComplete::OneOfoperation_resp;
    use crate::usp::mod_Notify::OneOfnotification::*;
    use crate::usp::mod_Notify::{
        Event, ObjectCreation, ObjectDeletion, OnBoardRequest, OperationComplete, ValueChange,
    };
    use crate::usp::mod_Request::OneOfreq_type::*;
    use crate::usp_types::OperateResponse;

    Body {
        msg_body: request({
            Request {
                req_type: notify({
                    let mut notr = Notify::default();
                    notr.subscription_id = sub_id.into();
                    notr.send_resp = send_resp;
                    notr.notification = match typ {
                        NotifyType::OnBoardRequest {
                            oui,
                            product_class,
                            serial_number,
                            agent_supported_protocol_versions,
                        } => on_board_req(OnBoardRequest {
                            agent_supported_protocol_versions: agent_supported_protocol_versions
                                .into(),
                            oui: oui.into(),
                            product_class: product_class.into(),
                            serial_number: serial_number.into(),
                        }),
                        NotifyType::ValueChange {
                            param_path,
                            param_value,
                        } => value_change(ValueChange {
                            param_path: param_path.into(),
                            param_value: param_value.into(),
                        }),
                        NotifyType::Event {
                            obj_path,
                            event_name,
                            params,
                        } => event(Event {
                            obj_path: obj_path.into(),
                            event_name: event_name.into(),
                            params: params
                                .iter()
                                .map(|(k, v)| {
                                    (Cow::Borrowed(k.as_ref()), Cow::Borrowed(v.as_ref()))
                                })
                                .collect::<HashMap<_, _>>(),
                        }),
                        NotifyType::ObjectCreation {
                            obj_path,
                            unique_keys,
                        } => obj_creation(ObjectCreation {
                            obj_path: obj_path.into(),
                            unique_keys: unique_keys
                                .iter()
                                .map(|(k, v)| {
                                    (Cow::Borrowed(k.as_ref()), Cow::Borrowed(v.as_ref()))
                                })
                                .collect::<HashMap<_, _>>(),
                        }),
                        NotifyType::ObjectDeletion { obj_path } => obj_deletion(ObjectDeletion {
                            obj_path: obj_path.into(),
                        }),
                        NotifyType::OperationComplete {
                            obj_path,
                            command_name,
                            command_key,
                            operation_resp,
                        } => oper_complete(OperationComplete {
                            obj_path: obj_path.into(),
                            command_name: command_name.into(),
                            command_key: command_key.into(),
                            operation_resp: match operation_resp {
                                OperateResponse::OutputArgs(h) => {
                                    OneOfoperation_resp::req_output_args(
                                        crate::usp::mod_Notify::mod_OperationComplete::OutputArgs {
                                            output_args: h
                                                .iter()
                                                .map(|(k, v)| {
                                                    (
                                                        Cow::Borrowed(k.as_ref()),
                                                        Cow::Borrowed(v.as_ref()),
                                                    )
                                                })
                                                .collect::<HashMap<_, _>>(),
                                        },
                                    )
                                }
                                OperateResponse::CommandFailure(code, msg) => {
                                    OneOfoperation_resp::cmd_failure(
                                        crate::usp::mod_Notify::mod_OperationComplete::CommandFailure
                                        {err_code: *code, err_msg: Cow::Borrowed(msg)}
                                        )
                                }
                            },
                        }),
                    };
                    notr
                }),
            }
        }),
    }
}

/// Generates a body of a USP Msg with a USP Operate request
///
/// # Arguments
///
/// * `command` - The full pathname of of the command to execute
/// * `command_key` - The command key to use in the request to allow later matching with a result
/// * `send_resp` - A boolean indicating whether a response is expected in reply to this request
/// * `args` - An array of tuples containing the command input arguments with path names and values
///
/// # Example
///
/// ```
/// use rusp::usp_generator::usp_operate_request;
/// let req = usp_operate_request("Device.Reboot()", "acommandkey", true, &[]);
/// ```
pub fn usp_operate_request<'a, V: AsRef<[(&'a str, &'a str)]>>(
    command: &'a str,
    command_key: &'a str,
    send_resp: bool,
    args: V,
) -> Body<'a> {
    use crate::usp::mod_Body::OneOfmsg_body::*;
    use crate::usp::mod_Request::OneOfreq_type::*;

    Body {
        msg_body: request({
            Request {
                req_type: operate({
                    let mut operater = Operate::default();
                    operater.command = Cow::Borrowed(command);
                    operater.command_key = Cow::Borrowed(command_key);
                    operater.send_resp = send_resp;
                    operater.input_args = args
                        .as_ref()
                        .iter()
                        .map(|(k, v)| (Cow::Borrowed(*k), Cow::Borrowed(*v)))
                        .collect::<HashMap<_, _>>();
                    operater
                }),
            }
        }),
    }
}

/// Generates a body of a USP Msg with a USP GetInstances request
///
/// # Arguments
///
/// * `obj_paths` - An array of parameter/object names to put into the GetInstances request
/// * `first_level_only` - Whether to just return information for the requested path or recursively
///
/// # Example
///
/// ```
/// use rusp::usp_generator::usp_get_instances_request;
/// let req = usp_get_instances_request(&["Device.", "Device.DeviceInfo."], true);
/// ```
pub fn usp_get_instances_request<S: AsRef<str>>(
    obj_paths: &'_ [S],
    first_level_only: bool,
) -> Body<'_> {
    use crate::usp::mod_Body::OneOfmsg_body::*;
    use crate::usp::mod_Request::OneOfreq_type::*;

    Body {
        msg_body: request({
            Request {
                req_type: get_instances({
                    let mut getinr = GetInstances {
                        first_level_only,
                        ..Default::default()
                    };
                    for path in obj_paths {
                        getinr.obj_paths.push(Cow::Borrowed(path.as_ref()));
                    }
                    getinr
                }),
            }
        }),
    }
}

/// Generates a body of a USP Msg with a USP GetSupportedDM request
///
/// # Arguments
///
/// * `paths` - An array of parameter/object names to put into the GetSupportedDM request
/// * `first_level_only` - Whether to just return information for the requested path or recursively
/// * `return_commands` - Return commands in response
/// * `return_events` - Return events in response
/// * `return_params` - Return parameters in response
///
/// # Example
///
/// ```
/// use rusp::usp_generator::usp_get_supported_dm_request;
/// let req = usp_get_supported_dm_request(&["Device.", "Device.DeviceInfo."], false, true, true, true);
/// ```
pub fn usp_get_supported_dm_request<S: AsRef<str>>(
    paths: &'_ [S],
    first_level_only: bool,
    return_commands: bool,
    return_events: bool,
    return_params: bool,
) -> Body<'_> {
    use crate::usp::mod_Body::OneOfmsg_body::*;
    use crate::usp::mod_Request::OneOfreq_type::*;

    Body {
        msg_body: request({
            Request {
                req_type: get_supported_dm({
                    let mut getsdmr = GetSupportedDM {
                        first_level_only,
                        return_commands,
                        return_events,
                        return_params,
                        ..Default::default()
                    };
                    for path in paths {
                        getsdmr.obj_paths.push(Cow::Borrowed(path.as_ref()));
                    }
                    getsdmr
                }),
            }
        }),
    }
}

/// Generates a body of a USP Msg with a USP GetSupportedProtocol request
///
/// # Arguments
///
/// * `cspv` - The controller supported protocol version
///
/// # Example
///
/// ```
/// use rusp::usp_generator::usp_get_supported_prototol_request;
/// let req = usp_get_supported_prototol_request("1.1");
/// ```
pub fn usp_get_supported_prototol_request(cspv: &str) -> Body {
    use crate::usp::mod_Body::OneOfmsg_body::*;
    use crate::usp::mod_Request::OneOfreq_type::*;

    Body {
        msg_body: request({
            Request {
                req_type: get_supported_protocol({
                    GetSupportedProtocol {
                        controller_supported_protocol_versions: Cow::Borrowed(cspv),
                    }
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
                                        resolved_path: Cow::Borrowed(path),
                                        result_params: params
                                            .into_iter()
                                            .map(|(k, v)| (Cow::Borrowed(k), Cow::Borrowed(v)))
                                            .collect::<HashMap<_, _>>(),
                                    });
                                }

                                RequestedPathResult {
                                    requested_path: Cow::Borrowed(path),
                                    err_code: 0,
                                    err_msg: Cow::Borrowed(""),
                                    resolved_path_results: respaths,
                                }
                            }
                            Err(failure) => RequestedPathResult {
                                requested_path: Cow::Borrowed(path),
                                err_code: failure.0,
                                err_msg: Cow::Borrowed(failure.1),
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
/// * `version` - The USP version of the record
/// * `to_id` - The USP Endpoint ID of the receiver
/// * `from_id` - The USP Endpoint ID of the sender
/// * `payload_security` - An enumeration of type `PayloadSecurity`
/// * `mac_signature` - Message authentication code or signature used to ensure the integrity of the
///   non-payload fields, when integrity protection of non-payload fields is performed
/// * `sender_cert` - PEM encoded certificate used to provide the signature in the `mac_signature`
///   field, when the payload security mechanism does not provide the mechanism to do so
/// * `msg` - The ProtoBuf encoded USP Msg
///
/// # Example
///
/// ```
/// use rusp::usp_generator::usp_no_session_context_record;
/// use rusp::usp_types::PayloadSecurity;
/// let newrecord = usp_no_session_context_record(
///     "1.1",
///     "proto::myfancyrecipient",
///     "proto::anonymous",
///     PayloadSecurity::PLAINTEXT,
///     &[],
///     &[],
///     &[],
/// );
/// ```
pub fn usp_no_session_context_record<'a>(
    version: &'a str,
    to_id: &'a str,
    from_id: &'a str,
    payload_security: PayloadSecurity,
    mac_signature: &'a [u8],
    sender_cert: &'a [u8],
    msg: &'a [u8],
) -> Record<'a> {
    use crate::usp_record::mod_Record::OneOfrecord_type::no_session_context;
    use crate::usp_record::NoSessionContextRecord;

    Record {
        version: version.into(),
        to_id: to_id.into(),
        from_id: from_id.into(),
        sender_cert: Cow::Borrowed(sender_cert),
        mac_signature: Cow::Borrowed(mac_signature),
        payload_security,
        record_type: no_session_context(NoSessionContextRecord {
            payload: msg.into(),
        }),
    }
}

/// Wraps a Usp Msg into an "session_context" USP Record with the specified record information
///
/// # Arguments
///
/// * `version` - The USP version of the record
/// * `to_id` - The USP Endpoint ID of the receiver
/// * `from_id` - The USP Endpoint ID of the sender
/// * `payload_security` - An enumeration of type `PayloadSecurity`
/// * `mac_signature` - Message authentication code or signature used to ensure the integrity of the
///   non-payload fields, when integrity protection of non-payload fields is performed
/// * `sender_cert` - PEM encoded certificate used to provide the signature in the `mac_signature`
///   field, when the payload security mechanism does not provide the mechanism to do so
/// * `session_id` - The ID of the context session
/// * `sequence_id` - The sequence number within the context session
/// * `expected_id` - The expected next sequence number within the context session
/// * `retransmit_id` - The sequence number of the part which is being retransmitted
/// * `msg` - The ProtoBuf encoded USP Msg
///
/// # Example
///
/// ```
/// use rusp::usp_generator::{usp_session_context_record, PayloadSARState};
/// use rusp::usp_types::PayloadSecurity;
/// let newrecord = usp_session_context_record(
///     "1.1",
///     "proto::myfancyrecipient",
///     "proto::anonymous",
///     PayloadSecurity::PLAINTEXT,
///     &[],
///     &[],
///     1234,
///     1,
///     2,
///     0,
///     PayloadSARState::NONE,
///     PayloadSARState::NONE,
///     &[],
/// );
/// ```
#[allow(clippy::too_many_arguments)]
pub fn usp_session_context_record<'a>(
    version: &'a str,
    to_id: &'a str,
    from_id: &'a str,
    payload_security: PayloadSecurity,
    mac_signature: &'a [u8],
    sender_cert: &'a [u8],
    session_id: u64,
    sequence_id: u64,
    expected_id: u64,
    retransmit_id: u64,
    payload_sar_state: PayloadSARState,
    payloadrec_sar_state: PayloadSARState,
    msg: &'a [u8],
) -> Record<'a> {
    use crate::usp_record::mod_Record::OneOfrecord_type::session_context;
    use crate::usp_record::SessionContextRecord;

    Record {
        version: version.into(),
        to_id: to_id.into(),
        from_id: from_id.into(),
        sender_cert: Cow::Borrowed(sender_cert),
        mac_signature: Cow::Borrowed(mac_signature),
        payload_security,
        record_type: session_context(SessionContextRecord {
            session_id,
            sequence_id,
            expected_id,
            retransmit_id,
            payload_sar_state: payload_sar_state.into(),
            payloadrec_sar_state: payloadrec_sar_state.into(),
            payload: vec![msg.into()],
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
                        subscription_id: Cow::Borrowed(subscription_id),
                    }
                }),
            }
        }),
    }
}

/// Creates a body for a USP Msg with a USP AddResp response
///
/// # Arguments
///
/// * `result` - A vector of Result tuples to put into the AddResp response
///
/// # Example
///
/// ```
/// use rusp::usp_generator::usp_add_response;
/// let resp = usp_add_response(vec![
///         ("Device.", Ok(("Device.", vec![("", 0, "")] , vec![("Foo", "Bar")]))),
///         ("Dev.", Err((7000, "Message failed"))),
///     ]);
/// ```
#[allow(clippy::type_complexity)]
pub fn usp_add_response<'a>(
    result: Vec<(
        &'a str,
        Result<
            (
                &'a str,
                Vec<(&'a str, u32, &'a str)>,
                Vec<(&'a str, &'a str)>,
            ),
            (u32, &'a str),
        >,
    )>,
) -> Body<'a> {
    use crate::usp::mod_AddResp::mod_CreatedObjectResult::mod_OperationStatus::{
        OneOfoper_status, ParameterError,
    };
    use crate::usp::mod_AddResp::mod_CreatedObjectResult::mod_OperationStatus::{
        OperationFailure, OperationSuccess,
    };
    use crate::usp::mod_AddResp::mod_CreatedObjectResult::OperationStatus;
    use crate::usp::mod_AddResp::CreatedObjectResult;
    use crate::usp::mod_Body::OneOfmsg_body::*;
    use crate::usp::mod_Response::OneOfresp_type::*;
    use crate::usp::AddResp;

    Body {
        msg_body: response({
            Response {
                resp_type: add_resp({
                    let mut addrsp = AddResp::default();
                    for (path, state) in result {
                        addrsp.created_obj_results.push(match state {
                            Ok((instantiated_path, param_errs, unique_keys)) => {
                                let param_errs = param_errs
                                    .into_iter()
                                    .map(|(param, err_code, err_msg)| ParameterError {
                                        param: Cow::Borrowed(param),
                                        err_code,
                                        err_msg: Cow::Borrowed(err_msg),
                                    })
                                    .collect();

                                let unique_keys = unique_keys
                                    .into_iter()
                                    .map(|(k, v)| (Cow::Borrowed(k), Cow::Borrowed(v)))
                                    .collect();

                                let op = OperationSuccess {
                                    instantiated_path: Cow::Borrowed(instantiated_path),
                                    param_errs,
                                    unique_keys,
                                };
                                CreatedObjectResult {
                                    requested_path: Cow::Borrowed(&path),
                                    oper_status: Some(OperationStatus {
                                        oper_status: OneOfoper_status::oper_success(op),
                                    }),
                                }
                            }
                            Err((err_code, err_msg)) => {
                                let op = OperationFailure {
                                    err_code,
                                    err_msg: Cow::Borrowed(err_msg),
                                };
                                CreatedObjectResult {
                                    requested_path: Cow::Borrowed(&path),
                                    oper_status: Some(OperationStatus {
                                        oper_status: OneOfoper_status::oper_failure(op),
                                    }),
                                }
                            }
                        })
                    }
                    addrsp
                }),
            }
        }),
    }
}

/// Creates a body for a USP Msg with a USP DeleteResp response
///
/// # Arguments
///
/// * `result` - A vector of Result tuples to put into the DeleteResp response
///
/// # Example
///
/// ```
/// use rusp::usp_generator::usp_delete_response;
/// let resp = usp_delete_response(vec![
///         ("Device.", Ok((vec!["Device."], vec![("", 0, "")]))),
///         ("Dev.", Err((7000, "Message failed"))),
///     ]);
/// ```
#[allow(clippy::type_complexity)]
pub fn usp_delete_response<'a>(
    result: Vec<(
        &'a str,
        Result<(Vec<&'a str>, Vec<(&'a str, u32, &'a str)>), (u32, &'a str)>,
    )>,
) -> Body<'a> {
    use crate::usp::mod_Body::OneOfmsg_body::*;
    use crate::usp::mod_DeleteResp::mod_DeletedObjectResult::mod_OperationStatus::{
        OneOfoper_status, OperationFailure, OperationSuccess, UnaffectedPathError,
    };
    use crate::usp::mod_DeleteResp::mod_DeletedObjectResult::OperationStatus;
    use crate::usp::mod_DeleteResp::DeletedObjectResult;
    use crate::usp::mod_Response::OneOfresp_type::*;
    use crate::usp::DeleteResp;

    Body {
        msg_body: response({
            Response {
                resp_type: delete_resp({
                    let mut del_rsp = DeleteResp::default();
                    for (path, state) in result {
                        del_rsp.deleted_obj_results.push(match state {
                            Ok((affected_paths, unaffected_path_errs)) => {
                                let affected_paths = affected_paths
                                    .into_iter()
                                    .map(|aff_path| Cow::Borrowed(aff_path))
                                    .collect();

                                let unaffected_path_errs = unaffected_path_errs
                                    .into_iter()
                                    .map(|(unaffected_path, err_code, err_msg)| {
                                        UnaffectedPathError {
                                            unaffected_path: Cow::Borrowed(unaffected_path),
                                            err_code,
                                            err_msg: Cow::Borrowed(err_msg),
                                        }
                                    })
                                    .collect();

                                let op = OperationSuccess {
                                    affected_paths,
                                    unaffected_path_errs,
                                };
                                DeletedObjectResult {
                                    requested_path: Cow::Borrowed(path),
                                    oper_status: Some(OperationStatus {
                                        oper_status: OneOfoper_status::oper_success(op),
                                    }),
                                }
                            }
                            Err((err_code, err_msg)) => {
                                let op = OperationFailure {
                                    err_code,
                                    err_msg: Cow::Borrowed(err_msg),
                                };
                                DeletedObjectResult {
                                    requested_path: Cow::Borrowed(path),
                                    oper_status: Some(OperationStatus {
                                        oper_status: OneOfoper_status::oper_failure(op),
                                    }),
                                }
                            }
                        })
                    }
                    del_rsp
                }),
            }
        }),
    }
}

/// Creates a body for a USP Msg with a USP GetInstancesResp response
///
/// # Arguments
///
/// * `result` - A vector of Result tuples to put into the GetInstancesResp response
///
/// # Example
///
/// ```
/// use rusp::usp_generator::usp_get_instances_response;
/// let resp = usp_get_instances_response(vec![
///         ("Device.", Ok(vec![("Device.", vec![("Foo", "Bar")])])),
///         ("Dev.", Err((7000, "Message failed"))),
///     ]);
/// ```
#[allow(clippy::type_complexity)]
pub fn usp_get_instances_response<'a>(
    result: Vec<(
        &'a str,
        Result<Vec<(&'a str, Vec<(&'a str, &'a str)>)>, (u32, &'a str)>,
    )>,
) -> Body<'a> {
    use crate::usp::mod_Body::OneOfmsg_body::*;
    use crate::usp::mod_GetInstancesResp::{CurrInstance, RequestedPathResult};
    use crate::usp::mod_Response::OneOfresp_type::*;
    use crate::usp::GetInstancesResp;

    Body {
        msg_body: response({
            Response {
                resp_type: get_instances_resp({
                    let mut get_instances_rsp = GetInstancesResp::default();
                    for (path, state) in result {
                        get_instances_rsp.req_path_results.push(match state {
                            Ok(success) => {
                                let mut curr_insts = Vec::with_capacity(success.len());
                                for (instantiated_obj_path, unique_keys) in success {
                                    let instantiated_obj_path =
                                        Cow::Borrowed(instantiated_obj_path);
                                    let unique_keys = unique_keys
                                        .into_iter()
                                        .map(|(k, v)| (Cow::Borrowed(k), Cow::Borrowed(v)))
                                        .collect();

                                    curr_insts.push(CurrInstance {
                                        instantiated_obj_path,
                                        unique_keys,
                                    });
                                }

                                RequestedPathResult {
                                    requested_path: Cow::Borrowed(path),
                                    err_code: 0,
                                    err_msg: Cow::Borrowed(""),
                                    curr_insts,
                                }
                            }
                            Err((err_code, err_msg)) => RequestedPathResult {
                                requested_path: Cow::Borrowed(path),
                                err_code,
                                err_msg: Cow::Borrowed(err_msg),
                                curr_insts: Vec::default(),
                            },
                        })
                    }
                    get_instances_rsp
                }),
            }
        }),
    }
}

/// Creates a body for a USP Msg with a USP GetSupportedProtocolResp response
///
/// # Arguments
///
/// * `result` - A comma separated list of USP Protocol Versions (major.minor) supported.
///
/// # Example
///
/// ```
/// use rusp::usp_generator::usp_get_supported_protocol_response;
/// let resp = usp_get_supported_protocol_response("1.1");
/// ```
pub fn usp_get_supported_protocol_response(result: &str) -> Body {
    use crate::usp::mod_Body::OneOfmsg_body::*;
    use crate::usp::mod_Response::OneOfresp_type::*;
    use crate::usp::GetSupportedProtocolResp;

    Body {
        msg_body: response({
            Response {
                resp_type: get_supported_protocol_resp(GetSupportedProtocolResp {
                    agent_supported_protocol_versions: Cow::Borrowed(result),
                }),
            }
        }),
    }
}

/// Enum describing the result of an operation, sent through the Operate response
#[derive(Debug, PartialEq)]
pub enum OperationResponse<'a> {
    /// A path to the object responsible for performing the operation asynchronously, corresponds to
    /// `req_obj_path` in the protobuf scheme
    Async(&'a str),
    /// The result of an operation that was made synchronously, corresponds to `req_output_args` in
    /// the protobuf scheme
    Sync(Vec<(&'a str, &'a str)>),
    /// An operation error, corresponds to `cmd_failure` in the protobuf scheme
    Error(u32, &'a str),
}

/// Creates a body for a USP Msg with a USP OperateResp response
///
/// # Arguments
///
/// * `result` - The result of an operation
///
/// # Example
///
/// ```
/// use rusp::usp_generator::{usp_operate_response, OperationResponse};
///
/// let resp_output_args = OperationResponse::Sync(vec![("Foo", "Bar")]);
/// let resp_error = OperationResponse::Error(7000, "Message failed");
/// let resp = usp_operate_response(vec![
///         ("Device.Command()", resp_output_args),
///         ("Device.Command()", resp_error),
///     ]);
/// ```
#[allow(clippy::type_complexity)]
pub fn usp_operate_response<'a>(result: Vec<(&'a str, OperationResponse<'a>)>) -> Body<'a> {
    use crate::usp::mod_Body::OneOfmsg_body::*;
    use crate::usp::mod_OperateResp::mod_OperationResult::{
        CommandFailure, OneOfoperation_resp, OutputArgs,
    };
    use crate::usp::mod_OperateResp::OperationResult;
    use crate::usp::mod_Response::OneOfresp_type::*;
    use crate::usp::OperateResp;

    Body {
        msg_body: response({
            Response {
                resp_type: operate_resp({
                    let mut operate_rsp = OperateResp::default();
                    for (executed_command, state) in result {
                        operate_rsp.operation_results.push(match state {
                            OperationResponse::Async(req_obj_path) => OperationResult {
                                executed_command: Cow::Borrowed(executed_command),
                                operation_resp: OneOfoperation_resp::req_obj_path(Cow::Borrowed(
                                    req_obj_path,
                                )),
                            },
                            OperationResponse::Sync(req_output_args) => {
                                let output_args = req_output_args
                                    .into_iter()
                                    .map(|(k, v)| (Cow::Borrowed(k), Cow::Borrowed(v)))
                                    .collect();
                                let output_args = OutputArgs { output_args };

                                OperationResult {
                                    executed_command: Cow::Borrowed(executed_command),
                                    operation_resp: OneOfoperation_resp::req_output_args(
                                        output_args,
                                    ),
                                }
                            }
                            OperationResponse::Error(err_code, err_msg) => {
                                let cmd_fail = CommandFailure {
                                    err_code,
                                    err_msg: Cow::Borrowed(err_msg),
                                };
                                OperationResult {
                                    executed_command: Cow::Borrowed(executed_command),
                                    operation_resp: OneOfoperation_resp::cmd_failure(cmd_fail),
                                }
                            }
                        })
                    }
                    operate_rsp
                }),
            }
        }),
    }
}

/// Creates a body for a USP Msg with a USP SetResp response
///
/// # Arguments
///
/// * `result` - A vector of Result tuples to put into the SetResp response
///
/// # Example
///
/// ```
/// use rusp::usp_generator::usp_set_response;
/// let resp = usp_set_response(vec![
///         ("Device.", Ok(vec![("Device.", vec![] , vec![("Foo", "Bar")])])),
///         ("Dev.", Err((7000, "Message failed", vec![]))),
///     ]);
/// ```
#[allow(clippy::type_complexity)]
pub fn usp_set_response<'a>(
    result: Vec<(
        &'a str,
        Result<
            Vec<(
                &'a str,
                Vec<(&'a str, u32, &'a str)>,
                Vec<(&'a str, &'a str)>,
            )>,
            (u32, &'a str, Vec<(&'a str, Vec<(&'a str, u32, &'a str)>)>),
        >,
    )>,
) -> Body<'a> {
    use crate::usp::mod_Body::OneOfmsg_body::*;
    use crate::usp::mod_Response::OneOfresp_type::*;
    use crate::usp::mod_SetResp::mod_UpdatedObjectResult::mod_OperationStatus::{
        OneOfoper_status, OperationFailure, OperationSuccess, ParameterError,
        UpdatedInstanceFailure, UpdatedInstanceResult,
    };
    use crate::usp::mod_SetResp::mod_UpdatedObjectResult::OperationStatus;
    use crate::usp::mod_SetResp::UpdatedObjectResult;
    use crate::usp::SetResp;

    Body {
        msg_body: response({
            Response {
                resp_type: set_resp({
                    let mut setrsp = SetResp::default();
                    for (path, state) in result {
                        setrsp.updated_obj_results.push(match state {
                            Ok(success) => {
                                let mut updated_inst_results = Vec::with_capacity(success.len());
                                for (affected_path, param_errs, updated_params) in success {
                                    let param_errs = param_errs
                                        .into_iter()
                                        .map(|(param, err_code, err_msg)| ParameterError {
                                            param: Cow::Borrowed(param),
                                            err_code,
                                            err_msg: Cow::Borrowed(err_msg),
                                        })
                                        .collect();
                                    let updated_params = updated_params
                                        .into_iter()
                                        .map(|(k, v)| (Cow::Borrowed(k), Cow::Borrowed(v)))
                                        .collect();

                                    updated_inst_results.push(UpdatedInstanceResult {
                                        affected_path: Cow::Borrowed(affected_path),
                                        param_errs,
                                        updated_params,
                                    });
                                }
                                let op = OperationSuccess {
                                    updated_inst_results,
                                };
                                let op_status = OperationStatus {
                                    oper_status: OneOfoper_status::oper_success(op),
                                };
                                UpdatedObjectResult {
                                    requested_path: Cow::Borrowed(path),
                                    oper_status: Some(op_status),
                                }
                            }
                            Err((err_code, err_msg, inst_failures)) => {
                                let mut updated_inst_failures =
                                    Vec::with_capacity(inst_failures.len());
                                for (affected_path, param_errs) in inst_failures {
                                    let param_errs = param_errs
                                        .into_iter()
                                        .map(|(param, err_code, err_msg)| ParameterError {
                                            param: Cow::Borrowed(param),
                                            err_code,
                                            err_msg: Cow::Borrowed(err_msg),
                                        })
                                        .collect();

                                    updated_inst_failures.push(UpdatedInstanceFailure {
                                        affected_path: Cow::Borrowed(affected_path),
                                        param_errs,
                                    });
                                }
                                let op = OperationFailure {
                                    err_code,
                                    err_msg: Cow::Borrowed(err_msg),
                                    updated_inst_failures,
                                };
                                let op_status = OperationStatus {
                                    oper_status: OneOfoper_status::oper_failure(op),
                                };
                                UpdatedObjectResult {
                                    requested_path: Cow::Borrowed(path),
                                    oper_status: Some(op_status),
                                }
                            }
                        })
                    }
                    setrsp
                }),
            }
        }),
    }
}

/// Creates a body for a USP Msg with a USP GetSupportedDM response
///
/// # Arguments
///
/// * `result` - A vector of Result tuples to put into the GetSupportedDM response
///
/// # Example
///
/// ```
/// use rusp::usp_generator::usp_get_supported_dm_response;
/// let resp = usp_get_supported_dm_response(vec![
///     ("Device.", "urn:broadband-forum-org:tr-181-2-12-0", Ok(vec![
///         ("Device.", "OBJ_READ_ONLY", false, vec![("Foo", "PARAM_READ_ONLY", "PARAM_STRING", "VALUE_CHANGE_ALLOWED")],
///         vec![("Bar", vec![], vec![], "CMD_SYNC")],
///         vec![("Event", vec![])], vec![],)
///     ])),
///     ("Dev.", "urn:broadband-forum-org:tr-181-2-12-0", Err((7000, "Message failed"))),
/// ]);
/// ```
#[allow(clippy::type_complexity)]
pub fn usp_get_supported_dm_response<'a>(
    result: Vec<(
        &'a str, // req_obj_path
        &'a str, // data_model_inst_uri
        Result<
            Vec<(
                &'a str, // supported_obj_path
                &'a str, // access
                bool,    // is_multi_instance
                Vec<(
                    // supported params
                    &'a str, // param name
                    &'a str, // access
                    &'a str, // value type
                    &'a str, // value change
                )>,
                Vec<(
                    // supported commands
                    &'a str,      // command name
                    Vec<&'a str>, // input args
                    Vec<&'a str>, // output args
                    &'a str,      // command type
                )>,
                Vec<(
                    // supported events
                    &'a str,      // event name
                    Vec<&'a str>, // arg names
                )>,
                Vec<&'a str>, // divergent_paths
            )>,
            (u32, &'a str),
        >,
    )>,
) -> Body<'a> {
    use crate::usp::mod_Body::OneOfmsg_body::*;
    use crate::usp::mod_GetSupportedDMResp::{
        RequestedObjectResult, SupportedCommandResult, SupportedEventResult, SupportedObjectResult,
        SupportedParamResult,
    };
    use crate::usp::mod_Response::OneOfresp_type::*;
    use crate::usp::GetSupportedDMResp;

    Body {
        msg_body: response({
            Response {
                resp_type: get_supported_dm_resp({
                    let mut supported_dm_rsp = GetSupportedDMResp::default();
                    for (path, uri, state) in result {
                        supported_dm_rsp.req_obj_results.push(match state {
                            Ok(success) => {
                                let mut supported_objs = Vec::with_capacity(success.len());
                                for (
                                    supported_obj_path,
                                    access,
                                    is_multi_instance,
                                    supported_params,
                                    supported_commands,
                                    supported_events,
                                    divergent_paths,
                                ) in success
                                {
                                    let supported_params = supported_params
                                        .into_iter()
                                        .map(
                                            |(
                                                param_name,
                                                param_access,
                                                param_value_type,
                                                param_value_change,
                                            )| {
                                                SupportedParamResult {
                                                    param_name: Cow::Borrowed(param_name),
                                                    access: param_access.into(),
                                                    value_type: param_value_type.into(),
                                                    value_change: param_value_change.into(),
                                                }
                                            },
                                        )
                                        .collect();
                                    let supported_commands = supported_commands
                                        .into_iter()
                                        .map(
                                            |(
                                                command_name,
                                                input_arg_names,
                                                output_arg_names,
                                                command_type,
                                            )| {
                                                let input_arg_names = input_arg_names
                                                    .into_iter()
                                                    .map(Cow::Borrowed)
                                                    .collect();
                                                let output_arg_names = output_arg_names
                                                    .into_iter()
                                                    .map(Cow::Borrowed)
                                                    .collect();
                                                SupportedCommandResult {
                                                    command_name: Cow::Borrowed(command_name),
                                                    input_arg_names,
                                                    output_arg_names,
                                                    command_type: command_type.into(),
                                                }
                                            },
                                        )
                                        .collect();
                                    let supported_events = supported_events
                                        .into_iter()
                                        .map(|(event_name, arg_names)| {
                                            let arg_names =
                                                arg_names.into_iter().map(Cow::Borrowed).collect();
                                            SupportedEventResult {
                                                event_name: Cow::Borrowed(event_name),
                                                arg_names,
                                            }
                                        })
                                        .collect();
                                    supported_objs.push(SupportedObjectResult {
                                        supported_obj_path: Cow::Borrowed(supported_obj_path),
                                        access: access.into(),
                                        is_multi_instance,
                                        supported_commands,
                                        supported_events,
                                        supported_params,
                                        divergent_paths: divergent_paths
                                            .into_iter()
                                            .map(Cow::Borrowed)
                                            .collect(),
                                    });
                                }
                                RequestedObjectResult {
                                    req_obj_path: Cow::Borrowed(path),
                                    err_code: 0,
                                    err_msg: Cow::Borrowed(""),
                                    data_model_inst_uri: Cow::Borrowed(uri),
                                    supported_objs,
                                }
                            }
                            Err((err_code, err_msg)) => RequestedObjectResult {
                                req_obj_path: Cow::Borrowed(path),
                                err_code,
                                err_msg: Cow::Borrowed(err_msg),
                                data_model_inst_uri: Cow::Borrowed(uri),
                                supported_objs: Vec::default(),
                            },
                        })
                    }
                    supported_dm_rsp
                }),
            }
        }),
    }
}
