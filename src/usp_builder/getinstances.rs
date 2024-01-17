use crate::usp::mod_Body::OneOfmsg_body::{request, response};
use crate::usp::mod_GetInstancesResp::{CurrInstance, RequestedPathResult};
use crate::usp::mod_Request::OneOfreq_type::get_instances;
use crate::usp::mod_Response::OneOfresp_type::get_instances_resp;
use crate::usp::{Body, GetInstances, GetInstancesResp, Request, Response};

use crate::usp_generator;

use anyhow::Result;

#[derive(Clone)]
pub struct GetInstancesBuilder {
    obj_paths: Vec<String>,
    first_level_only: bool,
}

impl GetInstancesBuilder {
    pub const fn new() -> Self {
        Self {
            obj_paths: vec![],
            first_level_only: false,
        }
    }

    pub fn with_first_level_only(mut self, first_level_only: bool) -> Self {
        self.first_level_only = first_level_only;
        self
    }

    pub fn with_obj_paths(mut self, obj_paths: Vec<String>) -> Self {
        self.obj_paths = obj_paths;
        self
    }

    pub fn build(self) -> Result<Body<'static>> {
        Ok(Body {
            msg_body: request({
                Request {
                    req_type: get_instances({
                        GetInstances {
                            obj_paths: self.obj_paths.into_iter().map(Into::into).collect(),
                            first_level_only: self.first_level_only,
                        }
                    }),
                }
            }),
        })
    }
}

#[derive(Clone)]
pub struct CurrInstanceBuilder {
    instantiated_obj_path: String,
    unique_keys: Vec<(String, String)>,
}

impl CurrInstanceBuilder {
    pub const fn new(instantiated_obj_path: String) -> Self {
        Self {
            instantiated_obj_path,
            unique_keys: vec![],
        }
    }

    pub fn with_unique_keys(mut self, unique_keys: Vec<(String, String)>) -> Self {
        self.unique_keys = unique_keys;
        self
    }

    pub fn build(self) -> Result<CurrInstance<'static>> {
        let unique_keys = self
            .unique_keys
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect();

        Ok(CurrInstance {
            instantiated_obj_path: self.instantiated_obj_path.into(),
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
    pub const fn new(requested_path: String) -> Self {
        Self {
            requested_path,
            err_code: 0,
            err_msg: None,
            curr_insts: vec![],
        }
    }

    pub fn set_err(mut self, err_code: u32, err_msg: Option<String>) -> Self {
        self.err_code = err_code;
        self.err_msg = err_msg;
        self
    }

    pub fn with_curr_insts(mut self, curr_insts: Vec<CurrInstanceBuilder>) -> Self {
        self.curr_insts = curr_insts;
        self
    }

    pub fn build(self) -> Result<RequestedPathResult<'static>> {
        let err_msg = self
            .err_msg
            .clone()
            .unwrap_or_else(|| usp_generator::get_err_msg(self.err_code).to_string());

        let curr_insts = self
            .curr_insts
            .into_iter()
            .map(|c| c.build())
            .collect::<Result<Vec<_>>>()?;

        Ok(RequestedPathResult {
            requested_path: self.requested_path.into(),
            err_code: self.err_code,
            err_msg: err_msg.into(),
            curr_insts,
        })
    }
}

#[derive(Clone)]
pub struct GetInstancesRespBuilder {
    req_path_results: Vec<ReqPathResultBuilder>,
}

impl GetInstancesRespBuilder {
    pub const fn new() -> Self {
        Self {
            req_path_results: vec![],
        }
    }

    pub fn with_req_path_results(mut self, req_path_results: Vec<ReqPathResultBuilder>) -> Self {
        self.req_path_results = req_path_results;
        self
    }

    pub fn build(self) -> Result<Body<'static>> {
        let req_path_results = self
            .req_path_results
            .into_iter()
            .map(|r| r.build())
            .collect::<Result<Vec<_>>>()?;

        Ok(Body {
            msg_body: response({
                Response {
                    resp_type: get_instances_resp(GetInstancesResp { req_path_results }),
                }
            }),
        })
    }
}
