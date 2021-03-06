use crate::usp::*;
use crate::usp_record::*;

use anyhow::Context;
use serde::ser::{Serialize, SerializeStruct, Serializer};

impl Serialize for Record<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use crate::usp_decoder::*;
        use mod_Record::OneOfrecord_type::{no_session_context, session_context};

        let mut state = serializer.serialize_struct("Record", 7)?;
        state.serialize_field("version", &self.version)?;
        state.serialize_field("to_id", &self.to_id)?;
        state.serialize_field("from_id", &self.from_id)?;
        state.serialize_field("payload_security", &self.payload_security)?;
        state.serialize_field("mac_signature", &self.mac_signature)?;
        state.serialize_field("sender_cert", &self.sender_cert)?;

        match &self.record_type {
            no_session_context(context) => {
                let msg = try_decode_msg(&context.payload);
                if let Ok(msg) = msg {
                    state.serialize_field("payload", &msg)?;
                } else {
                    return Err(serde::ser::Error::custom(
                        msg.context("parsing of the protobuf USP Message failed")
                            .unwrap_err(),
                    ));
                }
            }
            session_context(context) => {
                state.serialize_field("payload", context)?;
            }
            _ => {
                return Err(serde::ser::Error::custom("Can't handle session_context!"));
            }
        }

        state.end()
    }
}

impl Serialize for mod_Record::PayloadSecurity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use mod_Record::PayloadSecurity::*;
        match *self {
            PLAINTEXT => serializer.serialize_unit_variant("PayloadSecurity", 0, "PLAINTEXT"),
            TLS12 => serializer.serialize_unit_variant("PayloadSecurity", 1, "TLS12"),
        }
    }
}

impl Serialize for SessionContextRecord<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SessionContextRecord", 7)?;
        state.serialize_field("session_id", &self.session_id)?;
        state.serialize_field("sequence_id", &self.sequence_id)?;
        state.serialize_field("expected_id", &self.expected_id)?;
        state.serialize_field("retransmit_id", &self.retransmit_id)?;
        state.serialize_field("payload_sar_state", &self.payload_sar_state)?;
        state.serialize_field("payloadrec_sar_state", &self.payloadrec_sar_state)?;
        state.serialize_field("payload", &self.payload)?;
        state.end()
    }
}

impl Serialize for mod_SessionContextRecord::PayloadSARState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use mod_SessionContextRecord::PayloadSARState::*;
        match *self {
            NONE => serializer.serialize_unit_variant("PayloadSARState", 0, "NONE"),
            BEGIN => serializer.serialize_unit_variant("PayloadSARState", 1, "BEGIN"),
            INPROCESS => serializer.serialize_unit_variant("PayloadSARState", 2, "INPROCESS"),
            COMPLETE => serializer.serialize_unit_variant("PayloadSARState", 3, "COMPLETE"),
        }
    }
}

impl Serialize for Msg<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Msg", 2)?;
        state.serialize_field("Header", &self.header)?;
        state.serialize_field("Body", &self.body)?;
        state.end()
    }
}

impl Serialize for Header<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Header", 2)?;
        state.serialize_field("msg_id", &self.msg_id)?;
        state.serialize_field("msg_type", &self.msg_type)?;
        state.end()
    }
}

