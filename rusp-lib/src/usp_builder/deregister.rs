use crate::usp::body::MsgBody;
use crate::usp::deregister_resp::deregistered_path_result::OperationStatus;
use crate::usp::deregister_resp::DeregisteredPathResult;
use crate::usp::request::ReqType;
use crate::usp::response::RespType;
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

    pub fn build(self) -> Result<Body> {
        Ok(Body {
            msg_body: Some(MsgBody::Request({
                Request {
                    req_type: Some(ReqType::Deregister(Deregister {
                        paths: self.paths,
                    })),
                }
            })),
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

    pub fn build(self) -> Result<DeregisteredPathResult> {
        use crate::usp::deregister_resp::deregistered_path_result::operation_status::{
            OperStatus, OperationFailure, OperationSuccess,
        };

        Ok(DeregisteredPathResult {
            requested_path: self.requested_path,
            oper_status: Some(match self.oper_status {
                DeregisterOperationStatus::Failure { err_code, err_msg } => OperationStatus {
                    oper_status: Some(OperStatus::OperFailure(OperationFailure {
                        err_code,
                        err_msg,
                    })),
                },
                DeregisterOperationStatus::Success(s) => OperationStatus {
                    oper_status: Some(OperStatus::OperSuccess(OperationSuccess {
                        deregistered_path: s,
                    })),
                },
                DeregisterOperationStatus::None => Err(anyhow::anyhow!(""))?,
            }),
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

    pub fn build(self) -> Result<Body> {
        let deregistered_path_results = self
            .deregistered_path_results
            .into_iter()
            .map(DeregisteredPathResultBuilder::build)
            .collect::<Result<Vec<_>>>()?;

        Ok(Body {
            msg_body: Some(MsgBody::Response({
                Response {
                    resp_type: Some(RespType::DeregisterResp(DeregisterResp {
                        deregistered_path_results,
                    })),
                }
            })),
        })
    }
}
