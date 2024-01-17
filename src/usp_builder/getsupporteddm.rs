use crate::usp::mod_Body::OneOfmsg_body::{request, response};
use crate::usp::mod_GetSupportedDMResp::SupportedUniqueKeySet;
use crate::usp::mod_GetSupportedDMResp::{
    CmdType,
    ObjAccessType::{self, *},
    ParamAccessType, ParamValueType, RequestedObjectResult, SupportedCommandResult,
    SupportedEventResult, SupportedObjectResult, SupportedParamResult, ValueChangeType,
};
use crate::usp::mod_Request::OneOfreq_type::get_supported_dm;
use crate::usp::mod_Response::OneOfresp_type::get_supported_dm_resp;
use crate::usp::{Body, GetSupportedDM, GetSupportedDMResp, Request, Response};
use crate::usp_generator;

use anyhow::Result;

#[derive(Clone)]
pub struct GetSupportedDMBuilder {
    obj_paths: Vec<String>,
    first_level_only: bool,
    return_commands: bool,
    return_events: bool,
    return_params: bool,
    return_unique_key_sets: bool,
}

impl GetSupportedDMBuilder {
    pub const fn new() -> Self {
        Self {
            obj_paths: vec![],
            first_level_only: false,
            return_commands: true,
            return_events: true,
            return_params: true,
            return_unique_key_sets: true,
        }
    }

    pub fn with_obj_paths(mut self, obj_paths: Vec<String>) -> Self {
        self.obj_paths = obj_paths;
        self
    }

    pub fn with_first_level_only(mut self, first_level_only: bool) -> Self {
        self.first_level_only = first_level_only;
        self
    }

    pub fn with_return_commands(mut self, return_commands: bool) -> Self {
        self.return_commands = return_commands;
        self
    }

    pub fn with_return_events(mut self, return_events: bool) -> Self {
        self.return_events = return_events;
        self
    }

    pub fn with_return_params(mut self, return_params: bool) -> Self {
        self.return_params = return_params;
        self
    }

    pub fn with_return_unique_key_sets(mut self, return_unique_key_sets: bool) -> Self {
        self.return_unique_key_sets = return_unique_key_sets;
        self
    }

    pub fn build(self) -> Result<Body<'static>> {
        Ok(Body {
            msg_body: request({
                Request {
                    req_type: get_supported_dm({
                        GetSupportedDM {
                            obj_paths: self.obj_paths.into_iter().map(|e| e.into()).collect(),
                            first_level_only: self.first_level_only,
                            return_commands: self.return_commands,
                            return_events: self.return_events,
                            return_params: self.return_params,
                            return_unique_key_sets: self.return_unique_key_sets,
                        }
                    }),
                }
            }),
        })
    }
}

#[derive(Clone)]
pub struct GSDMCommandResult {
    command_name: String,
    input_arg_names: Vec<String>,
    output_arg_names: Vec<String>,
    command_type: CmdType,
}

impl GSDMCommandResult {
    pub const fn new(command_name: String) -> Self {
        Self {
            command_name,
            input_arg_names: vec![],
            output_arg_names: vec![],
            command_type: CmdType::CMD_UNKNOWN,
        }
    }

    pub fn with_input_arg_names(mut self, input_arg_names: Vec<String>) -> Self {
        self.input_arg_names = input_arg_names;
        self
    }

    pub fn with_output_arg_names(mut self, output_arg_names: Vec<String>) -> Self {
        self.output_arg_names = output_arg_names;
        self
    }

    pub fn set_sync(mut self) -> Self {
        self.command_type = CmdType::CMD_SYNC;
        self
    }

    pub fn set_async(mut self) -> Self {
        self.command_type = CmdType::CMD_ASYNC;
        self
    }

    pub fn build(self) -> Result<SupportedCommandResult<'static>> {
        if matches!(self.command_type, CmdType::CMD_UNKNOWN) {
            anyhow::bail!(
                "Cannot build a Supported Command Result without a specified command type"
            );
        }
        Ok(SupportedCommandResult {
            command_name: self.command_name.into(),
            input_arg_names: self.input_arg_names.into_iter().map(|e| e.into()).collect(),
            output_arg_names: self
                .output_arg_names
                .into_iter()
                .map(|e| e.into())
                .collect(),
            command_type: self.command_type,
        })
    }
}

#[derive(Clone)]
pub struct GSDMEventResult {
    event_name: String,
    arg_names: Vec<String>,
}

impl GSDMEventResult {
    pub const fn new(event_name: String) -> Self {
        Self {
            event_name,
            arg_names: vec![],
        }
    }

    pub fn with_arg_names(mut self, arg_names: Vec<String>) -> Self {
        self.arg_names = arg_names;
        self
    }

