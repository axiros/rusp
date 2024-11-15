use crate::usp_errors;
use std::collections::HashMap;

use crate::usp::mod_Body::OneOfmsg_body::{request, response};
use crate::usp::mod_OperateResp::mod_OperationResult::OutputArgs;
use crate::usp::mod_OperateResp::{
    mod_OperationResult::{CommandFailure, OneOfoperation_resp},
    OperationResult,
};
use crate::usp::mod_Request::OneOfreq_type::operate;
use crate::usp::mod_Response::OneOfresp_type::operate_resp;
use crate::usp::{Body, Operate, OperateResp, Request, Response};

use anyhow::Result;

#[derive(Clone)]
pub struct OperateBuilder {
    command: String,
    command_key: String,
    send_resp: bool,
    input_args: Vec<(String, String)>,
}

impl OperateBuilder {
    #[must_use]
    pub const fn new(command: String) -> Self {
        Self {
            command,
            command_key: String::new(),
            send_resp: false,
            input_args: vec![],
        }
    }

    #[must_use]
    pub fn with_command_key(mut self, command_key: String) -> Self {
        self.command_key = command_key;
        self
    }

    #[must_use]
    pub const fn with_send_resp(mut self, send_resp: bool) -> Self {
        self.send_resp = send_resp;
        self
    }

    #[must_use]
    pub fn with_input_args(mut self, input_args: Vec<(String, String)>) -> Self {
        self.input_args = input_args;
        self
    }

    pub fn build(self) -> Result<Body> {
        Ok(Body {
            msg_body: request({
                Request {
                    req_type: operate({
                        Operate {
                            command: self.command,
                            command_key: self.command_key,
                            send_resp: self.send_resp,
                            input_args: self.input_args.into_iter().collect(),
                        }
                    }),
                }
            }),
        })
    }
}

#[derive(Clone)]
pub enum OperateRespOperationResult {
    Failure {
        err_code: u32,
        err_msg: String,
    },
    Path {
        req_obj_path: String,
    },
    OutputArgs {
        output_args: HashMap<String, String>,
    },
    None,
}

#[derive(Clone)]
pub struct OperateRespResultBuilder {
    executed_command: String,
    operation_result: OperateRespOperationResult,
}

impl OperateRespResultBuilder {
    #[must_use]
    pub const fn new(executed_command: String) -> Self {
        Self {
            operation_result: OperateRespOperationResult::None,
            executed_command,
        }
    }

    #[must_use]
    pub fn set_failure(mut self, err_code: u32, err_msg: Option<String>) -> Self {
        self.operation_result = OperateRespOperationResult::Failure {
            err_code,
            err_msg: err_msg.unwrap_or_else(|| usp_errors::get_err_msg(err_code).to_string()),
        };
        self
    }

    #[must_use]
    pub fn set_path(mut self, req_obj_path: String) -> Self {
        self.operation_result = OperateRespOperationResult::Path { req_obj_path };
        self
    }

    #[must_use]
    pub fn set_output_args(mut self, output_args: Vec<(String, String)>) -> Self {
        self.operation_result = OperateRespOperationResult::OutputArgs {
            output_args: output_args.into_iter().collect(),
        };
        self
    }

    pub fn build(self) -> Result<OperationResult> {
        match self.operation_result {
            OperateRespOperationResult::OutputArgs { output_args } => Ok(OperationResult {
                operation_resp: OneOfoperation_resp::req_output_args(OutputArgs {
                    output_args: output_args.into_iter().collect::<HashMap<_, _>>(),
                }),
                executed_command: self.executed_command,
            }),
            OperateRespOperationResult::Failure { err_code, err_msg } => Ok(OperationResult {
                operation_resp: OneOfoperation_resp::cmd_failure(CommandFailure {
                    err_code,
                    err_msg,
                }),
                executed_command: self.executed_command,
            }),
            OperateRespOperationResult::None => Err(anyhow::anyhow!(
                "Need to have either OutputArgs or Path or Failure"
            )),
            OperateRespOperationResult::Path { req_obj_path } => Ok(OperationResult {
                operation_resp: OneOfoperation_resp::req_obj_path(req_obj_path),
                executed_command: self.executed_command,
            }),
        }
    }
}

#[derive(Clone)]
pub struct OperateRespBuilder {
    operation_results: Vec<OperateRespResultBuilder>,
}

impl OperateRespBuilder {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            operation_results: vec![],
        }
    }

    #[must_use]
    pub fn with_operation_results(
        mut self,
        operation_results: Vec<OperateRespResultBuilder>,
    ) -> Self {
        self.operation_results = operation_results;
        self
    }

    pub fn build(self) -> Result<Body> {
        Ok(Body {
            msg_body: response({
                Response {
                    resp_type: operate_resp(OperateResp {
                        operation_results: self
                            .operation_results
                            .into_iter()
                            .map(OperateRespResultBuilder::build)
                            .collect::<Result<Vec<_>>>()?,
                    }),
                }
            }),
        })
    }
}