impl Serialize for mod_Header::MsgType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use mod_Header::MsgType::*;

        match *self {
            ERROR => serializer.serialize_unit_variant("MsgType", 0, "ERROR"),
            GET => serializer.serialize_unit_variant("MsgType", 1, "GET"),
            GET_RESP => serializer.serialize_unit_variant("MsgType", 2, "GET_RESP"),
            NOTIFY => serializer.serialize_unit_variant("MsgType", 3, "NOTIFY"),
            SET => serializer.serialize_unit_variant("MsgType", 4, "SET"),
            SET_RESP => serializer.serialize_unit_variant("MsgType", 5, "SET_RESP"),
            OPERATE => serializer.serialize_unit_variant("MsgType", 6, "OPERATE"),
            OPERATE_RESP => serializer.serialize_unit_variant("MsgType", 7, "OPERATE_RESP"),
            ADD => serializer.serialize_unit_variant("MsgType", 8, "ADD"),
            ADD_RESP => serializer.serialize_unit_variant("MsgType", 9, "ADD_RESP"),
            DELETE => serializer.serialize_unit_variant("MsgType", 10, "DELETE"),
            DELETE_RESP => serializer.serialize_unit_variant("MsgType", 11, "DELETE_RESP"),
            GET_SUPPORTED_DM => {
                serializer.serialize_unit_variant("MsgType", 12, "GET_SUPPORTED_DM")
            }
            GET_SUPPORTED_DM_RESP => {
                serializer.serialize_unit_variant("MsgType", 13, "GET_SUPPORTED_DM_RESP")
            }
            GET_INSTANCES => serializer.serialize_unit_variant("MsgType", 14, "GET_INSTANCES"),
            GET_INSTANCES_RESP => {
                serializer.serialize_unit_variant("MsgType", 15, "GET_INSTANCES_RESP")
            }
            NOTIFY_RESP => serializer.serialize_unit_variant("MsgType", 16, "NOTIFY_RESP"),
            GET_SUPPORTED_PROTO => {
                serializer.serialize_unit_variant("MsgType", 17, "GET_SUPPORTED_PROTO")
            }
            GET_SUPPORTED_PROTO_RESP => {
                serializer.serialize_unit_variant("MsgType", 18, "GET_SUPPORTED_PROTO_RESP")
            }
        }
    }
}

impl Serialize for Body<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use mod_Body::OneOfmsg_body::*;

        let mut state = serializer.serialize_struct("Body", 1)?;
        match self.msg_body {
            request(ref m) => state.serialize_field("Request", m)?,
            response(ref m) => state.serialize_field("Response", m)?,
            error(ref m) => state.serialize_field("Error", m)?,
            None => return Err(serde::ser::Error::custom("USP Msg without body?!?")),
        }
        state.end()
    }
}

impl Serialize for Request<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use mod_Request::OneOfreq_type::*;

        let mut state = serializer.serialize_struct("Request", 1)?;
        match self.req_type {
            get(ref m) => state.serialize_field("Get", &m)?,
            get_supported_dm(ref m) => state.serialize_field("GetSupportedDM", &m)?,
            get_instances(ref m) => state.serialize_field("GetInstances", &m)?,
            set(ref m) => state.serialize_field("Set", &m)?,
            add(ref m) => state.serialize_field("Add", &m)?,
            delete(ref m) => state.serialize_field("Delete", &m)?,
            operate(ref m) => state.serialize_field("Operate", &m)?,
            notify(ref m) => state.serialize_field("Notify", &m)?,
            get_supported_protocol(ref m) => state.serialize_field("GetSupportedProtocol", &m)?,
            None => return Err(serde::ser::Error::custom("USP Request Msg without type?!?")),
        }
        state.end()
    }
}

impl Serialize for Response<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use mod_Response::OneOfresp_type::*;

        let mut state = serializer.serialize_struct("Response", 1)?;
        match self.resp_type {
            get_resp(ref m) => state.serialize_field("GetResp", &m)?,
            get_supported_dm_resp(ref m) => state.serialize_field("GetSupportedDMResp", &m)?,
            get_instances_resp(ref m) => state.serialize_field("GetInstancesResp", &m)?,
            set_resp(ref m) => state.serialize_field("SetResp", &m)?,
            add_resp(ref m) => state.serialize_field("AddResp", &m)?,
            delete_resp(ref m) => state.serialize_field("DeleteResp", &m)?,
            operate_resp(ref m) => state.serialize_field("OperateResp", &m)?,
            notify_resp(ref m) => state.serialize_field("NotifyResp", &m)?,
            get_supported_protocol_resp(ref m) => {
                state.serialize_field("GetSupportedProtocolResp", &m)?
            }
            None => {
                return Err(serde::ser::Error::custom(
                    "USP Response Msg without type?!?",
                ))
            }
        }
        state.end()
    }
}

impl Serialize for Error<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Error", 3)?;
        state.serialize_field("err_code", &self.err_code)?;
        state.serialize_field("err_msg", &self.err_msg)?;
        state.serialize_field("param_errs", &self.param_errs)?;
        state.end()
    }
}