    pub fn build(self) -> Result<SupportedEventResult<'static>> {
        Ok(SupportedEventResult {
            event_name: self.event_name.into(),
            arg_names: self.arg_names.into_iter().map(|e| e.into()).collect(),
        })
    }
}

#[derive(Clone)]
pub struct GSDMParamResult {
    param_name: String,
    access: ParamAccessType,
    value_type: ParamValueType,
    value_change: ValueChangeType,
}

impl GSDMParamResult {
    pub const fn new(param_name: String) -> Self {
        Self {
            param_name,
            access: ParamAccessType::PARAM_READ_ONLY,
            value_type: ParamValueType::PARAM_UNKNOWN,
            value_change: ValueChangeType::VALUE_CHANGE_UNKNOWN,
        }
    }

    pub fn set_access_read_only(mut self) -> Self {
        self.access = ParamAccessType::PARAM_READ_ONLY;
        self
    }

    pub fn set_access_write_only(mut self) -> Self {
        self.access = ParamAccessType::PARAM_WRITE_ONLY;
        self
    }

    pub fn set_access_read_write(mut self) -> Self {
        self.access = ParamAccessType::PARAM_READ_WRITE;
        self
    }

    pub fn set_type_int(mut self) -> Self {
        self.value_type = ParamValueType::PARAM_INT;
        self
    }

    pub fn set_type_unsigned_int(mut self) -> Self {
        self.value_type = ParamValueType::PARAM_UNSIGNED_INT;
        self
    }

    pub fn set_type_long(mut self) -> Self {
        self.value_type = ParamValueType::PARAM_LONG;
        self
    }

    pub fn set_type_unsigned_long(mut self) -> Self {
        self.value_type = ParamValueType::PARAM_UNSIGNED_LONG;
        self
    }

    pub fn set_type_string(mut self) -> Self {
        self.value_type = ParamValueType::PARAM_STRING;
        self
    }

    pub fn set_type_base64(mut self) -> Self {
        self.value_type = ParamValueType::PARAM_BASE_64;
        self
    }

    pub fn set_type_hexbinary(mut self) -> Self {
        self.value_type = ParamValueType::PARAM_HEX_BINARY;
        self
    }

    pub fn set_type_datetime(mut self) -> Self {
        self.value_type = ParamValueType::PARAM_DATE_TIME;
        self
    }

    pub fn set_type_decimal(mut self) -> Self {
        self.value_type = ParamValueType::PARAM_DECIMAL;
        self
    }

    pub fn set_type_boolean(mut self) -> Self {
        self.value_type = ParamValueType::PARAM_BOOLEAN;
        self
    }

    pub fn set_value_change_allowed(mut self) -> Self {
        self.value_change = ValueChangeType::VALUE_CHANGE_ALLOWED;
        self
    }

    pub fn set_value_change_will_ignore(mut self) -> Self {
        self.value_change = ValueChangeType::VALUE_CHANGE_WILL_IGNORE;
        self
    }

    pub fn build(self) -> Result<SupportedParamResult<'static>> {
        if matches!(self.value_type, ParamValueType::PARAM_UNKNOWN) {
            anyhow::bail!("Cannot build a Supported Param Result without a specified value type");
        }
        if matches!(self.value_change, ValueChangeType::VALUE_CHANGE_UNKNOWN) {
            anyhow::bail!(
                "Cannot build a Supported Param Result without a specified value change behaviour"
            );
        }
        Ok(SupportedParamResult {
            param_name: self.param_name.into(),
            access: self.access,
            value_type: self.value_type,
            value_change: self.value_change,
        })
    }
}

#[derive(Clone)]
pub struct GSDMSupportedObjectResult {
    supported_obj_path: String,
    access: ObjAccessType,
    is_multi_instance: bool,
    supported_commands: Vec<GSDMCommandResult>,
    supported_events: Vec<GSDMEventResult>,
    supported_params: Vec<GSDMParamResult>,
    divergent_paths: Vec<String>,
    unique_key_sets: Vec<Vec<String>>,
}

impl GSDMSupportedObjectResult {
    pub const fn new(supported_obj_path: String) -> Self {
        Self {
            supported_obj_path,
            access: OBJ_ADD_ONLY,
            is_multi_instance: false,
            supported_commands: vec![],
            supported_events: vec![],
            supported_params: vec![],
            divergent_paths: vec![],
            unique_key_sets: vec![],
        }
    }

    pub fn with_is_multi_instance(mut self, is_multi_instance: bool) -> Self {
        self.is_multi_instance = is_multi_instance;
        self
    }

