use crate::usp::mod_Body::OneOfmsg_body::request;
use crate::usp::mod_Request::OneOfreq_type::operate;
use crate::usp::{Body, Operate, Request};

use anyhow::Result;

#[derive(Clone)]
pub struct OperateBuilder {
    command: String,
    command_key: String,
    send_resp: bool,
    input_args: Vec<(String, String)>,
}

impl<'a> OperateBuilder {
    pub const fn new(command: String, command_key: String) -> Self {
        Self {
            command,
            command_key,
            send_resp: false,
            input_args: vec![],
        }
    }

    pub fn with_send_resp(mut self, send_resp: bool) -> Self {
        self.send_resp = send_resp;
        self
    }

    pub fn with_input_args(mut self, input_args: Vec<(String, String)>) -> Self {
        self.input_args = input_args;
        self
    }

    pub fn build(self) -> Result<Body<'a>> {
        Ok(Body {
            msg_body: request({
                Request {
                    req_type: operate({
                        Operate {
                            command: self.command.into(),
                            command_key: self.command_key.into(),
                            send_resp: self.send_resp,
                            input_args: self
                                .input_args
                                .into_iter()
                                .map(|(k, v)| (k.into(), v.into()))
                                .collect(),
                        }
                    }),
                }
            }),
        })
    }
}
