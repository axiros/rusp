use crate::usp::*;
use crate::usp_record::*;

use std::fmt;

const INDENT: usize = 2;

impl fmt::Display for Record<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use crate::usp_decoder::*;
        use mod_Record::OneOfrecord_type::no_session_context;

        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}Record {{", "", aby = aby)?;
        writeln!(f, "{:aby$}version: {}", "", self.version, aby = aby2)?;
        writeln!(f, "{:aby$}to_id: {}", "", self.to_id, aby = aby2)?;
        writeln!(f, "{:aby$}from_id: {}", "", self.from_id, aby = aby2)?;
        writeln!(
            f,
            "{:aby$}payload_security: {}",
            "",
            self.payload_security,
            aby = aby2
        )?;
        writeln!(
            f,
            "{:aby$}mac_signature: {:#?}",
            "",
            self.mac_signature,
            aby = aby2
        )?;
        writeln!(
            f,
            "{:aby$}sender_cert: {:#?}",
            "",
            self.sender_cert,
            aby = aby2
        )?;
        if let no_session_context(context) = &self.record_type {
            write!(f, "{:aby$}", decode_msg(&context.payload), aby = aby2)?;
        } else {
            writeln!(f, "{:aby$}can't handle session_context!", "", aby = aby2)?;
        };

        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl fmt::Display for mod_Record::PayloadSecurity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        // TODO: Implement
        writeln!(f, "{:aby$}{:#?}", "", self, aby = aby2)
    }
}

impl fmt::Display for Msg<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;
        writeln!(f, "{:aby$}Msg {{", "", aby = aby)?;
        write!(f, "{:aby$}", self.header.clone().unwrap(), aby = aby2)?;
        write!(f, "{:aby$}", self.body.clone().unwrap(), aby = aby2)?;
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl fmt::Display for Header<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}Header: {{", "", aby = aby)?;
        writeln!(f, "{:aby$}msg_id: {}", "", self.msg_id, aby = aby2)?;
        write!(f, "{:aby$}msg_type: {}", "", self.msg_type, aby = aby2)?;
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl fmt::Display for mod_Header::MsgType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use mod_Header::MsgType::*;

        let aby = f.width().unwrap_or(0);

        writeln!(
            f,
            "{:aby$}{}",
            "",
            match self {
                ERROR => "ERROR",
                GET => "GET",
                GET_RESP => "GET_RESP",
                NOTIFY => "NOTIFY",
                SET => "SET",
                SET_RESP => "SET_RESP",
                OPERATE => "OPERATE",
                OPERATE_RESP => "OPERATE_RESP",
                ADD => "ADD",
                ADD_RESP => "ADD_RESP",
                DELETE => "DELETE",
                DELETE_RESP => "DELETE_RESP",
                GET_SUPPORTED_DM => "GET_SUPPORTED_DM",
                GET_SUPPORTED_DM_RESP => "GET_SUPPORTED_DM_RESP",
                GET_INSTANCES => "GET_INSTANCES",
                GET_INSTANCES_RESP => "GET_INSTANCES_RESP",
                NOTIFY_RESP => "NOTIFY_RESP",
                GET_SUPPORTED_PROTO => "GET_SUPPORTED_PROTO",
                GET_SUPPORTED_PROTO_RESP => "GET_SUPPORTED_PROTO_RESP",
            },
            aby = aby
        )
    }
}