impl Serialize for mod_Error::ParamError<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ParamError", 3)?;
        state.serialize_field("param_path", &self.param_path)?;
        state.serialize_field("err_code", &self.err_code)?;
        state.serialize_field("err_msg", &self.err_msg)?;
        state.end()
    }
}

impl Serialize for DeleteResp<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("DeleteResp", 1)?;
        state.serialize_field("deleted_obj_results", &self.deleted_obj_results)?;
        state.end()
    }
}

impl Serialize for Get<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Get", 1)?;
        state.serialize_field("param_paths", &self.param_paths)?;
        state.end()
    }
}

impl Serialize for GetSupportedDM<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("GetSupportedDM", 5)?;
        state.serialize_field("first_level_only", &self.first_level_only)?;
        state.serialize_field("return_commands", &self.return_commands)?;
        state.serialize_field("return_events", &self.return_events)?;
        state.serialize_field("return_params", &self.return_params)?;
        state.serialize_field("obj_paths", &self.obj_paths)?;
        state.end()
    }
}

impl Serialize for GetSupportedProtocol<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("GetSupportedProtocol", 1)?;
        state.serialize_field(
            "controller_supported_protocol_versions",
            &self.controller_supported_protocol_versions,
        )?;
        state.end()
    }
}

impl Serialize for Operate<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Operate", 4)?;
        state.serialize_field("command", &self.command)?;
        state.serialize_field("command_key", &self.command_key)?;
        state.serialize_field("send_resp", &self.send_resp)?;
        state.serialize_field("input_args", &self.input_args)?;
        state.end()
    }
}

impl Serialize for Notify<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use mod_Notify::OneOfnotification::*;

        let mut state = serializer.serialize_struct("Notify", 3)?;
        state.serialize_field("subscription_id", &self.subscription_id)?;
        state.serialize_field("send_resp", &self.send_resp)?;

        match self.notification {
            event(ref m) => state.serialize_field("event", &m)?,
            value_change(ref m) => state.serialize_field("value_change", &m)?,
            obj_creation(ref m) => state.serialize_field("obj_creation", &m)?,
            obj_deletion(ref m) => state.serialize_field("obj_deletion", &m)?,
            oper_complete(ref m) => state.serialize_field("oper_complete", &m)?,
            on_board_req(ref m) => state.serialize_field("on_board_req", &m)?,
            None => return Err(serde::ser::Error::custom("Unknown USP Notify type")),
        }

        state.end()
    }
}

impl Serialize for mod_Notify::Event<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Event", 3)?;
        state.serialize_field("obj_path", &self.obj_path)?;
        state.serialize_field("event_name", &self.event_name)?;
        state.serialize_field("params", &self.params)?;
        state.end()
    }
}

impl Serialize for mod_Notify::ValueChange<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ValueChange", 2)?;
        state.serialize_field("param_path", &self.param_path)?;
        state.serialize_field("param_value", &self.param_value)?;
        state.end()
    }
}

impl Serialize for mod_Notify::ObjectCreation<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ObjectCreation", 2)?;
        state.serialize_field("obj_path", &self.obj_path)?;
        state.serialize_field("unique_keys", &self.unique_keys)?;
        state.end()
    }
}

impl Serialize for mod_Notify::ObjectDeletion<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ObjectDeletion", 1)?;
        state.serialize_field("obj_path", &self.obj_path)?;
        state.end()
    }
}

impl Serialize for mod_Notify::OperationComplete<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use mod_Notify::mod_OperationComplete::OneOfoperation_resp::*;

        let mut state = serializer.serialize_struct("OperationComplete", 4)?;
        state.serialize_field("command_name", &self.command_name)?;
        state.serialize_field("obj_path", &self.obj_path)?;
        state.serialize_field("command_key", &self.command_key)?;

        match self.operation_resp {
            req_output_args(ref m) => state.serialize_field("req_output_args", m)?,
            cmd_failure(ref m) => state.serialize_field("cmd_failure", m)?,
            None => {
                return Err(serde::ser::Error::custom(
                    "USP Msg OperationStatus is unknown?!?",
                ))
            }
        }
        state.end()
    }
}

