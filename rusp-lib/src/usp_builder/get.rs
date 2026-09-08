use crate::usp::body::MsgBody;
use crate::usp::get_resp::{RequestedPathResult, ResolvedPathResult};
use crate::usp::request::ReqType;
use crate::usp::response::RespType;
use crate::usp::{Body, Get, GetResp, Request, Response};

use crate::usp_errors;

use anyhow::Result;

#[derive(Clone)]
pub struct GetBuilder {
    max_depth: u32,
    params: Vec<String>,
}

impl GetBuilder {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            max_depth: 0,
            params: vec![],
        }
    }

    #[must_use]
    pub const fn with_max_depth(mut self, max_depth: u32) -> Self {
        self.max_depth = max_depth;
        self
    }

    #[must_use]
    pub fn with_params(mut self, params: Vec<String>) -> Self {
        self.params = params;
        self
    }

    pub fn build(self) -> Result<Body> {
        Ok(Body {
            msg_body: Some(MsgBody::Request({
                Request {
                    req_type: Some(ReqType::Get({
                        Get {
                            max_depth: self.max_depth,
                            param_paths: self.params,
                        }
                    })),
                }
            })),
        })
    }
}

#[derive(Clone)]
pub struct ResolvedPathResultBuilder {
    resolved_path: String,
    result_params: Vec<(String, String)>,
}

impl ResolvedPathResultBuilder {
    #[must_use]
    pub const fn new(resolved_path: String) -> Self {
        Self {
            resolved_path,
            result_params: vec![],
        }
    }

    #[must_use]
    pub fn with_result_params(mut self, result_params: Vec<(String, String)>) -> Self {
        self.result_params = result_params;
        self
    }

    pub fn build(self) -> Result<ResolvedPathResult> {
        let result_params = self.result_params.into_iter().collect();

        Ok(ResolvedPathResult {
            resolved_path: self.resolved_path,
            result_params,
        })
    }
}

#[derive(Clone)]
pub struct ReqPathResultBuilder {
    requested_path: String,
    err_code: u32,
    err_msg: Option<String>,
    resolved_path_results: Vec<ResolvedPathResultBuilder>,
}

impl ReqPathResultBuilder {
    #[must_use]
    pub const fn new(requested_path: String) -> Self {
        Self {
            requested_path,
            err_code: 0,
            err_msg: None,
            resolved_path_results: vec![],
        }
    }

    #[must_use]
    pub fn set_err(mut self, err_code: u32, err_msg: Option<String>) -> Self {
        self.err_code = err_code;
        self.err_msg = err_msg;
        self
    }

    #[must_use]
    pub fn with_res_path_results(
        mut self,
        resolved_path_results: Vec<ResolvedPathResultBuilder>,
    ) -> Self {
        self.resolved_path_results = resolved_path_results;
        self
    }

    pub fn build(self) -> Result<RequestedPathResult> {
        let err_msg = self
            .err_msg
            .clone()
            .unwrap_or_else(|| usp_errors::get_err_msg(self.err_code).to_string());

        let resolved_path_results = self
            .resolved_path_results
            .into_iter()
            .map(ResolvedPathResultBuilder::build)
            .collect::<Result<Vec<_>>>()?;

        Ok(RequestedPathResult {
            requested_path: self.requested_path,
            err_code: self.err_code,
            err_msg,
            resolved_path_results,
        })
    }
}

#[derive(Clone)]
pub struct GetRespBuilder {
    req_path_results: Vec<ReqPathResultBuilder>,
}

impl GetRespBuilder {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            req_path_results: vec![],
        }
    }

    #[must_use]
    pub fn with_req_path_results(mut self, req_path_results: Vec<ReqPathResultBuilder>) -> Self {
        self.req_path_results = req_path_results;
        self
    }

    pub fn build(self) -> Result<Body> {
        let req_path_results = self
            .req_path_results
            .into_iter()
            .map(ReqPathResultBuilder::build)
            .collect::<Result<Vec<_>>>()?;

        Ok(Body {
            msg_body: Some(MsgBody::Response({
                Response {
                    resp_type: Some(RespType::GetResp(GetResp { req_path_results })),
                }
            })),
        })
    }
}
