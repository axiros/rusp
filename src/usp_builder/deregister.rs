use crate::usp::mod_Body::OneOfmsg_body::{request, response};
use crate::usp::mod_DeregisterResp::mod_DeregisteredPathResult::mod_OperationStatus::{
    OneOfoper_status, OperationFailure, OperationSuccess,
};
use crate::usp::mod_DeregisterResp::mod_DeregisteredPathResult::OperationStatus;
use crate::usp::mod_DeregisterResp::DeregisteredPathResult;
use crate::usp::mod_Request::OneOfreq_type::deregister;
use crate::usp::mod_Response::OneOfresp_type::deregister_resp;
use crate::usp::{Body, Deregister, DeregisterResp, Request, Response};

use crate::usp_errors;

use anyhow::Result;

#[derive(Clone)]
pub struct DeregisterBuilder {
    paths: Vec<String>,
}

impl DeregisterBuilder {
    #[must_use]
    pub const fn new() -> Self {
        Self { paths: vec![] }
    }

    #[must_use]
    pub fn with_paths(mut self, paths: Vec<String>) -> Self {
        self.paths = paths;
        self
    }

    pub fn build(self) -> Result<Body<'static>> {
        Ok(Body {
            msg_body: request({
                Request {
                    req_type: deregister({
                        Deregister {
                            paths: self
                                .paths
                                .into_iter()
                                .map(std::convert::Into::into)
                                .collect(),
                        }
                    }),
                }
            }),
        })
    }
}

#[derive(Clone)]
pub enum DeregisterOperationStatus {
    Failure { err_code: u32, err_msg: String },
    Success(Vec<String>),
    None,
}

#[derive(Clone)]
pub struct DeregisteredPathResultBuilder {
    pub requested_path: String,
    pub oper_status: DeregisterOperationStatus,
}

impl DeregisteredPathResultBuilder {
    #[must_use]
    pub const fn new(requested_path: String) -> Self {
        Self {
            requested_path,
            oper_status: DeregisterOperationStatus::None,
        }
    }

    #[must_use]
    pub fn set_failure(mut self, err_code: u32, err_msg: Option<String>) -> Self {
        self.oper_status = DeregisterOperationStatus::Failure {
            err_code,
            err_msg: err_msg.unwrap_or_else(|| usp_errors::get_err_msg(err_code).to_string()),
        };
        self
    }

    #[must_use]
    pub fn set_success(mut self, deregistered_path: Vec<String>) -> Self {
        self.oper_status = DeregisterOperationStatus::Success(deregistered_path);
        self
    }

    pub fn build(self) -> Result<DeregisteredPathResult<'static>> {
        Ok(DeregisteredPathResult {
            requested_path: self.requested_path.into(),
            oper_status: Some(match self.oper_status {
                DeregisterOperationStatus::Failure { err_code, err_msg } => Ok(OperationStatus {
                    oper_status: OneOfoper_status::oper_failure(OperationFailure {
                        err_code,
                        err_msg: err_msg.into(),
                    }),
                }),
                DeregisterOperationStatus::Success(s) => Ok(OperationStatus {
                    oper_status: OneOfoper_status::oper_success(OperationSuccess {
                        deregistered_path: s
                            .into_iter()
                            .map(std::convert::Into::into)
                            .collect::<Vec<_>>(),
                    }),
                }),
                DeregisterOperationStatus::None => Err(anyhow::anyhow!("")),
            }?),
        })
    }
}

#[derive(Clone)]
pub struct DeregisterRespBuilder {
    deregistered_path_results: Vec<DeregisteredPathResultBuilder>,
}

impl DeregisterRespBuilder {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            deregistered_path_results: vec![],
        }
    }

    #[must_use]
    pub fn with_deregistered_path_results(
        mut self,
        deregistered_path_results: Vec<DeregisteredPathResultBuilder>,
    ) -> Self {
        self.deregistered_path_results = deregistered_path_results;
        self
    }

    pub fn build(self) -> Result<Body<'static>> {
        let deregistered_path_results = self
            .deregistered_path_results
            .into_iter()
            .map(DeregisteredPathResultBuilder::build)
            .collect::<Result<Vec<_>>>()?;

        Ok(Body {
            msg_body: response({
                Response {
                    resp_type: deregister_resp(DeregisterResp {
                        deregistered_path_results,
                    }),
                }
            }),
        })
    }
}
