use crate::usp::mod_Body::OneOfmsg_body::{request, response};
use crate::usp::mod_Request::OneOfreq_type::get_supported_protocol;
use crate::usp::mod_Response::OneOfresp_type::get_supported_protocol_resp;
use crate::usp::{Body, GetSupportedProtocol, GetSupportedProtocolResp, Request, Response};

use anyhow::Result;

#[derive(Clone)]
pub struct GetSupportedProtocolBuilder {
    controller_supported_protocol_versions: String,
}

impl GetSupportedProtocolBuilder {
    #[must_use] pub const fn new(controller_supported_protocol_versions: String) -> Self {
        Self {
            controller_supported_protocol_versions,
        }
    }

    pub fn build(self) -> Result<Body<'static>> {
        Ok(Body {
            msg_body: request({
                Request {
                    req_type: get_supported_protocol({
                        GetSupportedProtocol {
                            controller_supported_protocol_versions: self
                                .controller_supported_protocol_versions
                                .into(),
                        }
                    }),
                }
            }),
        })
    }
}

#[derive(Clone)]
pub struct GetSupportedProtocolRespBuilder {
    agent_supported_protocol_versions: String,
}

impl GetSupportedProtocolRespBuilder {
    #[must_use] pub const fn new(agent_supported_protocol_versions: String) -> Self {
        Self {
            agent_supported_protocol_versions,
        }
    }

    pub fn build(self) -> Result<Body<'static>> {
        Ok(Body {
            msg_body: response({
                Response {
                    resp_type: get_supported_protocol_resp({
                        GetSupportedProtocolResp {
                            agent_supported_protocol_versions: self
                                .agent_supported_protocol_versions
                                .into(),
                        }
                    }),
                }
            }),
        })
    }
}