impl fmt::Display for Body<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use mod_Body::OneOfmsg_body::*;

        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}Body: {{", "", aby = aby)?;
        match self.msg_body {
            request(ref m) => write!(f, "{:aby$}", m, aby = aby2),
            response(ref m) => write!(f, "{:aby$}", m, aby = aby2),
            error(ref m) => write!(f, "{:aby$}", m, aby = aby2),
            None => writeln!(f, "{:aby$}None", "", aby = aby2),
        }?;
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl fmt::Display for Request<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use mod_Request::OneOfreq_type::*;

        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}Request: {{", "", aby = aby)?;
        match self.req_type {
            get(ref m) => write!(f, "{:aby$}", m, aby = aby2),
            get_supported_dm(ref m) => write!(f, "{:aby$}", m, aby = aby2),
            get_instances(ref m) => write!(f, "{:aby$}", m, aby = aby2),
            set(ref m) => write!(f, "{:aby$}", m, aby = aby2),
            add(ref m) => write!(f, "{:aby$}", m, aby = aby2),
            delete(ref m) => write!(f, "{:aby$}", m, aby = aby2),
            operate(ref m) => write!(f, "{:aby$}", m, aby = aby2),
            notify(ref m) => write!(f, "{:aby$}", m, aby = aby2),
            get_supported_protocol(ref m) => write!(f, "{:aby$}", m, aby = aby2),
            None => writeln!(f, "{:aby$}None", "", aby = aby2),
        }?;
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl fmt::Display for Response<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use mod_Response::OneOfresp_type::*;

        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}Response: {{", "", aby = aby)?;
        match self.resp_type {
            get_resp(ref m) => write!(f, "{:aby$}", m, aby = aby2),
            get_supported_dm_resp(ref m) => write!(f, "{:aby$}", m, aby = aby2),
            get_instances_resp(ref m) => write!(f, "{:aby$}", m, aby = aby2),
            set_resp(ref m) => write!(f, "{:aby$}", m, aby = aby2),
            add_resp(ref m) => write!(f, "{:aby$}", m, aby = aby2),
            delete_resp(ref m) => write!(f, "{:aby$}", m, aby = aby2),
            operate_resp(ref m) => write!(f, "{:aby$}", m, aby = aby2),
            notify_resp(ref m) => write!(f, "{:aby$}", m, aby = aby2),
            get_supported_protocol_resp(ref m) => write!(f, "{:aby$}", m, aby = aby2),
            None => writeln!(f, "{:aby$}None", "", aby = aby2),
        }?;
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl fmt::Display for Error<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;
        let aby3 = aby2 + INDENT;

        writeln!(f, "{:aby$}Error: {{", "", aby = aby)?;
        writeln!(f, "{:aby$}err_code: {}", "", self.err_code, aby = aby2)?;
        writeln!(f, "{:aby$}err_msg: {}", "", self.err_msg, aby = aby2)?;
        writeln!(f, "{:aby$}param_errs: [", "", aby = aby2)?;
        for result in self.param_errs.iter() {
            write!(f, "{:aby$}", result, aby = aby3)?;
        }
        writeln!(f, "{:aby$}]", "", aby = aby2)?;
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl fmt::Display for mod_Error::ParamError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        // TODO: Implement
        writeln!(f, "{:aby$}{:#?}", "", self.param_path, aby = aby2)
    }
}

impl fmt::Display for DeleteResp<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;
        let aby3 = aby2 + INDENT;

        writeln!(f, "{:aby$}DeleteResp: {{", "", aby = aby)?;
        writeln!(f, "{:aby$}deleted_obj_results: [", "", aby = aby2)?;
        for res in self.deleted_obj_results.iter() {
            write!(f, "{:aby$}", res, aby = aby3)?;
        }
        writeln!(f, "{:aby$}]", "", aby = aby2)?;
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl fmt::Display for Get<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let aby = f.width().unwrap_or(0);

        writeln!(
            f,
            "{:aby$}Get {{ param_paths: [ {} ] }}",
            "",
            self.param_paths.join(", "),
            aby = aby
        )
    }
}

