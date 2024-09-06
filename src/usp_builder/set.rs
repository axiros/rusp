use std::collections::HashMap;

use crate::usp::mod_Set::{UpdateObject, UpdateParamSetting};
use crate::usp::mod_SetResp::mod_UpdatedObjectResult::mod_OperationStatus::{
    OneOfoper_status::{oper_failure, oper_success},
    OperationFailure, OperationSuccess,
};
use crate::usp::mod_SetResp::ParameterError;
use crate::usp::mod_SetResp::{mod_UpdatedObjectResult::OperationStatus, UpdatedObjectResult};
use crate::usp::mod_SetResp::{UpdatedInstanceFailure, UpdatedInstanceResult};
use crate::usp::{Body, Request, Response, Set, SetResp};
use crate::usp_errors;

use anyhow::Result;

#[derive(Clone)]
pub struct UpdateObjectBuilder {
    obj_path: String,
    param_settings: Vec<(String, String, bool)>,
}

impl UpdateObjectBuilder {
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

    pub fn build(self) -> Result<UpdateObject<'static>> {
        let param_settings = self
            .param_settings
            .into_iter()
            .map(|(n, v, r)| UpdateParamSetting {
                param: n.into(),
                value: v.into(),
                required: r,
            })
            .collect();

        Ok(UpdateObject {
            obj_path: self.obj_path.into(),
            param_settings,
        })
    }
}

#[derive(Clone)]
pub struct SetBuilder {
    allow_partial: bool,
    update_objs: Vec<UpdateObjectBuilder>,
}

impl SetBuilder {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            allow_partial: false,
            update_objs: vec![],
        }
    }

    #[must_use]
    pub const fn with_allow_partial(mut self, allow_partial: bool) -> Self {
        self.allow_partial = allow_partial;
        self
    }

    #[must_use]
    pub fn with_update_objs(mut self, update_objs: Vec<UpdateObjectBuilder>) -> Self {
        self.update_objs = update_objs;
        self
    }

    pub fn build(self) -> Result<Body<'static>> {
        use crate::usp::mod_Body::OneOfmsg_body::request;
        use crate::usp::mod_Request::OneOfreq_type::set;

        let update_objs = self
            .update_objs
            .into_iter()
            .map(UpdateObjectBuilder::build)
            .collect::<Result<Vec<_>>>()?;

        Ok(Body {
            msg_body: request({
                Request {
                    req_type: set({
                        Set {
                            allow_partial: self.allow_partial,
                            update_objs,
                        }
                    }),
                }
            }),
        })
    }
}

#[derive(Clone)]
pub struct SetRespParameterError {
    pub param: String,
    pub err_code: u32,
    pub err_msg: String,
}

#[derive(Clone)]
pub struct SetOperationSuccessBuilder {
    pub affected_path: String,
    pub param_errs: Vec<SetRespParameterError>,
    pub updated_params: HashMap<String, String>,
}

#[derive(Clone)]
pub struct UpdatedInstanceFailureBuilder {
    affected_path: String,
    param_errs: Vec<SetRespParameterError>,
}

impl UpdatedInstanceFailureBuilder {
    #[must_use]
    pub const fn new(affected_path: String) -> Self {
        Self {
            affected_path,
            param_errs: vec![],
        }
    }

    #[must_use]
    pub fn with_param_errs(mut self, param_errs: Vec<SetRespParameterError>) -> Self {
        self.param_errs = param_errs;
        self
    }
}

#[derive(Clone)]
pub struct SetOperationFailureBuilder {
    err_code: u32,
    err_msg: String,
    updated_inst_failures: Vec<UpdatedInstanceFailureBuilder>,
}

#[derive(Clone)]
pub enum SetOperationStatus {
    Failure(SetOperationFailureBuilder),
    Success(Vec<SetOperationSuccessBuilder>),
    None,
}

impl SetOperationStatus {
    #[must_use]
    pub const fn new() -> Self {
        Self::None
    }

