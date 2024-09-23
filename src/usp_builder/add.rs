use std::collections::HashMap;

use crate::usp::mod_Add::{CreateObject, CreateParamSetting};
use crate::usp::mod_AddResp::{mod_CreatedObjectResult::OperationStatus, CreatedObjectResult};
use crate::usp::{Add, AddResp, Body, Request, Response};
use crate::usp_errors;

use anyhow::Result;

#[derive(Clone)]
pub struct CreateObjectBuilder {
    obj_path: String,
    param_settings: Vec<(String, String, bool)>,
}

impl CreateObjectBuilder {
    #[must_use]
    pub const fn new(obj_path: String) -> Self {
        Self {
            obj_path,
            param_settings: vec![],
        }
    }

    #[must_use]
    pub fn with_param_settings(mut self, param_settings: Vec<(String, String, bool)>) -> Self {
        self.param_settings = param_settings;
        self
    }

    pub fn build(self) -> Result<CreateObject> {
        let param_settings = self
            .param_settings
            .into_iter()
            .map(|(n, v, r)| CreateParamSetting {
                param: n,
                value: v,
                required: r,
            })
            .collect();

        Ok(CreateObject {
            obj_path: self.obj_path,
            param_settings,
        })
    }
}

#[derive(Clone)]
pub struct AddBuilder {
    allow_partial: bool,
    create_objs: Vec<CreateObjectBuilder>,
}

impl AddBuilder {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            allow_partial: false,
            create_objs: vec![],
        }
    }

    #[must_use]
    pub const fn with_allow_partial(mut self, allow_partial: bool) -> Self {
        self.allow_partial = allow_partial;
        self
    }

    #[must_use]
    pub fn with_create_objs(mut self, create_objs: Vec<CreateObjectBuilder>) -> Self {
        self.create_objs = create_objs;
        self
    }

    pub fn build(self) -> Result<Body> {
        use crate::usp::mod_Body::OneOfmsg_body::request;
        use crate::usp::mod_Request::OneOfreq_type::add;

        let create_objs = self
            .create_objs
            .into_iter()
            .map(CreateObjectBuilder::build)
            .collect::<Result<Vec<_>>>()?;

        Ok(Body {
            msg_body: request({
                Request {
                    req_type: add({
                        Add {
                            allow_partial: self.allow_partial,
                            create_objs,
                        }
                    }),
                }
            }),
        })
    }
}

#[derive(Clone)]
pub struct AddRespParameterError {
    pub param: String,
    pub err_code: u32,
    pub err_msg: String,
}

#[derive(Clone)]
pub struct AddOperationSuccessBuilder {
    pub instantiated_path: String,
    pub param_errs: Vec<AddRespParameterError>,
    pub unique_keys: HashMap<String, String>,
}

#[derive(Clone)]
pub struct AddOperationFailureBuilder {
    pub err_code: u32,
    pub err_msg: String,
}

#[derive(Clone)]
pub enum AddOperationStatus {
    Failure(AddOperationFailureBuilder),
    Success(AddOperationSuccessBuilder),
    None,
}

impl AddOperationStatus {
    #[must_use]
    pub const fn new() -> Self {
        Self::None
    }

    #[must_use]
    pub fn set_failure(self, err_code: u32, err_msg: Option<String>) -> Self {
        Self::Failure(AddOperationFailureBuilder {
            err_code,
            err_msg: err_msg.unwrap_or_else(|| usp_errors::get_err_msg(err_code).to_string()),
        })
    }

    #[must_use]
    pub fn set_success(
        self,
        instantiated_path: String,
        param_errs: Vec<AddRespParameterError>,
        unique_keys: HashMap<String, String>,
    ) -> Self {
        Self::Success(AddOperationSuccessBuilder {
            instantiated_path,
            param_errs,
            unique_keys,
        })
    }

    pub fn build(self) -> Result<OperationStatus> {
        use crate::usp::mod_AddResp::mod_CreatedObjectResult::mod_OperationStatus::{
            OneOfoper_status::{oper_failure, oper_success},
            OperationFailure, OperationSuccess,
        };
        use crate::usp::mod_AddResp::ParameterError;
        match self {
            Self::Failure(f) => Ok(OperationStatus {
                oper_status: oper_failure(OperationFailure {
                    err_code: f.err_code,
                    err_msg: f.err_msg,
                }),
            }),
            Self::Success(s) => Ok(OperationStatus {
                oper_status: oper_success(OperationSuccess {
                    instantiated_path: s.instantiated_path,
                    param_errs: s
                        .param_errs
                        .into_iter()
                        .map(|e| ParameterError {
                            param: e.param,
                            err_code: e.err_code,
                            err_msg: if e.err_msg.is_empty() {
                                usp_errors::get_err_msg(e.err_code).into()
                            } else {
                                e.err_msg
                            },
                        })
                        .collect(),
                    unique_keys: s
                        .unique_keys
                        .into_iter()
                        .collect(),
                }),
            }),
            Self::None => Err(anyhow::anyhow!("")),
        }
    }
}

#[derive(Clone)]
pub struct CreatedObjectResultsBuilder {
    requested_path: String,
    oper_status: AddOperationStatus,
}

impl CreatedObjectResultsBuilder {
    #[must_use]
    pub const fn new(requested_path: String, oper_status: AddOperationStatus) -> Self {
        Self {
            requested_path,
            oper_status,
        }
    }

    pub fn build(self) -> Result<CreatedObjectResult> {
        Ok(CreatedObjectResult {
            requested_path: self.requested_path,
            oper_status: Some(self.oper_status.build()?),
        })
    }
}

#[derive(Clone)]
pub struct AddRespBuilder {
    created_obj_results: Vec<CreatedObjectResultsBuilder>,
}

impl AddRespBuilder {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            created_obj_results: vec![],
        }
    }

    #[must_use]
    pub fn with_created_obj_results(
        mut self,
        created_obj_results: Vec<CreatedObjectResultsBuilder>,
    ) -> Self {
        self.created_obj_results = created_obj_results;
        self
    }

    pub fn build(self) -> Result<Body> {
        use crate::usp::mod_Body::OneOfmsg_body::response;
        use crate::usp::mod_Response::OneOfresp_type::add_resp;

        let created_obj_results = self
            .created_obj_results
            .into_iter()
            .map(CreatedObjectResultsBuilder::build)
            .collect::<Result<Vec<_>>>()?;

        Ok(Body {
            msg_body: response({
                Response {
                    resp_type: add_resp({
                        AddResp {
                            created_obj_results,
                        }
                    }),
                }
            }),
        })
    }
}
