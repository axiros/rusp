use crate::usp::mod_Body::OneOfmsg_body::{request, response};
use crate::usp::mod_GetResp::{RequestedPathResult, ResolvedPathResult};
use crate::usp::mod_Request::OneOfreq_type::get;
use crate::usp::mod_Response::OneOfresp_type::get_resp;
use crate::usp::{Body, Get, GetResp, Request, Response};

use crate::usp_generator;

use anyhow::Result;

#[derive(Clone)]
pub struct GetBuilder {
    max_depth: u32,
    params: Vec<String>,
}

impl<'a> GetBuilder {
    pub fn new() -> Self {
        Self {
            max_depth: 0,
            params: vec![],
        }
    }

    pub fn with_max_depth(mut self, max_depth: u32) -> Self {
        self.max_depth = max_depth;
        self
    }

    pub fn with_params(mut self, params: Vec<String>) -> Self {
        self.params = params;
        self
    }

    pub fn build(self) -> Result<Body<'a>> {
        Ok(Body {
            msg_body: request({
                Request {
                    req_type: get({
                        Get {
                            max_depth: self.max_depth,
                            param_paths: self.params.into_iter().map(Into::into).collect(),
                        }
                    }),
                }
            }),
        })
    }
}

#[derive(Clone)]
pub struct ResolvedPathResultBuilder {
    pub resolved_path: String,
    pub result_params: Vec<(String, String)>,
}

impl<'a> ResolvedPathResultBuilder {
    pub fn new(resolved_path: String) -> Self {
        Self {
            resolved_path,
            result_params: vec![],
        }
    }

    pub fn with_result_params(mut self, result_params: Vec<(String, String)>) -> Self {
        self.result_params = result_params;
        self
    }

    pub fn add_result_param(mut self, name: String, value: String) -> Self {
        self.result_params.push((name, value));
        self
    }

    pub fn build(self) -> Result<ResolvedPathResult<'a>> {
        let result_params = self
            .result_params
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect();

        Ok(ResolvedPathResult {
            resolved_path: self.resolved_path.into(),
            result_params,
        })
    }
}

#[derive(Clone)]
pub struct ReqPathResultBuilder {
    pub requested_path: String,
    pub err_code: u32,
    pub err_msg: Option<String>,
    pub resolved_path_results: Vec<ResolvedPathResultBuilder>,
}

impl<'a> ReqPathResultBuilder {
    pub fn new(requested_path: String) -> Self {
        Self {
            requested_path,
            err_code: 0,
            err_msg: None,
            resolved_path_results: vec![],
        }
    }

    pub fn with_res_path_results(
        mut self,
        resolved_path_results: Vec<ResolvedPathResultBuilder>,
    ) -> Self {
        self.resolved_path_results = resolved_path_results;
        self
    }

    pub fn add_res_path_result(mut self, resolved_path_result: ResolvedPathResultBuilder) -> Self {
        self.resolved_path_results.push(resolved_path_result);
        self
    }

    pub fn build(self) -> Result<RequestedPathResult<'a>> {
        let err_msg = self
            .err_msg
            .clone()
            .unwrap_or_else(|| usp_generator::get_err_msg(self.err_code).to_string());

        let resolved_path_results = self
            .resolved_path_results
            .into_iter()
            .map(|path| path.build())
            .collect::<Result<Vec<_>>>()?;

        Ok(RequestedPathResult {
            requested_path: self.requested_path.into(),
            err_code: 0,
            err_msg: err_msg.into(),
            resolved_path_results,
        })
    }
}

#[derive(Clone)]
pub struct GetRespBuilder {
    req_path_results: Vec<ReqPathResultBuilder>,
}

impl<'a> GetRespBuilder {
    pub fn new() -> Self {
        Self {
            req_path_results: vec![],
        }
    }

    pub fn with_req_path_results(mut self, req_path_results: Vec<ReqPathResultBuilder>) -> Self {
        self.req_path_results = req_path_results;
        self
    }

    pub fn add_req_path_result(mut self, req_path_result: ReqPathResultBuilder) -> Self {
        self.req_path_results.push(req_path_result);
        self
    }

    pub fn build(self) -> Result<Body<'a>> {
        let req_path_results = self
            .req_path_results
            .into_iter()
            .map(|r| r.build())
            .collect::<Result<Vec<_>>>()?;

        Ok(Body {
            msg_body: response({
                Response {
                    resp_type: get_resp(GetResp { req_path_results }),
                }
            }),
        })
    }
}
