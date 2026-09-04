use crate::usp::body::MsgBody;
use crate::usp::get_instances_resp::{CurrInstance, RequestedPathResult};
use crate::usp::request::ReqType;
use crate::usp::response::RespType;
use crate::usp::{Body, GetInstances, GetInstancesResp, Request, Response};

use crate::usp_errors;

use anyhow::Result;

#[derive(Clone)]
pub struct GetInstancesBuilder {
    obj_paths: Vec<String>,
    first_level_only: bool,
}

impl GetInstancesBuilder {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            obj_paths: vec![],
            first_level_only: false,
        }
    }

    #[must_use]
    pub const fn with_first_level_only(mut self, first_level_only: bool) -> Self {
        self.first_level_only = first_level_only;
        self
    }

    #[must_use]
    pub fn with_obj_paths(mut self, obj_paths: Vec<String>) -> Self {
        self.obj_paths = obj_paths;
        self
    }

    pub fn build(self) -> Result<Body> {
        Ok(Body {
            msg_body: Some(MsgBody::Request({
                Request {
                    req_type: Some(ReqType::GetInstances({
                        GetInstances {
                            obj_paths: self.obj_paths,
                            first_level_only: self.first_level_only,
                        }
                    })),
                }
            })),
        })
    }
}

#[derive(Clone)]
pub struct CurrInstanceBuilder {
    instantiated_obj_path: String,
    unique_keys: Vec<(String, String)>,
}

impl CurrInstanceBuilder {
    #[must_use]
    pub const fn new(instantiated_obj_path: String) -> Self {
        Self {
            instantiated_obj_path,
            unique_keys: vec![],
        }
    }

    #[must_use]
    pub fn with_unique_keys(mut self, unique_keys: Vec<(String, String)>) -> Self {
        self.unique_keys = unique_keys;
        self
    }

    pub fn build(self) -> Result<CurrInstance> {
        let unique_keys = self.unique_keys.into_iter().collect();

        Ok(CurrInstance {
            instantiated_obj_path: self.instantiated_obj_path,
            unique_keys,
        })
    }
}

#[derive(Clone)]
pub struct ReqPathResultBuilder {
    pub requested_path: String,
    pub err_code: u32,
    pub err_msg: Option<String>,
    pub curr_insts: Vec<CurrInstanceBuilder>,
}

impl ReqPathResultBuilder {
    #[must_use]
    pub const fn new(requested_path: String) -> Self {
        Self {
            requested_path,
            err_code: 0,
            err_msg: None,
            curr_insts: vec![],
        }
    }

    #[must_use]
    pub fn set_err(mut self, err_code: u32, err_msg: Option<String>) -> Self {
        self.err_code = err_code;
        self.err_msg = err_msg;
        self
    }

    #[must_use]
    pub fn with_curr_insts(mut self, curr_insts: Vec<CurrInstanceBuilder>) -> Self {
        self.curr_insts = curr_insts;
        self
    }

    pub fn build(self) -> Result<RequestedPathResult> {
        let err_msg = self
            .err_msg
            .clone()
            .unwrap_or_else(|| usp_errors::get_err_msg(self.err_code).to_string());

        let curr_insts = self
            .curr_insts
            .into_iter()
            .map(CurrInstanceBuilder::build)
            .collect::<Result<Vec<_>>>()?;

        Ok(RequestedPathResult {
            requested_path: self.requested_path,
            err_code: self.err_code,
            err_msg,
            curr_insts,
        })
    }
}

#[derive(Clone)]
pub struct GetInstancesRespBuilder {
    req_path_results: Vec<ReqPathResultBuilder>,
}

impl GetInstancesRespBuilder {
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
                    resp_type: Some(RespType::GetInstancesResp({
                        GetInstancesResp {
                            req_path_results,
                        }
                    })),
                }
            })),
        })
    }
}
