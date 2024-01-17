use crate::usp::mod_Body::OneOfmsg_body::{request, response};
use crate::usp::mod_DeleteResp::mod_DeletedObjectResult::mod_OperationStatus::{
    OneOfoper_status::*, OperationFailure, OperationSuccess,
};
use crate::usp::mod_DeleteResp::UnaffectedPathError;
use crate::usp::mod_DeleteResp::{mod_DeletedObjectResult::OperationStatus, DeletedObjectResult};
use crate::usp::mod_Request::OneOfreq_type::delete;
use crate::usp::mod_Response::OneOfresp_type::delete_resp;
use crate::usp::{Body, Delete, DeleteResp, Request, Response};
use crate::usp_generator;

use anyhow::Result;

#[derive(Clone)]
pub struct DeleteBuilder {
    allow_partial: bool,
    obj_paths: Vec<String>,
}

impl<'a> DeleteBuilder {
    pub fn new() -> Self {
        Self {
            allow_partial: false,
            obj_paths: vec![],
        }
    }

    pub fn with_allow_partial(mut self, allow_partial: bool) -> Self {
        self.allow_partial = allow_partial;
        self
    }

    pub fn with_obj_paths(mut self, params: Vec<String>) -> Self {
        self.obj_paths = params;
        self
    }

    pub fn build(self) -> Result<Body<'a>> {
        Ok(Body {
            msg_body: request({
                Request {
                    req_type: delete({
                        Delete {
                            allow_partial: self.allow_partial,
                            obj_paths: self.obj_paths.into_iter().map(Into::into).collect(),
                        }
                    }),
                }
            }),
        })
    }
}

#[derive(Clone)]
pub struct DeleteRespUnaffectedPathError {
    unaffected_path: String,
    err_code: u32,
    pub err_msg: String,
}

#[derive(Clone)]
pub struct DeleteRespOperationSuccessBuilder {
    affected_paths: Vec<String>,
    unaffected_path_errs: Vec<DeleteRespUnaffectedPathError>,
}

#[derive(Clone)]
pub enum DeleteRespOperationStatus {
    Failure { err_code: u32, err_msg: String },
    Success(DeleteRespOperationSuccessBuilder),
    None,
}

impl DeleteRespOperationStatus {
    pub fn new() -> Self {
        Self::None
    }

    pub fn set_failure(self, err_code: u32, err_msg: Option<String>) -> Self {
        Self::Failure {
            err_code,
            err_msg: err_msg.unwrap_or_else(|| usp_generator::get_err_msg(err_code).to_string()),
        }
    }

    pub fn set_success(
        self,
        affected_paths: Vec<String>,
        unaffected_path_errs: Vec<DeleteRespUnaffectedPathError>,
    ) -> Self {
        Self::Success(DeleteRespOperationSuccessBuilder {
            affected_paths,
            unaffected_path_errs,
        })
    }

    pub fn build<'a>(self) -> Result<OperationStatus<'a>> {
        match self {
            DeleteRespOperationStatus::Success(s) => Ok(OperationStatus {
                oper_status: oper_success(OperationSuccess {
                    affected_paths: s.affected_paths.into_iter().map(Into::into).collect(),
                    unaffected_path_errs: s
                        .unaffected_path_errs
                        .into_iter()
                        .map(|e| UnaffectedPathError {
                            unaffected_path: e.unaffected_path.into(),
                            err_code: e.err_code,
                            err_msg: if !e.err_msg.is_empty() {
                                e.err_msg.into()
                            } else {
                                usp_generator::get_err_msg(e.err_code).into()
                            },
                        })
                        .collect(),
                }),
            }),
            DeleteRespOperationStatus::Failure { err_code, err_msg } => Ok(OperationStatus {
                oper_status: oper_failure(OperationFailure {
                    err_code,
                    err_msg: err_msg.into(),
                }),
            }),
            DeleteRespOperationStatus::None => Err(anyhow::anyhow!("")),
        }
    }
}

#[derive(Clone)]
pub struct DeletedObjectResultsBuilder {
    requested_path: String,
    oper_status: DeleteRespOperationStatus,
}

impl DeletedObjectResultsBuilder {
    pub fn new(requested_path: String, oper_status: DeleteRespOperationStatus) -> Self {
        Self {
            requested_path,
            oper_status,
        }
    }

    pub fn build<'a>(self) -> Result<DeletedObjectResult<'a>> {
        Ok(DeletedObjectResult {
            requested_path: self.requested_path.into(),
            oper_status: Some(self.oper_status.build()?),
        })
    }
}

#[derive(Clone)]
pub struct DeleteRespBuilder {
    deleted_obj_results: Vec<DeletedObjectResultsBuilder>,
}

impl<'a> DeleteRespBuilder {
    pub fn new() -> Self {
        Self {
            deleted_obj_results: vec![],
        }
    }

    pub fn with_deleted_obj_results(
        mut self,
        deleted_obj_results: Vec<DeletedObjectResultsBuilder>,
    ) -> Self {
        self.deleted_obj_results = deleted_obj_results;
        self
    }

    pub fn build(self) -> Result<Body<'a>> {
        Ok(Body {
            msg_body: response({
                Response {
                    resp_type: delete_resp({
                        DeleteResp {
                            deleted_obj_results: self
                                .deleted_obj_results
                                .into_iter()
                                .map(|r| r.build())
                                .collect::<Result<Vec<_>>>()?,
                        }
                    }),
                }
            }),
        })
    }
}
