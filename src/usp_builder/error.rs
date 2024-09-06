use crate::usp::{Body, Error};

use crate::usp_errors;

use crate::usp::mod_Body::OneOfmsg_body::error;
use crate::usp::mod_Error::ParamError;

use anyhow::Result;

#[derive(Clone)]
pub struct ErrorBuilder {
    code: u32,
    message: Option<String>,
    param_errs: Vec<(String, u32, String)>,
}

impl ErrorBuilder {
    #[must_use] pub const fn new() -> Self {
        Self {
            code: 0,
            message: None,
            param_errs: vec![],
        }
    }

    #[must_use] pub fn set_err(mut self, code: u32, message: Option<String>) -> Self {
        self.code = code;
        self.message = message;
        self
    }

    #[must_use] pub fn with_param_errs(mut self, errs: Vec<(String, u32, String)>) -> Self {
        self.param_errs = errs;
        self
    }

    pub fn build(self) -> Result<Body<'static>> {
        let message = self
            .message
            .clone()
            .unwrap_or_else(|| usp_errors::get_err_msg(self.code).to_string());

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