    #[must_use]
    pub fn set_failure(
        self,
        err_code: u32,
        err_msg: Option<String>,
        updated_inst_failures: Vec<UpdatedInstanceFailureBuilder>,
    ) -> Self {
        Self::Failure(SetOperationFailureBuilder {
            err_code,
            err_msg: err_msg.unwrap_or_else(|| usp_errors::get_err_msg(err_code).to_string()),
            updated_inst_failures,
        })
    }

    #[must_use]
    pub fn set_success(self, updated_inst_results: Vec<SetOperationSuccessBuilder>) -> Self {
        Self::Success(updated_inst_results)
    }

    pub fn build(self) -> Result<OperationStatus<'static>> {
        match self {
            Self::Failure(f) => Ok(OperationStatus {
                oper_status: oper_failure(OperationFailure {
                    err_code: f.err_code,
                    err_msg: f.err_msg.into(),
                    updated_inst_failures: f
                        .updated_inst_failures
                        .into_iter()
                        .map(|e| UpdatedInstanceFailure {
                            affected_path: e.affected_path.into(),
                            param_errs: e
                                .param_errs
                                .into_iter()
                                .map(|p| ParameterError {
                                    param: p.param.into(),
                                    err_code: p.err_code,
                                    err_msg: if p.err_msg.is_empty() {
                                        usp_errors::get_err_msg(p.err_code).into()
                                    } else {
                                        p.err_msg.into()
                                    },
                                })
                                .collect(),
                        })
                        .collect(),
                }),
            }),
            Self::Success(s) => Ok(OperationStatus {
                oper_status: oper_success(OperationSuccess {
                    updated_inst_results: s
                        .into_iter()
                        .map(|s| UpdatedInstanceResult {
                            affected_path: s.affected_path.into(),
                            param_errs: s
                                .param_errs
                                .into_iter()
                                .map(|e| ParameterError {
                                    param: e.param.into(),
                                    err_code: e.err_code,
                                    err_msg: if e.err_msg.is_empty() {
                                        usp_errors::get_err_msg(e.err_code).into()
                                    } else {
                                        e.err_msg.into()
                                    },
                                })
                                .collect(),
                            updated_params: s
                                .updated_params
                                .into_iter()
                                .map(|(k, v)| (k.into(), v.into()))
                                .collect(),
                        })
                        .collect(),
                }),
            }),
            Self::None => Err(anyhow::anyhow!("")),
        }
    }
}

#[derive(Clone)]
pub struct UpdatedObjectResultsBuilder {
    requested_path: String,
    oper_status: SetOperationStatus,
}

impl UpdatedObjectResultsBuilder {
    #[must_use]
    pub const fn new(requested_path: String, oper_status: SetOperationStatus) -> Self {
        Self {
            requested_path,
            oper_status,
        }
    }

    pub fn build(self) -> Result<UpdatedObjectResult<'static>> {
        Ok(UpdatedObjectResult {
            requested_path: self.requested_path.into(),
            oper_status: Some(self.oper_status.build()?),
        })
    }
}

#[derive(Clone)]
pub struct SetRespBuilder {
    updated_obj_results: Vec<UpdatedObjectResultsBuilder>,
}

impl SetRespBuilder {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            updated_obj_results: vec![],
        }
    }

    #[must_use]
    pub fn with_updated_obj_results(
        mut self,
        updated_obj_results: Vec<UpdatedObjectResultsBuilder>,
    ) -> Self {
        self.updated_obj_results = updated_obj_results;
        self
    }

    pub fn build(self) -> Result<Body<'static>> {
        use crate::usp::mod_Body::OneOfmsg_body::response;
        use crate::usp::mod_Response::OneOfresp_type::set_resp;

        let updated_obj_results = self
            .updated_obj_results
            .into_iter()
            .map(UpdatedObjectResultsBuilder::build)
            .collect::<Result<Vec<_>>>()?;

        Ok(Body {
            msg_body: response({
                Response {
                    resp_type: set_resp({
                        SetResp {
                            updated_obj_results,
                        }
                    }),
                }
            }),
        })
    }
}
