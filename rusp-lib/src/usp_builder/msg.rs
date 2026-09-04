use crate::usp::body::MsgBody;
use crate::usp::request::ReqType;
use crate::usp::response::RespType;
use crate::usp::Body;
use crate::usp::Header;
use crate::usp::Msg;

use anyhow::anyhow;
use anyhow::{Context, Result};

#[derive(Clone)]
pub struct MsgBuilder {
    msg_id: Option<String>,
    body: Option<Body>,
}

impl MsgBuilder {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            msg_id: None,
            body: None,
        }
    }

    #[must_use]
    pub fn with_msg_id(mut self, msg_id: String) -> Self {
        self.msg_id = Some(msg_id);
        self
    }

    #[must_use]
    pub fn with_body(mut self, body: Body) -> Self {
        self.body = Some(body);
        self
    }

    pub fn build(self) -> Result<Msg> {
        use crate::usp::header::MsgType;

        let msg_id = self
            .msg_id
            .with_context(|| "Cannot produce USP Msg without msg_id")?;
        let body = self
            .body
            .with_context(|| "Cannot produce USP Msg without msg_body")?;

        let msg_type = match &body.msg_body {
            Some(MsgBody::Request(req)) => match &req.req_type {
                Some(ReqType::Get(_)) => MsgType::Get,
                Some(ReqType::GetSupportedDm(_)) => MsgType::GetSupportedDm,
                Some(ReqType::GetInstances(_)) => MsgType::GetInstances,
                Some(ReqType::Set(_)) => MsgType::Set,
                Some(ReqType::Add(_)) => MsgType::Add,
                Some(ReqType::Delete(_)) => MsgType::Delete,
                Some(ReqType::Operate(_)) => MsgType::Operate,
                Some(ReqType::Notify(_)) => MsgType::Notify,
                Some(ReqType::GetSupportedProtocol(_)) => MsgType::GetSupportedProto,
                Some(ReqType::Register(_)) => MsgType::Register,
                Some(ReqType::Deregister(_)) => MsgType::Deregister,
                None => anyhow::bail!("Request type can't be None"),
            },
            Some(MsgBody::Response(resp)) => match &resp.resp_type {
                Some(RespType::GetResp(_)) => MsgType::GetResp,
                Some(RespType::GetSupportedDmResp(_)) => MsgType::GetSupportedDmResp,
                Some(RespType::GetInstancesResp(_)) => MsgType::GetInstancesResp,
                Some(RespType::SetResp(_)) => MsgType::SetResp,
                Some(RespType::AddResp(_)) => MsgType::AddResp,
                Some(RespType::DeleteResp(_)) => MsgType::DeleteResp,
                Some(RespType::OperateResp(_)) => MsgType::OperateResp,
                Some(RespType::NotifyResp(_)) => MsgType::NotifyResp,
                Some(RespType::GetSupportedProtocolResp(_)) => MsgType::GetSupportedProtoResp,
                Some(RespType::RegisterResp(_)) => MsgType::RegisterResp,
                Some(RespType::DeregisterResp(_)) => MsgType::DeregisterResp,
                None => Err(anyhow!("Response type can't be None"))?,
            },
            Some(MsgBody::Error(_)) => MsgType::Error,
            None => Err(anyhow!("Body type can't be None"))?,
        };

        Ok(Msg {
            header: Some(Header {
                msg_id,
                msg_type: msg_type as i32,
            }),
            body: Some(body),
        })
    }
}
