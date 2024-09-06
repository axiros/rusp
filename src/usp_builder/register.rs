use crate::usp::mod_Body::OneOfmsg_body::{request, response};
use crate::usp::mod_Register::RegistrationPath;
use crate::usp::mod_RegisterResp::mod_RegisteredPathResult::mod_OperationStatus::{
    OneOfoper_status, OperationFailure, OperationSuccess,
};
use crate::usp::mod_RegisterResp::mod_RegisteredPathResult::OperationStatus;
use crate::usp::mod_RegisterResp::RegisteredPathResult;
use crate::usp::mod_Request::OneOfreq_type::register;
use crate::usp::mod_Response::OneOfresp_type::register_resp;
use crate::usp::{Body, Register, RegisterResp, Request, Response};

use crate::usp_errors;

use anyhow::Result;

#[derive(Clone)]
pub struct RegisterBuilder {
    allow_partial: bool,
    reg_paths: Vec<String>,
}

impl RegisterBuilder {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            allow_partial: false,
            reg_paths: vec![],
        }
    }

    #[must_use]
    pub const fn with_allow_partial(mut self, allow_partial: bool) -> Self {
        self.allow_partial = allow_partial;
        self
    }

    #[must_use]
    pub fn with_reg_paths(mut self, reg_paths: Vec<String>) -> Self {
        self.reg_paths = reg_paths;
        self
    }

    pub fn build(self) -> Result<Body<'static>> {
        Ok(Body {
            msg_body: request({
                Request {
                    req_type: register({
                        Register {
                            allow_partial: self.allow_partial,
                            reg_paths: self
                                .reg_paths
                                .into_iter()
                                .map(|p| RegistrationPath { path: p.into() })
                                .collect(),
                        }
                    }),
                }
            }),
        })
    }
}

#[derive(Clone)]
pub enum RegisterOperationStatus {
    Failure { err_code: u32, err_msg: String },
    Success(String),
    None,
}

#[derive(Clone)]
pub struct RegisteredPathResultBuilder {
    pub requested_path: String,
    pub oper_status: RegisterOperationStatus,
}

impl RegisteredPathResultBuilder {
    #[must_use]
    pub const fn new(requested_path: String) -> Self {
        Self {
            requested_path,
            oper_status: RegisterOperationStatus::None,
        }
    }

    #[must_use]
    pub fn set_failure(mut self, err_code: u32, err_msg: Option<String>) -> Self {
        self.oper_status = RegisterOperationStatus::Failure {
            err_code,
            err_msg: err_msg.unwrap_or_else(|| usp_errors::get_err_msg(err_code).to_string()),
        };
        self
    }

    #[must_use]
    pub fn set_success(mut self, registered_path: String) -> Self {
        self.oper_status = RegisterOperationStatus::Success(registered_path);
        self
    }

    pub fn build(self) -> Result<RegisteredPathResult<'static>> {
        Ok(RegisteredPathResult {
            requested_path: self.requested_path.into(),
            oper_status: Some(match self.oper_status {
                RegisterOperationStatus::Failure { err_code, err_msg } => Ok(OperationStatus {
                    oper_status: OneOfoper_status::oper_failure(OperationFailure {
                        err_code,
                        err_msg: err_msg.into(),
                    }),
                }),
                RegisterOperationStatus::Success(s) => Ok(OperationStatus {
                    oper_status: OneOfoper_status::oper_success(OperationSuccess {
                        registered_path: s.into(),
                    }),
                }),
                RegisterOperationStatus::None => Err(anyhow::anyhow!("")),
            }?),
        })
    }
}

#[derive(Clone)]
pub struct RegisterRespBuilder {
    registered_path_results: Vec<RegisteredPathResultBuilder>,
}

impl RegisterRespBuilder {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            registered_path_results: vec![],
        }
    }

    #[must_use]
    pub fn with_registered_path_results(
        mut self,
        registered_path_results: Vec<RegisteredPathResultBuilder>,
    ) -> Self {
        self.registered_path_results = registered_path_results;
        self
    }

    pub fn build(self) -> Result<Body<'static>> {
        let registered_path_results = self
            .registered_path_results
            .into_iter()
            .map(RegisteredPathResultBuilder::build)
            .collect::<Result<Vec<_>>>()?;

        Ok(Body {
            msg_body: response({
                Response {
                    resp_type: register_resp(RegisterResp {
                        registered_path_results,
                    }),
                }
            }),
        })
    }
}