impl Serialize for mod_Notify::mod_OperationComplete::OutputArgs<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("OutputArgs", 1)?;
        state.serialize_field("output_args", &self.output_args)?;
        state.end()
    }
}

impl Serialize for mod_Notify::mod_OperationComplete::CommandFailure<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("CommandFailure", 2)?;
        state.serialize_field("err_code", &self.err_code)?;
        state.serialize_field("err_msg", &self.err_msg)?;
        state.end()
    }
}

impl Serialize for mod_Notify::OnBoardRequest<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("OnBoardRequest", 4)?;
        state.serialize_field("oui", &self.oui)?;
        state.serialize_field("product_class", &self.product_class)?;
        state.serialize_field("serial_number", &self.serial_number)?;
        state.serialize_field(
            "agent_supported_protocol_versions",
            &self.agent_supported_protocol_versions,
        )?;
        state.end()
    }
}

impl Serialize for Set<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Set", 2)?;
        state.serialize_field("allow_partial", &self.allow_partial)?;
        state.serialize_field("update_objs", &self.update_objs)?;
        state.end()
    }
}

impl Serialize for mod_Set::UpdateObject<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("UpdateObject", 2)?;
        state.serialize_field("obj_path", &self.obj_path)?;
        state.serialize_field("param_settings", &self.param_settings)?;
        state.end()
    }
}

impl Serialize for mod_Set::UpdateParamSetting<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("UpdateParamSetting", 3)?;
        state.serialize_field("param", &self.param)?;
        state.serialize_field("value", &self.value)?;
        state.serialize_field("required", &self.required)?;
        state.end()
    }
}

impl Serialize for Add<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Add", 2)?;
        state.serialize_field("allow_partial", &self.allow_partial)?;
        state.serialize_field("create_objs", &self.create_objs)?;
        state.end()
    }
}

impl Serialize for mod_Add::CreateObject<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("CreateObject", 2)?;
        state.serialize_field("obj_path", &self.obj_path)?;
        state.serialize_field("param_settings", &self.param_settings)?;
        state.end()
    }
}

impl Serialize for mod_Add::CreateParamSetting<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("CreateParamSetting", 3)?;
        state.serialize_field("param", &self.param)?;
        state.serialize_field("value", &self.value)?;
        state.serialize_field("required", &self.required)?;
        state.end()
    }
}

impl Serialize for Delete<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Delete", 2)?;
        state.serialize_field("allow_partial", &self.allow_partial)?;
        state.serialize_field("obj_paths", &self.obj_paths)?;
        state.end()
    }
}

impl Serialize for GetInstances<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("GetInstances", 2)?;
        state.serialize_field("first_level_only", &self.first_level_only)?;
        state.serialize_field("obj_paths", &self.obj_paths)?;
        state.end()
    }
}

impl Serialize for GetResp<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("GetResp", 1)?;
        state.serialize_field("req_path_results", &self.req_path_results)?;
        state.end()
    }
}

impl Serialize for GetSupportedDMResp<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("GetSupportedDMResp", 1)?;
        state.serialize_field("req_obj_results", &self.req_obj_results)?;
        state.end()
    }
}

impl Serialize for mod_GetSupportedDMResp::RequestedObjectResult<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("RequestedObjectResult", 5)?;
        state.serialize_field("req_obj_path", &self.req_obj_path)?;
        state.serialize_field("err_code", &self.err_code)?;
        state.serialize_field("err_msg", &self.err_msg)?;
        state.serialize_field("data_model_inst_uri", &self.data_model_inst_uri)?;
        state.serialize_field("supported_objs", &self.supported_objs)?;
        state.end()
    }
}

impl Serialize for mod_GetSupportedDMResp::SupportedObjectResult<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SupportedObjectResult", 6)?;
        state.serialize_field("supported_obj_path", &self.supported_obj_path)?;
        state.serialize_field("access", &self.access)?;
        state.serialize_field("is_multi_instance", &self.is_multi_instance)?;
        state.serialize_field("supported_commands", &self.supported_commands)?;
        state.serialize_field("supported_events", &self.supported_events)?;
        state.serialize_field("supported_params", &self.supported_params)?;
        state.end()
    }
}