    pub fn with_supported_commands(mut self, supported_commands: Vec<GSDMCommandResult>) -> Self {
        self.supported_commands = supported_commands;
        self
    }

    pub fn with_supported_events(mut self, supported_events: Vec<GSDMEventResult>) -> Self {
        self.supported_events = supported_events;
        self
    }

    pub fn with_supported_params(mut self, supported_params: Vec<GSDMParamResult>) -> Self {
        self.supported_params = supported_params;
        self
    }

    pub fn with_divergent_paths(mut self, divergent_paths: Vec<String>) -> Self {
        self.divergent_paths = divergent_paths;
        self
    }

    pub fn with_unique_key_sets(mut self, unique_key_sets: Vec<Vec<String>>) -> Self {
        self.unique_key_sets = unique_key_sets;
        self
    }

    pub fn build(self) -> Result<SupportedObjectResult<'static>> {
        let supported_commands = self
            .supported_commands
            .into_iter()
            .map(|i| i.build())
            .collect::<Result<Vec<_>>>()?;

        let supported_events = self
            .supported_events
            .into_iter()
            .map(|i| i.build())
            .collect::<Result<Vec<_>>>()?;

        let supported_params = self
            .supported_params
            .into_iter()
            .map(|i| i.build())
            .collect::<Result<Vec<_>>>()?;

        let unique_key_sets: Vec<SupportedUniqueKeySet<'_>> = self
            .unique_key_sets
            .into_iter()
            .map(|i| SupportedUniqueKeySet {
                key_names: i.into_iter().map(std::borrow::Cow::Owned).collect(),
            })
            .collect();

        Ok(SupportedObjectResult {
            supported_obj_path: self.supported_obj_path.into(),
            access: self.access,
            is_multi_instance: self.is_multi_instance,
            supported_commands,
            supported_events,
            supported_params,
            divergent_paths: self.divergent_paths.into_iter().map(|e| e.into()).collect(),
            unique_key_sets,
        })
    }
}

#[derive(Clone)]
pub struct GSDMReqObjectResultBuilder {
    req_obj_path: String,
    err_code: u32,
    err_msg: Option<String>,
    data_model_inst_uri: String,
    supported_objs: Vec<GSDMSupportedObjectResult>,
}

impl GSDMReqObjectResultBuilder {
    pub const fn new(req_obj_path: String) -> Self {
        Self {
            req_obj_path,
            err_code: 0,
            err_msg: None,
            data_model_inst_uri: String::new(),
            supported_objs: vec![],
        }
    }

    pub fn set_err(mut self, err_code: u32, err_msg: Option<String>) -> Self {
        self.err_code = err_code;
        self.err_msg =
            Some(err_msg.unwrap_or_else(|| usp_generator::get_err_msg(err_code).to_string()));
        self
    }

    pub fn with_data_model_inst_uri(mut self, data_model_inst_uri: String) -> Self {
        self.data_model_inst_uri = data_model_inst_uri;
        self
    }

    pub fn with_supported_objs(mut self, supported_objs: Vec<GSDMSupportedObjectResult>) -> Self {
        self.supported_objs = supported_objs;
        self
    }

    pub fn build(self) -> Result<RequestedObjectResult<'static>> {
        let err_msg = self
            .err_msg
            .clone()
            .unwrap_or_else(|| usp_generator::get_err_msg(self.err_code).to_string());

        let supported_objs = self
            .supported_objs
            .into_iter()
            .map(|c| c.build())
            .collect::<Result<Vec<_>>>()?;

        Ok(RequestedObjectResult {
            req_obj_path: self.req_obj_path.into(),
            err_code: self.err_code,
            err_msg: err_msg.into(),
            data_model_inst_uri: self.data_model_inst_uri.into(),
            supported_objs,
        })
    }
}

#[derive(Clone)]
pub struct GetSupportedDMRespBuilder {
    req_obj_results: Vec<GSDMReqObjectResultBuilder>,
}

impl GetSupportedDMRespBuilder {
    pub const fn new() -> Self {
        Self {
            req_obj_results: vec![],
        }
    }

    pub fn with_req_obj_results(
        mut self,
        req_obj_results: Vec<GSDMReqObjectResultBuilder>,
    ) -> Self {
        self.req_obj_results = req_obj_results;
        self
    }

    pub fn build(self) -> Result<Body<'static>> {
        let req_obj_results = self
            .req_obj_results
            .into_iter()
            .map(|r| r.build())
            .collect::<Result<Vec<_>>>()?;

        Ok(Body {
            msg_body: response({
                Response {
                    resp_type: get_supported_dm_resp(GetSupportedDMResp { req_obj_results }),
                }
            }),
        })
    }
}