impl fmt::Display for GetSupportedDM<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}GetSupportedDM: {{", "", aby = aby)?;
        writeln!(
            f,
            "{:aby$}first_level_only: {:aby$}",
            "",
            self.first_level_only,
            aby = aby2
        )?;
        writeln!(
            f,
            "{:aby$}return_commands: {:aby$}",
            "",
            self.return_commands,
            aby = aby2
        )?;
        writeln!(
            f,
            "{:aby$}return_events: {:aby$}",
            "",
            self.return_events,
            aby = aby2
        )?;
        writeln!(
            f,
            "{:aby$}return_params: {:aby$}",
            "",
            self.return_params,
            aby = aby2
        )?;
        writeln!(
            f,
            "{:aby$}obj_paths: [ {} ]",
            "",
            self.obj_paths.join(", "),
            aby = aby2
        )?;
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl fmt::Display for GetSupportedProtocol<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}GetSupportedProtocol: {{", "", aby = aby)?;
        writeln!(
            f,
            "{:aby$}controller_supported_protocol_versions: {:aby$}",
            "",
            self.controller_supported_protocol_versions,
            aby = aby2
        )?;
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl fmt::Display for Operate<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;
        let aby3 = aby2 + INDENT;

        writeln!(f, "{:aby$}Operate: {{", "", aby = aby)?;
        writeln!(f, "{:aby$}command: {:aby$}", "", self.command, aby = aby2)?;
        writeln!(
            f,
            "{:aby$}command_key: {:aby$}",
            "",
            self.command_key,
            aby = aby2
        )?;
        writeln!(
            f,
            "{:aby$}send_resp: {:aby$}",
            "",
            self.send_resp,
            aby = aby2
        )?;
        writeln!(f, "{:aby$}input_args: {{", "", aby = aby2)?;
        for (k, v) in self.input_args.iter() {
            writeln!(f, "{:aby$}{}: {}", "", k, v, aby = aby3)?;
        }
        writeln!(f, "{:aby$}}}", "", aby = aby2)?;
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl fmt::Display for Notify<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use mod_Notify::OneOfnotification::*;
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}Notify: {{", "", aby = aby)?;
        writeln!(
            f,
            "{:aby$}subscription_id: {}",
            "",
            self.subscription_id,
            aby = aby2
        )?;
        writeln!(f, "{:aby$}send_resp: {}", "", self.send_resp, aby = aby2)?;
        match self.notification {
            event(ref m) => write!(f, "{:aby$}", m, aby = aby2),
            value_change(ref m) => write!(f, "{:aby$}", m, aby = aby2),
            obj_creation(ref m) => write!(f, "{:aby$}", m, aby = aby2),
            obj_deletion(ref m) => write!(f, "{:aby$}", m, aby = aby2),
            oper_complete(ref m) => write!(f, "{:aby$}", m, aby = aby2),
            on_board_req(ref m) => write!(f, "{:aby$}", m, aby = aby2),
            None => writeln!(f, "{:aby$}None", "", aby = aby2),
        }?;
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl fmt::Display for mod_Notify::Event<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;
        let aby3 = aby2 + INDENT;

        writeln!(f, "{:aby$}Event: {{", "", aby = aby)?;
        writeln!(f, "{:aby$}obj_path: {}", "", self.obj_path, aby = aby2)?;
        writeln!(f, "{:aby$}event_name: {}", "", self.event_name, aby = aby2)?;
        writeln!(f, "{:aby$}params: {{", "", aby = aby2)?;
        for (k, v) in self.params.iter() {
            writeln!(f, "{:aby$}{}: {}", "", k, v, aby = aby3)?;
        }
        writeln!(f, "{:aby$}}}", "", aby = aby2)?;
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl fmt::Display for mod_Notify::ValueChange<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}ValueChange: {{", "", aby = aby)?;
        writeln!(f, "{:aby$}param_path: {}", "", self.param_path, aby = aby2)?;
        writeln!(
            f,
            "{:aby$}param_value: {}",
            "",
            self.param_value,
            aby = aby2
        )?;
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl fmt::Display for mod_Notify::ObjectCreation<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;
        let aby3 = aby2 + INDENT;

        writeln!(f, "{:aby$}ObjectCreation: {{", "", aby = aby)?;
        writeln!(f, "{:aby$}obj_path: {}", "", self.obj_path, aby = aby2)?;
        writeln!(f, "{:aby$}unique_keys: {{", "", aby = aby2)?;
        for (k, v) in self.unique_keys.iter() {
            writeln!(f, "{:aby$}{}: {}", "", k, v, aby = aby3)?;
        }
        writeln!(f, "{:aby$}}}", "", aby = aby2)?;
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl fmt::Display for mod_Notify::ObjectDeletion<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}ObjectDeletion: {{", "", aby = aby)?;
        writeln!(f, "{:aby$}obj_path: {}", "", self.obj_path, aby = aby2)?;
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl fmt::Display for mod_Notify::OperationComplete<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}OperationComplete: {{", "", aby = aby)?;
        writeln!(
            f,
            "{:aby$}command_name: {}",
            "",
            self.command_name,
            aby = aby2
        )?;
        writeln!(f, "{:aby$}obj_path: {}", "", self.obj_path, aby = aby2)?;
        writeln!(
            f,
            "{:aby$}command_key: {}",
            "",
            self.command_key,
            aby = aby2
        )?;
        writeln!(
            f,
            "{:aby$}operation_resp: {:?}",
            "",
            self.operation_resp,
            aby = aby2
        )?;
        writeln!(f, "{:aby$}}}", "", aby = aby2)?;
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl fmt::Display for mod_Notify::OnBoardRequest<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}OnBoardRequest: {{", "", aby = aby)?;
        writeln!(f, "{:aby$}oui: {}", "", self.oui, aby = aby2)?;
        writeln!(
            f,
            "{:aby$}product_class: {}",
            "",
            self.product_class,
            aby = aby2
        )?;
        writeln!(
            f,
            "{:aby$}serial_number: {}",
            "",
            self.serial_number,
            aby = aby2
        )?;
        writeln!(
            f,
            "{:aby$}agent_supported_protocol_versions: {}",
            "",
            self.agent_supported_protocol_versions,
            aby = aby2
        )?;
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl fmt::Display for Set<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}Set: {{", "", aby = aby)?;
        writeln!(
            f,
            "{:aby$}allow_partial: {}",
            "",
            self.allow_partial,
            aby = aby2
        )?;
        for result in self.update_objs.iter() {
            write!(f, "{:aby$}", result, aby = aby2)?;
        }
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl fmt::Display for mod_Set::UpdateObject<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}UpdateObject: {{", "", aby = aby)?;
        writeln!(f, "{:aby$}obj_path: {}", "", self.obj_path, aby = aby2)?;
        for ps in self.param_settings.iter() {
            write!(f, "{:aby$}", ps, aby = aby2)?;
        }
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl fmt::Display for mod_Set::UpdateParamSetting<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}UpdateParamSetting: {{", "", aby = aby)?;
        writeln!(f, "{:aby$}param: {}", "", self.param, aby = aby2)?;
        writeln!(f, "{:aby$}value: {}", "", self.value, aby = aby2)?;
        writeln!(f, "{:aby$}required: {}", "", self.required, aby = aby2)?;
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl fmt::Display for Add<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}Add: {{", "", aby = aby)?;
        writeln!(
            f,
            "{:aby$}allow_partial: {}",
            "",
            self.allow_partial,
            aby = aby2
        )?;
        for result in self.create_objs.iter() {
            write!(f, "{:aby$}", result, aby = aby2)?;
        }
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl fmt::Display for mod_Add::CreateObject<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}CreateObject: {{", "", aby = aby)?;
        writeln!(f, "{:aby$}obj_path: {}", "", self.obj_path, aby = aby2)?;
        for ps in self.param_settings.iter() {
            write!(f, "{:aby$}", ps, aby = aby2)?;
        }
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl fmt::Display for mod_Add::CreateParamSetting<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}CreateParamSetting: {{", "", aby = aby)?;
        writeln!(f, "{:aby$}param: {}", "", self.param, aby = aby2)?;
        writeln!(f, "{:aby$}value: {}", "", self.value, aby = aby2)?;
        writeln!(f, "{:aby$}required: {}", "", self.required, aby = aby2)?;
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl fmt::Display for Delete<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}Delete: {{", "", aby = aby)?;
        writeln!(
            f,
            "{:aby$}allow_partial: {}",
            "",
            self.allow_partial,
            aby = aby2
        )?;
        writeln!(
            f,
            "{:aby$}obj_paths: [ {} ]",
            "",
            self.obj_paths.join(", "),
            aby = aby2
        )?;
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl fmt::Display for GetInstances<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}GetInstances: {{", "", aby = aby)?;
        writeln!(
            f,
            "{:aby$}first_level_only: {}",
            "",
            self.first_level_only,
            aby = aby2
        )?;
        writeln!(
            f,
            "{:aby$}obj_paths: [ {} ]",
            "",
            self.obj_paths.join(", "),
            aby = aby2
        )?;
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl fmt::Display for GetResp<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}GetResp: [", "", aby = aby)?;
        for result in self.req_path_results.iter() {
            write!(f, "{:aby$}", result, aby = aby2)?;
        }

        writeln!(f, "{:aby$}]", "", aby = aby)
    }
}

impl fmt::Display for GetSupportedDMResp<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}GetSupportedDMResp: [", "", aby = aby)?;
        for result in self.req_obj_results.iter() {
            write!(f, "{:aby$}", result, aby = aby2)?;
        }

        writeln!(f, "{:aby$}]", "", aby = aby)
    }
}

impl fmt::Display for mod_GetSupportedDMResp::RequestedObjectResult<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;
        let aby3 = aby2 + INDENT;

        writeln!(f, "{:aby$}RequestedObjectResult: {{", "", aby = aby)?;
        writeln!(
            f,
            "{:aby$}req_obj_path: {}",
            "",
            self.req_obj_path,
            aby = aby2
        )?;
        writeln!(f, "{:aby$}err_code: {}", "", self.err_code, aby = aby2)?;
        writeln!(f, "{:aby$}err_msg: {}", "", self.err_msg, aby = aby2)?;
        writeln!(
            f,
            "{:aby$}data_model_inst_uri: {}",
            "",
            self.data_model_inst_uri,
            aby = aby2
        )?;
        writeln!(f, "{:aby$}supported_objs: [", "", aby = aby2)?;
        for result in self.supported_objs.iter() {
            write!(f, "{:aby$}", result, aby = aby3)?;
        }
        writeln!(f, "{:aby$}]", "", aby = aby2)?;
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl fmt::Display for mod_GetSupportedDMResp::SupportedObjectResult<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}SupportedObjectResult: {{", "", aby = aby)?;
        writeln!(
            f,
            "{:aby$}supported_obj_path: {}",
            "",
            self.supported_obj_path,
            aby = aby2
        )?;
        writeln!(f, "{:aby$}access: {:#?}", "", self.access, aby = aby2)?;
        writeln!(
            f,
            "{:aby$}is_multi_instance: {}",
            "",
            self.is_multi_instance,
            aby = aby2
        )?;
        for result in self.supported_commands.iter() {
            write!(f, "{:aby$}", result, aby = aby2)?;
        }
        for result in self.supported_events.iter() {
            write!(f, "{:aby$}", result, aby = aby2)?;
        }
        for result in self.supported_params.iter() {
            write!(f, "{:aby$}", result, aby = aby2)?;
        }
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl fmt::Display for mod_GetSupportedDMResp::SupportedCommandResult<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        // TODO: Implement
        writeln!(f, "{:aby$}{:#?}", "", self, aby = aby2)
    }
}

impl fmt::Display for mod_GetSupportedDMResp::SupportedEventResult<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        // TODO: Implement
        writeln!(f, "{:aby$}{:#?}", "", self, aby = aby2)
    }
}

impl fmt::Display for mod_GetSupportedDMResp::SupportedParamResult<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}SupportedParamResult {{", "", aby = aby)?;
        writeln!(f, "{:aby$}param_name: {}", "", self.param_name, aby = aby2)?;
        writeln!(f, "{:aby$}access: {:#?}", "", self.access, aby = aby2)?;
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl fmt::Display for GetInstancesResp<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}GetInstancesResp: [", "", aby = aby)?;
        for result in self.req_path_results.iter() {
            write!(f, "{:aby$}", result, aby = aby2)?;
        }

        writeln!(f, "{:aby$}]", "", aby = aby)
    }
}

impl fmt::Display for mod_GetInstancesResp::RequestedPathResult<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}RequestedPathResult: {{", "", aby = aby)?;
        writeln!(
            f,
            "{:aby$}requested_path: {}",
            "",
            self.requested_path,
            aby = aby2
        )?;
        writeln!(f, "{:aby$}err_code: {}", "", self.err_code, aby = aby2)?;
        writeln!(f, "{:aby$}err_msg: {}", "", self.err_msg, aby = aby2)?;
        for result in self.curr_insts.iter() {
            write!(f, "{:aby$}", result, aby = aby2)?;
        }
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl fmt::Display for mod_GetInstancesResp::CurrInstance<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;
        let aby3 = aby2 + INDENT;

        writeln!(f, "{:aby$}{{", "", aby = aby)?;
        writeln!(
            f,
            "{:aby$}instantiated_obj_path: {}",
            "",
            self.instantiated_obj_path,
            aby = aby2
        )?;
        writeln!(f, "{:aby$}unique_keys: {{", "", aby = aby2)?;
        for (k, v) in self.unique_keys.iter() {
            writeln!(f, "{:aby$}{}: {}", "", k, v, aby = aby3)?;
        }
        writeln!(f, "{:aby$}}}", "", aby = aby2)?;
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl fmt::Display for SetResp<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}SetResp: [", "", aby = aby)?;
        for res in self.updated_obj_results.iter() {
            write!(f, "{:aby$}", res, aby = aby2)?;
        }
        writeln!(f, "{:aby$}]", "", aby = aby)
    }
}

impl fmt::Display for mod_SetResp::UpdatedObjectResult<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}UpdatedObjectResult: {{", "", aby = aby)?;
        writeln!(
            f,
            "{:aby$}requested_path: {}",
            "",
            self.requested_path,
            aby = aby2
        )?;
        write!(
            f,
            "{:aby$}oper_status: {:aby$}",
            "",
            self.oper_status.clone().unwrap(),
            aby = aby2
        )?;
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl fmt::Display for mod_SetResp::mod_UpdatedObjectResult::OperationStatus<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        use mod_SetResp::mod_UpdatedObjectResult::mod_OperationStatus::OneOfoper_status::*;

        match &self.oper_status {
            status => match status {
                oper_success(ref m) => write!(f, "{:#aby$}", m, aby = aby),
                oper_failure(ref m) => write!(f, "{:#aby$}", m, aby = aby),
                None => writeln!(f, "{:aby$}None", "", aby = aby2),
            },
        }
    }
}

impl fmt::Display
    for mod_SetResp::mod_UpdatedObjectResult::mod_OperationStatus::OperationSuccess<'_>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;
        let aby3 = aby2 + INDENT;

        writeln!(f, "OperationSuccess: {{")?;
        writeln!(f, "{:aby$}updated_inst_results: [", "", aby = aby2)?;
        for r in self.updated_inst_results.iter() {
            write!(f, "{:#aby$}", r, aby = aby3)?;
        }
        writeln!(f, "{:aby$}]", "", aby = aby2)?;
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl fmt::Display
    for mod_SetResp::mod_UpdatedObjectResult::mod_OperationStatus::UpdatedInstanceResult<'_>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;
        let aby3 = aby2 + INDENT;

        writeln!(f, "{:aby$}UpdatedInstanceResult: {{", "", aby = aby)?;
        writeln!(
            f,
            "{:aby$}affected_path: {}",
            "",
            self.affected_path,
            aby = aby2
        )?;
        writeln!(f, "{:aby$}updated_params: {{", "", aby = aby2)?;
        for (k, v) in self.updated_params.iter() {
            writeln!(f, "{:aby$}{}: {}", "", k, v, aby = aby3)?;
        }
        writeln!(f, "{:aby$}}}", "", aby = aby2)?;
        for r in self.param_errs.iter() {
            write!(f, "{:#aby$?}", r, aby = aby3)?;
        }
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl fmt::Display
    for mod_SetResp::mod_UpdatedObjectResult::mod_OperationStatus::OperationFailure<'_>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;
        let aby3 = aby2 + INDENT;

        writeln!(f, "OperationFailure: {{")?;
        writeln!(f, "{:aby$}err_code: {}", "", self.err_code, aby = aby2)?;
        writeln!(f, "{:aby$}err_msg: {}", "", self.err_msg, aby = aby2)?;
        writeln!(f, "{:aby$}updated_inst_results: [", "", aby = aby2)?;
        for r in self.updated_inst_failures.iter() {
            write!(f, "{:#aby$?}", r, aby = aby3)?;
        }
        writeln!(f, "{:aby$}]", "", aby = aby2)?;
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl fmt::Display for OperateResp<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}OperateResp: {{", "", aby = aby)?;
        for res in self.operation_results.iter() {
            write!(f, "{:aby$}", res, aby = aby2)?;
        }
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl fmt::Display for mod_OperateResp::OperationResult<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        use mod_OperateResp::mod_OperationResult::OneOfoperation_resp::*;

        writeln!(f, "{:aby$}OperationResult: {{", "", aby = aby)?;
        writeln!(
            f,
            "{:aby$}executed_command: {}",
            "",
            self.executed_command,
            aby = aby2
        )?;
        write!(f, "{:aby$}operation_resp: ", "", aby = aby2)?;
        match &self.operation_resp {
            req_obj_path(ref m) => writeln!(f, "{}", m),
            req_output_args(ref m) => write!(f, "{:aby$}", m, aby = aby2),
            cmd_failure(ref m) => write!(f, "{:aby$}", m, aby = aby2),
            None => Ok(()),
        }?;
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl fmt::Display for mod_OperateResp::mod_OperationResult::OutputArgs<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "OutputArgs: {{")?;
        for (k, v) in self.output_args.iter() {
            writeln!(f, "{:aby$}{} : {}", "", k, v, aby = aby2)?;
        }
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl fmt::Display for mod_OperateResp::mod_OperationResult::CommandFailure<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}CommandFailure: {{", "", aby = aby)?;
        writeln!(f, "{:aby$}err_code: {}", "", self.err_code, aby = aby2)?;
        writeln!(f, "{:aby$}err_msg: {}", "", self.err_msg, aby = aby2)?;
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl fmt::Display for NotifyResp<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}NotifyResp: {{", "", aby = aby)?;
        writeln!(
            f,
            "{:aby$}subscription_id: {}",
            "",
            self.subscription_id,
            aby = aby2
        )?;
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl fmt::Display for GetSupportedProtocolResp<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        // TODO: Implement
        writeln!(
            f,
            "{:aby$}{:#?}",
            "",
            self.agent_supported_protocol_versions,
            aby = aby2
        )
    }
}

impl fmt::Display for mod_DeleteResp::DeletedObjectResult<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}DeletedObjectResult: {{", "", aby = aby)?;
        writeln!(
            f,
            "{:aby$}requested_path: {}",
            "",
            self.requested_path,
            aby = aby2
        )?;
        write!(
            f,
            "{:aby$}oper_status: {:aby$}",
            "",
            self.oper_status.clone().unwrap(),
            aby = aby2
        )?;
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl fmt::Display for mod_DeleteResp::mod_DeletedObjectResult::OperationStatus<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        use mod_DeleteResp::mod_DeletedObjectResult::mod_OperationStatus::OneOfoper_status::*;

        match &self.oper_status {
            status => match status {
                oper_success(ref m) => write!(f, "{:aby$}", m, aby = aby),
                oper_failure(ref m) => write!(f, "{:aby$}", m, aby = aby),
                None => writeln!(f, "{:aby$}None", "", aby = aby2),
            },
        }
    }
}

impl fmt::Display
    for mod_DeleteResp::mod_DeletedObjectResult::mod_OperationStatus::OperationSuccess<'_>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;
        let aby3 = aby2 + INDENT;

        writeln!(f, "OperationSuccess: {{")?;
        writeln!(
            f,
            "{:aby$}affected_path: [ {} ]",
            "",
            self.affected_paths.join(", "),
            aby = aby2
        )?;
        writeln!(f, "{:aby$}unaffected_path_errs: [", "", aby = aby2)?;
        for err in self.unaffected_path_errs.iter() {
            write!(f, "{:aby$}", err, aby = aby3)?;
        }
        writeln!(f, "{:aby$}]", "", aby = aby2)?;
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl fmt::Display
    for mod_DeleteResp::mod_DeletedObjectResult::mod_OperationStatus::OperationFailure<'_>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f)?;
        writeln!(f, "{:aby$}OperationFailure: {{", "", aby = aby)?;
        writeln!(f, "{:aby$}err_code: {}", "", self.err_code, aby = aby2)?;
        writeln!(f, "{:aby$}err_msg: {}", "", self.err_msg, aby = aby2)?;
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl fmt::Display
    for mod_DeleteResp::mod_DeletedObjectResult::mod_OperationStatus::UnaffectedPathError<'_>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}UnaffectedPathError: {{", "", aby = aby)?;
        writeln!(f, "{:aby$}err_code: {}", "", self.err_code, aby = aby2)?;
        writeln!(f, "{:aby$}err_msg: {}", "", self.err_msg, aby = aby2)?;
        writeln!(
            f,
            "{:aby$}unaffected_path: {}",
            "",
            self.unaffected_path,
            aby = aby2
        )?;
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl fmt::Display for mod_GetResp::RequestedPathResult<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}RequestedPathResult: {{", "", aby = aby)?;
        writeln!(
            f,
            "{:aby$}requested_path: {}",
            "",
            self.requested_path,
            aby = aby2
        )?;
        writeln!(f, "{:aby$}err_code: {}", "", self.err_code, aby = aby2)?;
        writeln!(f, "{:aby$}err_msg: {}", "", self.err_msg, aby = aby2)?;
        for result in self.resolved_path_results.iter() {
            write!(f, "{:aby$}", result, aby = aby2)?;
        }
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl fmt::Display for mod_GetResp::ResolvedPathResult<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;
        let aby3 = aby2 + INDENT;

        writeln!(f, "{:aby$}ResolvedPathResult: {{", "", aby = aby)?;
        writeln!(
            f,
            "{:aby$}resolved_path: {}",
            "",
            self.resolved_path,
            aby = aby2
        )?;
        writeln!(f, "{:aby$}result_params: {{", "", aby = aby2)?;
        for (k, v) in self.result_params.iter() {
            writeln!(f, "{:aby$}{}: {}", "", k, v, aby = aby3)?;
        }
        writeln!(f, "{:aby$}}}", "", aby = aby2)?;
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl fmt::Display for AddResp<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}AddResp: [", "", aby = aby)?;
        for result in self.created_obj_results.iter() {
            write!(f, "{:aby$}", result, aby = aby2)?;
        }
        writeln!(f, "{:aby$}]", "", aby = aby)
    }
}

impl fmt::Display for mod_AddResp::CreatedObjectResult<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}CreatedObjectResult: {{", "", aby = aby)?;
        writeln!(
            f,
            "{:aby$}requested_path: {}",
            "",
            self.requested_path,
            aby = aby2
        )?;
        write!(
            f,
            "{:aby$}oper_status: {:aby$}",
            "",
            self.oper_status.clone().unwrap(),
            aby = aby2
        )?;
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl fmt::Display for mod_AddResp::mod_CreatedObjectResult::OperationStatus<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        use mod_AddResp::mod_CreatedObjectResult::mod_OperationStatus::OneOfoper_status::*;

        match &self.oper_status {
            status => match status {
                oper_success(ref m) => write!(f, "{:aby$}", m, aby = aby),
                oper_failure(ref m) => write!(f, "{:aby$}", m, aby = aby),
                None => writeln!(f, "{:aby$}None", "", aby = aby2),
            },
        }
    }
}

impl fmt::Display
    for mod_AddResp::mod_CreatedObjectResult::mod_OperationStatus::OperationFailure<'_>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}OperationFailure: {{", "", aby = aby)?;
        writeln!(f, "{:aby$}err_code: {}", "", self.err_code, aby = aby2)?;
        writeln!(f, "{:aby$}err_msg: {}", "", self.err_msg, aby = aby2)?;
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl fmt::Display
    for mod_AddResp::mod_CreatedObjectResult::mod_OperationStatus::OperationSuccess<'_>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;
        let aby3 = aby2 + INDENT;

        writeln!(f)?;
        writeln!(f, "{:aby$}OperationSuccess: {{", "", aby = aby)?;
        writeln!(
            f,
            "{:aby$}instantiated_path: {}",
            "",
            self.instantiated_path,
            aby = aby2
        )?;
        writeln!(f, "{:aby$}param_errs: [", "", aby = aby2)?;
        for result in self.param_errs.iter() {
            write!(f, "{:aby$}", result, aby = aby3)?;
        }
        writeln!(f, "{:aby$}]", "", aby = aby2)?;
        writeln!(f, "{:aby$}unique_keys: {{", "", aby = aby2)?;
        for (k, v) in self.unique_keys.iter() {
            writeln!(f, "{:aby$}{}: {}", "", k, v, aby = aby3)?;
        }
        writeln!(f, "{:aby$}}}", "", aby = aby2)?;
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl fmt::Display
    for mod_AddResp::mod_CreatedObjectResult::mod_OperationStatus::ParameterError<'_>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}ParameterError: {{", "", aby = aby)?;
        writeln!(f, "{:aby$}param: {}", "", self.param, aby = aby2)?;
        writeln!(f, "{:aby$}err_code: {}", "", self.err_code, aby = aby2)?;
        writeln!(f, "{:aby$}err_msg: {}", "", self.err_msg, aby = aby2)?;
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}