impl Serialize for mod_GetSupportedDMResp::ObjAccessType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use mod_GetSupportedDMResp::ObjAccessType::*;

        match *self {
            OBJ_READ_ONLY => serializer.serialize_unit_variant("ObjAccessType", 0, "OBJ_READ_ONLY"),
            OBJ_ADD_DELETE => {
                serializer.serialize_unit_variant("ObjAccessType", 1, "OBJ_ADD_DELETE")
            }
            OBJ_ADD_ONLY => serializer.serialize_unit_variant("ObjAccessType", 2, "OBJ_ADD_ONLY"),
            OBJ_DELETE_ONLY => {
                serializer.serialize_unit_variant("ObjAccessType", 3, "OBJ_DELETE_ONLY")
            }
        }
    }
}

impl Serialize for mod_GetSupportedDMResp::SupportedCommandResult<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SupportedCommandResult", 3)?;
        state.serialize_field("command_name", &self.command_name)?;
        state.serialize_field("input_arg_names", &self.input_arg_names)?;
        state.serialize_field("output_arg_names", &self.output_arg_names)?;
        state.end()
    }
}

impl Serialize for mod_GetSupportedDMResp::SupportedEventResult<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SupportedEventResult", 2)?;
        state.serialize_field("event_name", &self.event_name)?;
        state.serialize_field("arg_names", &self.arg_names)?;
        state.end()
    }
}

impl Serialize for mod_GetSupportedDMResp::SupportedParamResult<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SupportedParamResult", 2)?;
        state.serialize_field("param_name", &self.param_name)?;
        state.serialize_field("access", &self.access)?;
        state.end()
    }
}

impl Serialize for mod_GetSupportedDMResp::ParamAccessType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use mod_GetSupportedDMResp::ParamAccessType::*;

        match *self {
            PARAM_READ_ONLY => {
                serializer.serialize_unit_variant("ParamAccessType", 0, "PARAM_READ_ONLY")
            }
            PARAM_READ_WRITE => {
                serializer.serialize_unit_variant("ParamAccessType", 1, "PARAM_READ_WRITE")
            }
            PARAM_WRITE_ONLY => {
                serializer.serialize_unit_variant("ParamAccessType", 2, "PARAM_WRITE_ONLY")
            }
        }
    }
}

impl Serialize for GetInstancesResp<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("GetInstancesResp", 1)?;
        state.serialize_field("req_path_results", &self.req_path_results)?;
        state.end()
    }
}

impl Serialize for mod_GetInstancesResp::RequestedPathResult<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("RequestedPathResult", 4)?;
        state.serialize_field("requested_path", &self.requested_path)?;
        state.serialize_field("err_code", &self.err_code)?;
        state.serialize_field("err_msg", &self.err_msg)?;
        state.serialize_field("curr_insts", &self.curr_insts)?;
        state.end()
    }
}

impl Serialize for mod_GetInstancesResp::CurrInstance<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("CurrInstance", 2)?;
        state.serialize_field("instantiated_obj_path", &self.instantiated_obj_path)?;
        state.serialize_field("unique_keys", &self.unique_keys)?;
        state.end()
    }
}

impl Serialize for SetResp<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SetResp", 1)?;
        state.serialize_field("updated_obj_results", &self.updated_obj_results)?;
        state.end()
    }
}

impl Serialize for mod_SetResp::UpdatedObjectResult<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("UpdatedObjectResult", 2)?;
        state.serialize_field("requested_path", &self.requested_path)?;
        state.serialize_field("oper_status", &self.oper_status)?;
        state.end()
    }
}

impl Serialize for mod_SetResp::mod_UpdatedObjectResult::OperationStatus<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use mod_SetResp::mod_UpdatedObjectResult::mod_OperationStatus::OneOfoper_status::*;

        let mut state = serializer.serialize_struct("OperationStatus", 1)?;
        match &self.oper_status {
            oper_success(ref m) => state.serialize_field("oper_success", m)?,
            oper_failure(ref m) => state.serialize_field("oper_failure", m)?,
            None => {
                return Err(serde::ser::Error::custom(
                    "USP Msg OperationStatus is unknown?!?",
                ))
            }
        }
        state.end()
    }
}

