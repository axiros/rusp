use crate::usp::{Body, Error};

use crate::usp_generator;

use crate::usp::mod_Body::OneOfmsg_body::error;
use crate::usp::mod_Error::ParamError;

use anyhow::Result;

pub struct ErrorBuilder {
    code: u32,
    message: Option<String>,
    param_errs: Vec<(String, u32, String)>,
}

impl<'a> ErrorBuilder {
    pub const fn new() -> Self {
        Self {
            code: 0,
            message: None,
            param_errs: vec![],
        }
    }

    pub fn with_code(mut self, code: u32) -> Self {
        self.code = code;
        self
    }

    pub fn with_message(mut self, message: String) -> Self {
        self.message = Some(message);
        self
    }

    pub fn with_errs(mut self, errs: Vec<(String, u32, String)>) -> Self {
        self.param_errs = errs;
        self
    }

    pub fn build(self) -> Result<Body<'a>> {
        let message = self
            .message
            .clone()
            .unwrap_or_else(|| usp_generator::get_err_msg(self.code).to_string());

        let param_errs = self.param_errs;

        Ok(Body {
            msg_body: error({
                Error {
                    err_code: self.code,
                    err_msg: message.into(),
                    param_errs: param_errs
                        .into_iter()
                        .map(|(param_path, err_code, err_msg)| ParamError {
                            param_path: param_path.into(),
                            err_code,
                            err_msg: err_msg.into(),
                        })
                        .collect(),
                }
            }),
        })
    }
}
