use crate::usp::body::MsgBody;
use crate::usp::request::ReqType;
use crate::usp::response::RespType;
use crate::usp::{Body, GetSupportedProtocol, GetSupportedProtocolResp, Request, Response};

use anyhow::Result;

#[derive(Clone)]
pub struct GetSupportedProtocolBuilder {
    controller_supported_protocol_versions: String,
}

impl GetSupportedProtocolBuilder {
    #[must_use]
    pub const fn new(controller_supported_protocol_versions: String) -> Self {
        Self {
            controller_supported_protocol_versions,
        }
    }

    pub fn build(self) -> Result<Body> {
        Ok(Body {
            msg_body: Some(MsgBody::Request({
                Request {
                    req_type: Some(ReqType::GetSupportedProtocol({
                        GetSupportedProtocol {
                            controller_supported_protocol_versions: self
                                .controller_supported_protocol_versions,
                        }
                    })),
                }
            })),
        })
    }
}

#[derive(Clone)]
pub struct GetSupportedProtocolRespBuilder {
    agent_supported_protocol_versions: String,
}

impl GetSupportedProtocolRespBuilder {
    #[must_use]
    pub const fn new(agent_supported_protocol_versions: String) -> Self {
        Self {
            agent_supported_protocol_versions,
        }
    }

    pub fn build(self) -> Result<Body> {
        Ok(Body {
            msg_body: Some(MsgBody::Response({
                Response {
                    resp_type: Some(RespType::GetSupportedProtocolResp({
                        GetSupportedProtocolResp {
                            agent_supported_protocol_versions: self
                                .agent_supported_protocol_versions,
                        }
                    })),
                }
            })),
        })
    }
}