impl Serialize for mod_SetResp::mod_UpdatedObjectResult::mod_OperationStatus::OperationSuccess<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("OperationSuccess", 1)?;
        state.serialize_field("updated_inst_results", &self.updated_inst_results)?;
        state.end()
    }
}

impl Serialize
    for mod_SetResp::mod_UpdatedObjectResult::mod_OperationStatus::UpdatedInstanceResult<'_>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("UpdatedInstanceResult", 3)?;
        state.serialize_field("affected_path", &self.affected_path)?;
        state.serialize_field("updated_params", &self.updated_params)?;
        state.serialize_field("param_errs", &self.param_errs)?;
        state.end()
    }
}

impl Serialize
    for mod_SetResp::mod_UpdatedObjectResult::mod_OperationStatus::UpdatedInstanceFailure<'_>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("UpdatedInstanceFailure", 2)?;
        state.serialize_field("affected_path", &self.affected_path)?;
        state.serialize_field("param_errs", &self.param_errs)?;
        state.end()
    }
}

impl Serialize for mod_SetResp::mod_UpdatedObjectResult::mod_OperationStatus::OperationFailure<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("OperationFailure", 3)?;
        state.serialize_field("err_code", &self.err_code)?;
        state.serialize_field("err_msg", &self.err_msg)?;
        state.serialize_field("updated_inst_failures", &self.updated_inst_failures)?;
        state.end()
    }
}

impl Serialize for mod_SetResp::mod_UpdatedObjectResult::mod_OperationStatus::ParameterError<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ParameterError", 3)?;
        state.serialize_field("param", &self.param)?;
        state.serialize_field("err_code", &self.err_code)?;
        state.serialize_field("err_msg", &self.err_msg)?;
        state.end()
    }
}

impl Serialize for OperateResp<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("OperateResp", 1)?;
        state.serialize_field("operation_results", &self.operation_results)?;
        state.end()
    }
}

impl Serialize for mod_OperateResp::OperationResult<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use mod_OperateResp::mod_OperationResult::OneOfoperation_resp::*;

        let mut state = serializer.serialize_struct("OperationResult", 2)?;
        state.serialize_field("executed_command", &self.executed_command)?;
        match self.operation_resp {
            req_obj_path(ref m) => state.serialize_field("req_obj_path", m)?,
            req_output_args(ref m) => state.serialize_field("req_output_args", m)?,
            cmd_failure(ref m) => state.serialize_field("cmd_failure", m)?,
            None => {
                return Err(serde::ser::Error::custom(
                    "USP Msg OperationStatus is unknown?!?",
                ))
            }
        }
        state.end()
    }
}

impl Serialize for mod_OperateResp::mod_OperationResult::OutputArgs<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("OutputArgs", 1)?;
        state.serialize_field("output_args", &self.output_args)?;
        state.end()
    }
}

impl Serialize for mod_OperateResp::mod_OperationResult::CommandFailure<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("CommandFailure", 2)?;
        state.serialize_field("err_code", &self.err_code)?;
        state.serialize_field("err_msg", &self.err_msg)?;
        state.end()
    }
}

impl Serialize for NotifyResp<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("NotifyResp", 1)?;
        state.serialize_field("subscription_id", &self.subscription_id)?;
        state.end()
    }
}

impl Serialize for GetSupportedProtocolResp<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("GetSupportedProtocolResp", 1)?;
        state.serialize_field(
            "agent_supported_protocol_versions",
            &self.agent_supported_protocol_versions,
        )?;
        state.end()
    }
}

impl Serialize for mod_DeleteResp::DeletedObjectResult<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("DeletedObjectResult", 2)?;
        state.serialize_field("requested_path", &self.requested_path)?;
        state.serialize_field("oper_status", &self.oper_status)?;
        state.end()
    }
}

impl Serialize for mod_DeleteResp::mod_DeletedObjectResult::OperationStatus<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use mod_DeleteResp::mod_DeletedObjectResult::mod_OperationStatus::OneOfoper_status::*;

        let mut state = serializer.serialize_struct("OperationStatus", 1)?;
        match &self.oper_status {
            oper_success(ref m) => state.serialize_field("oper_success", m)?,
            oper_failure(ref m) => state.serialize_field("oper_failure", m)?,
            None => {
                return Err(serde::ser::Error::custom(
                    "USP Msg OperationStatus is unknown?!?",
                ))
            }
        }
        state.end()
    }
}

impl Serialize
    for mod_DeleteResp::mod_DeletedObjectResult::mod_OperationStatus::OperationSuccess<'_>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("OperationSuccess", 2)?;
        state.serialize_field("affected_paths", &self.affected_paths)?;
        state.serialize_field("unaffected_path_errs", &self.unaffected_path_errs)?;
        state.end()
    }
}

impl Serialize
    for mod_DeleteResp::mod_DeletedObjectResult::mod_OperationStatus::OperationFailure<'_>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("OperationFailure", 2)?;
        state.serialize_field("err_code", &self.err_code)?;
        state.serialize_field("err_msg", &self.err_msg)?;
        state.end()
    }
}

impl Serialize
    for mod_DeleteResp::mod_DeletedObjectResult::mod_OperationStatus::UnaffectedPathError<'_>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("UnaffectedPathError", 3)?;
        state.serialize_field("err_code", &self.err_code)?;
        state.serialize_field("err_msg", &self.err_msg)?;
        state.serialize_field("unaffected_path", &self.unaffected_path)?;
        state.end()
    }
}

impl Serialize for mod_GetResp::RequestedPathResult<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("RequestedPathResult", 4)?;
        state.serialize_field("requested_path", &self.requested_path)?;
        state.serialize_field("err_code", &self.err_code)?;
        state.serialize_field("err_msg", &self.err_msg)?;
        state.serialize_field("resolved_path_results", &self.resolved_path_results)?;
        state.end()
    }
}

impl Serialize for mod_GetResp::ResolvedPathResult<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ResolvedPathResult", 2)?;
        state.serialize_field("resolved_path", &self.resolved_path)?;
        state.serialize_field("result_params", &self.result_params)?;
        state.end()
    }
}

impl Serialize for AddResp<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("AddResp", 1)?;
        state.serialize_field("created_obj_results", &self.created_obj_results)?;
        state.end()
    }
}

impl Serialize for mod_AddResp::CreatedObjectResult<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("CreatedObjectResult", 2)?;
        state.serialize_field("requested_path", &self.requested_path)?;
        state.serialize_field("oper_status", &self.oper_status)?;
        state.end()
    }
}

impl Serialize for mod_AddResp::mod_CreatedObjectResult::OperationStatus<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use mod_AddResp::mod_CreatedObjectResult::mod_OperationStatus::OneOfoper_status::*;
        let mut state = serializer.serialize_struct("OperationStatus", 1)?;
        match self.oper_status {
            oper_success(ref m) => state.serialize_field("oper_success", m)?,
            oper_failure(ref m) => state.serialize_field("oper_failure", m)?,
            None => {
                return Err(serde::ser::Error::custom(
                    "USP Msg OperationStatus is unknown?!?",
                ))
            }
        }
        state.end()
    }
}

impl Serialize for mod_AddResp::mod_CreatedObjectResult::mod_OperationStatus::OperationFailure<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("OperationFailure", 2)?;
        state.serialize_field("err_code", &self.err_code)?;
        state.serialize_field("err_msg", &self.err_msg)?;
        state.end()
    }
}

impl Serialize for mod_AddResp::mod_CreatedObjectResult::mod_OperationStatus::OperationSuccess<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("OperationSuccess", 3)?;
        state.serialize_field("instantiated_path", &self.instantiated_path)?;
        state.serialize_field("param_errs", &self.param_errs)?;
        state.serialize_field("unique_keys", &self.unique_keys)?;
        state.end()
    }
}

impl Serialize for mod_AddResp::mod_CreatedObjectResult::mod_OperationStatus::ParameterError<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ParameterError", 3)?;
        state.serialize_field("param", &self.param)?;
        state.serialize_field("err_code", &self.err_code)?;
        state.serialize_field("err_msg", &self.err_msg)?;
        state.end()
    }
}
