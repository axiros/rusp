use crate::usp;
use crate::usp_record;

const INDENT: usize = 2;

impl std::fmt::Display for usp_record::Record<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use crate::usp_decoder::*;
        use crate::usp_record::mod_Record::OneOfrecord_type::no_session_context;

        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}Record {{", "", aby = aby)?;
        writeln!(
            f,
            "{:aby$}version: {}",
            "",
            self.version.clone().unwrap(),
            aby = aby2
        )?;
        writeln!(
            f,
            "{:aby$}to_id: {}",
            "",
            self.to_id.clone().unwrap_or_else(|| "".into()),
            aby = aby2
        )?;
        writeln!(
            f,
            "{:aby$}from_id: {}",
            "",
            self.from_id.clone().unwrap_or_else(|| "".into()),
            aby = aby2
        )?;
        match self.payload_security {
            Some(sec) => write!(f, "{:aby$}payload_security: {}", "", sec, aby = aby2),
            None => writeln!(f, "{:aby$}payload_security: ", "", aby = aby2),
        }?;
        match &self.mac_signature {
            Some(sig) => write!(f, "{:aby$}mac_signature: {:#?}", "", sig, aby = aby2),
            None => writeln!(f, "{:aby$}mac_signature: ", "", aby = aby2),
        }?;
        match &self.sender_cert {
            Some(cert) => write!(f, "{:aby$}sender_cert: {:#?}", "", cert, aby = aby2),
            None => writeln!(f, "{:aby$}sender_cert: ", "", aby = aby2),
        }?;
        if let no_session_context(context) = &self.record_type {
            if let Some(payload) = &context.payload {
                write!(f, "{:aby$}", decode_msg(&payload.clone()), aby = aby2)?;
            } else {
                writeln!(f, "{:aby$}no payload!", "", aby = aby2)?;
            }
        } else {
            writeln!(f, "{:aby$}can't handle session_context!", "", aby = aby2)?;
        };

        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl std::fmt::Display for usp_record::mod_Record::PayloadSecurity {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        // TODO: Implement
        writeln!(f, "{:aby$}{:?}", "", self, aby = aby2)
    }
}

impl std::fmt::Display for usp::Msg<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;
        writeln!(f, "{:aby$}Msg {{", "", aby = aby)?;
        write!(f, "{:aby$}", self.header.clone().unwrap(), aby = aby2)?;
        write!(f, "{:aby$}", self.body.clone().unwrap(), aby = aby2)?;
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl std::fmt::Display for usp::Header<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}Header: {{", "", aby = aby)?;
        writeln!(
            f,
            "{:aby$}msg_id: {}",
            "",
            self.msg_id.clone().expect("Message must contain a message id"),
            aby = aby2
        )?;
        if self.msg_type.is_some() {
            write!(
                f,
                "{:aby$}msg_type: {}",
                "",
                self.msg_type.unwrap(),
                aby = aby2
            )?;
        }
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl std::fmt::Display for usp::mod_Header::MsgType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use crate::usp::mod_Header::MsgType::*;

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

impl std::fmt::Display for usp::Body<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use crate::usp::mod_Body::OneOfmsg_body::*;

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

impl std::fmt::Display for usp::Request<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use crate::usp::mod_Request::OneOfreq_type::*;

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

impl std::fmt::Display for usp::Response<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use crate::usp::mod_Response::OneOfresp_type::*;

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

impl std::fmt::Display for usp::Error<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;
        let aby3 = aby2 + INDENT;

        writeln!(f, "{:aby$}Error: {{", "", aby = aby)?;
        writeln!(
            f,
            "{:aby$}err_msg: {}",
            "",
            self.err_msg.clone().unwrap_or_else(|| "".into()),
            aby = aby2
        )?;
        writeln!(
            f,
            "{:aby$}err_code: {}",
            "",
            self.err_code.unwrap_or(0),
            aby = aby2
        )?;
        writeln!(f, "{:aby$}param_errs: [", "", aby = aby2)?;
        for result in self.param_errs.iter() {
            write!(f, "{:aby$}", result, aby = aby3)?;
        }
        writeln!(f, "{:aby$}]", "", aby = aby2)?;
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl std::fmt::Display for usp::mod_Error::ParamError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        // TODO: Implement
        writeln!(f, "{:aby$}{:?}", "", self.param_path, aby = aby2)
    }
}

impl std::fmt::Display for usp::DeleteResp<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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

impl std::fmt::Display for usp::Get<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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

impl std::fmt::Display for usp::GetSupportedDM<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}GetSupportedDM: {{", "", aby = aby)?;
        writeln!(
            f,
            "{:aby$}first_level_only: {:aby$}",
            "",
            self.first_level_only.unwrap_or(false),
            aby = aby2
        )?;
        writeln!(
            f,
            "{:aby$}return_commands: {:aby$}",
            "",
            self.return_commands.unwrap_or(false),
            aby = aby2
        )?;
        writeln!(
            f,
            "{:aby$}return_events: {:aby$}",
            "",
            self.return_events.unwrap_or(false),
            aby = aby2
        )?;
        writeln!(
            f,
            "{:aby$}return_params: {:aby$}",
            "",
            self.return_params.unwrap_or(false),
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

impl std::fmt::Display for usp::GetSupportedProtocol<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}GetSupportedProtocol: {{", "", aby = aby)?;
        writeln!(
            f,
            "{:aby$}controller_supported_protocol_versions: {:aby$}",
            "",
            self.controller_supported_protocol_versions.clone().unwrap(),
            aby = aby2
        )?;
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl std::fmt::Display for usp::Operate<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;
        let aby3 = aby2 + INDENT;

        writeln!(f, "{:aby$}Operate: {{", "", aby = aby)?;
        writeln!(
            f,
            "{:aby$}command: {:aby$}",
            "",
            self.command.clone().unwrap(),
            aby = aby2
        )?;
        writeln!(
            f,
            "{:aby$}command_key: {:aby$}",
            "",
            self.command_key.clone().unwrap(),
            aby = aby2
        )?;
        writeln!(
            f,
            "{:aby$}send_resp: {:aby$}",
            "",
            self.send_resp.unwrap_or(false),
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

impl std::fmt::Display for usp::Notify<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use crate::usp::mod_Notify::OneOfnotification::*;
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}Notify: {{", "", aby = aby)?;
        writeln!(
            f,
            "{:aby$}subscription_id: {}",
            "",
            self.subscription_id.clone().unwrap_or("".into()),
            aby = aby2
        )?;
        writeln!(
            f,
            "{:aby$}send_resp: {}",
            "",
            self.send_resp.unwrap_or(false),
            aby = aby2
        )?;
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

impl std::fmt::Display for usp::mod_Notify::Event<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        // TODO: Implement
        writeln!(f, "{:aby$}{:?}", "", self.event_name, aby = aby2)
    }
}

impl std::fmt::Display for usp::mod_Notify::ValueChange<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        // TODO: Implement
        writeln!(f, "{:aby$}{:?}", "", self.param_path, aby = aby2)
    }
}

impl std::fmt::Display for usp::mod_Notify::ObjectCreation<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        // TODO: Implement
        writeln!(f, "{:aby$}{:?}", "", self.obj_path, aby = aby2)
    }
}

impl std::fmt::Display for usp::mod_Notify::ObjectDeletion<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        // TODO: Implement
        writeln!(f, "{:aby$}{:?}", "", self.obj_path, aby = aby2)
    }
}

impl std::fmt::Display for usp::mod_Notify::OperationComplete<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        // TODO: Implement
        writeln!(f, "{:aby$}{:?}", "", self.operation_resp, aby = aby2)
    }
}

impl std::fmt::Display for usp::mod_Notify::OnBoardRequest<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}OnBoardRequest: {{", "", aby = aby)?;
        writeln!(
            f,
            "{:aby$}oui: {}",
            "",
            self.oui.clone().unwrap_or_else(|| "".into()),
            aby = aby2
        )?;
        writeln!(
            f,
            "{:aby$}product_class: {}",
            "",
            self.product_class.clone().unwrap_or_else(|| "".into()),
            aby = aby2
        )?;
        writeln!(
            f,
            "{:aby$}serial_number: {}",
            "",
            self.serial_number.clone().unwrap_or_else(|| "".into()),
            aby = aby2
        )?;
        writeln!(
            f,
            "{:aby$}agent_supported_protocol_versions: {}",
            "",
            self.agent_supported_protocol_versions
                .clone()
                .unwrap_or_else(|| "".into()),
            aby = aby2
        )?;
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl std::fmt::Display for usp::Set<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}Add: {{", "", aby = aby)?;
        writeln!(
            f,
            "{:aby$}allow_partial: {}",
            "",
            self.allow_partial.unwrap(),
            aby = aby2
        )?;
        for result in self.update_objs.iter() {
            write!(f, "{:aby$}", result, aby = aby2)?;
        }
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl std::fmt::Display for usp::mod_Set::UpdateObject<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}UpdateObject: {{", "", aby = aby)?;
        writeln!(
            f,
            "{:aby$}obj_path: {}",
            "",
            self.obj_path.clone().unwrap_or_else(|| "".into()),
            aby = aby2
        )?;
        for ps in self.param_settings.iter() {
            write!(f, "{:aby$}", ps, aby = aby2)?;
        }
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl std::fmt::Display for usp::mod_Set::UpdateParamSetting<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}UpdateParamSetting: {{", "", aby = aby)?;
        writeln!(
            f,
            "{:aby$}param: {}",
            "",
            self.param.clone().unwrap_or_else(|| "".into()),
            aby = aby2
        )?;
        writeln!(
            f,
            "{:aby$}value: {}",
            "",
            self.value.clone().unwrap_or_else(|| "".into()),
            aby = aby2
        )?;
        writeln!(
            f,
            "{:aby$}required: {}",
            "",
            self.required.unwrap(),
            aby = aby2
        )?;
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl std::fmt::Display for usp::Add<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}Add: {{", "", aby = aby)?;
        writeln!(
            f,
            "{:aby$}allow_partial: {}",
            "",
            self.allow_partial.unwrap(),
            aby = aby2
        )?;
        for result in self.create_objs.iter() {
            write!(f, "{:aby$}", result, aby = aby2)?;
        }
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl std::fmt::Display for usp::mod_Add::CreateObject<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}CreateObject: {{", "", aby = aby)?;
        writeln!(
            f,
            "{:aby$}obj_path: {}",
            "",
            self.obj_path.clone().unwrap_or_else(|| "".into()),
            aby = aby2
        )?;
        for ps in self.param_settings.iter() {
            write!(f, "{:aby$}", ps, aby = aby2)?;
        }
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl std::fmt::Display for usp::mod_Add::CreateParamSetting<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}CreateParamSetting: {{", "", aby = aby)?;
        writeln!(
            f,
            "{:aby$}param: {}",
            "",
            self.param.clone().unwrap_or_else(|| "".into()),
            aby = aby2
        )?;
        writeln!(
            f,
            "{:aby$}value: {}",
            "",
            self.value.clone().unwrap_or_else(|| "".into()),
            aby = aby2
        )?;
        writeln!(
            f,
            "{:aby$}required: {}",
            "",
            self.required.unwrap(),
            aby = aby2
        )?;
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl std::fmt::Display for usp::Delete<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}Delete: {{", "", aby = aby)?;
        writeln!(
            f,
            "{:aby$}allow_partial: {}",
            "",
            self.allow_partial.unwrap(),
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

impl std::fmt::Display for usp::GetInstances<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}GetInstances: {{", "", aby = aby)?;
        writeln!(
            f,
            "{:aby$}first_level_only: {}",
            "",
            self.first_level_only.unwrap_or(false),
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

impl std::fmt::Display for usp::GetResp<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}GetResp: [", "", aby = aby)?;
        for result in self.req_path_results.iter() {
            write!(f, "{:aby$}", result, aby = aby2)?;
        }

        writeln!(f, "{:aby$}]", "", aby = aby)
    }
}

impl std::fmt::Display for usp::GetSupportedDMResp<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}GetSupportedDMResp: [", "", aby = aby)?;
        for result in self.req_obj_results.iter() {
            write!(f, "{:aby$}", result, aby = aby2)?;
        }

        writeln!(f, "{:aby$}]", "", aby = aby)
    }
}

impl std::fmt::Display for usp::mod_GetSupportedDMResp::RequestedObjectResult<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}RequestedObjectResult: {{", "", aby = aby)?;
        writeln!(
            f,
            "{:aby$}req_obj_path: {}",
            "",
            self.req_obj_path.clone().unwrap_or_else(|| "".into()),
            aby = aby2
        )?;
        writeln!(
            f,
            "{:aby$}err_code: {}",
            "",
            self.err_code.unwrap_or(0),
            aby = aby2
        )?;
        writeln!(
            f,
            "{:aby$}err_msg: {}",
            "",
            self.err_msg.clone().unwrap_or_else(|| "".into()),
            aby = aby2
        )?;
        writeln!(
            f,
            "{:aby$}data_model_inst_uri: {}",
            "",
            self.data_model_inst_uri
                .clone()
                .unwrap_or_else(|| "".into()),
            aby = aby2
        )?;
        for result in self.supported_objs.iter() {
            write!(f, "{:aby$}", result, aby = aby2)?;
        }
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl std::fmt::Display for usp::mod_GetSupportedDMResp::SupportedObjectResult<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}RequestedObjectResult: {{", "", aby = aby)?;
        writeln!(
            f,
            "{:aby$}supported_obj_path: {}",
            "",
            self.supported_obj_path.clone().unwrap_or_else(|| "".into()),
            aby = aby2
        )?;
        writeln!(f, "{:aby$}access: {:?}", "", self.access, aby = aby2)?;
        writeln!(
            f,
            "{:aby$}is_multi_instance: {}",
            "",
            self.is_multi_instance.unwrap_or_else(|| false),
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

impl std::fmt::Display for usp::mod_GetSupportedDMResp::SupportedCommandResult<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        // TODO: Implement
        writeln!(f, "{:aby$}{:?}", "", self, aby = aby2)
    }
}

impl std::fmt::Display for usp::mod_GetSupportedDMResp::SupportedEventResult<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        // TODO: Implement
        writeln!(f, "{:aby$}{:?}", "", self, aby = aby2)
    }
}

impl std::fmt::Display for usp::mod_GetSupportedDMResp::SupportedParamResult<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        // TODO: Implement
        writeln!(f, "{:aby$}{:?}", "", self, aby = aby2)
    }
}

impl std::fmt::Display for usp::GetInstancesResp<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}GetInstancesResp: [", "", aby = aby)?;
        for result in self.req_path_results.iter() {
            write!(f, "{:aby$}", result, aby = aby2)?;
        }

        writeln!(f, "{:aby$}]", "", aby = aby)
    }
}

impl std::fmt::Display for usp::mod_GetInstancesResp::RequestedPathResult<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}RequestedPathResult: {{", "", aby = aby)?;
        writeln!(
            f,
            "{:aby$}requested_path: {}",
            "",
            self.requested_path.clone().unwrap_or_else(|| "".into()),
            aby = aby2
        )?;
        writeln!(
            f,
            "{:aby$}err_code: {}",
            "",
            self.err_code.unwrap_or(0),
            aby = aby2
        )?;
        writeln!(
            f,
            "{:aby$}err_msg: {}",
            "",
            self.err_msg.clone().unwrap_or_else(|| "".into()),
            aby = aby2
        )?;
        for result in self.curr_insts.iter() {
            write!(f, "{:aby$}", result, aby = aby2)?;
        }
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl std::fmt::Display for usp::mod_GetInstancesResp::CurrInstance<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;
        let aby3 = aby2 + INDENT;

        writeln!(f, "{:aby$}{{", "", aby = aby)?;
        writeln!(
            f,
            "{:aby$}instantiated_obj_path: {}",
            "",
            self.instantiated_obj_path
                .clone()
                .unwrap_or_else(|| "".into()),
            aby = aby2
        )?;
        writeln!(f, "{:aby$}unique_keys: {{", "", aby = aby2)?;
        for (k, v) in self.unique_keys.iter() {
            writeln!(f, "{:aby$}{}: {}", "", k, v, aby = aby3)?;
        }
        writeln!(f, "{:aby$}}}", "", aby = aby2)?;
        writeln!(f, "{:aby$}}},", "", aby = aby)
    }
}

impl std::fmt::Display for usp::SetResp<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        // TODO: Implement
        writeln!(f, "{:aby$}{:?}", "", self.updated_obj_results, aby = aby2)
    }
}

impl std::fmt::Display for usp::OperateResp<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}OperateResp: {{", "", aby = aby)?;
        for res in self.operation_results.iter() {
            write!(f, "{:aby$}", res, aby = aby2)?;
        }
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl std::fmt::Display for usp::mod_OperateResp::OperationResult<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        use crate::usp::mod_OperateResp::mod_OperationResult::OneOfoperation_resp::*;

        writeln!(f, "{:aby$}OperationResult: {{", "", aby = aby)?;
        writeln!(
            f,
            "{:aby$}executed_command: {}",
            "",
            self.executed_command.clone().unwrap_or_else(|| "".into()),
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

impl std::fmt::Display for usp::mod_OperateResp::mod_OperationResult::OutputArgs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "OutputArgs: {{")?;
        for (k, v) in self.output_args.iter() {
            writeln!(f, "{:aby$}{} : {}", "", k, v, aby = aby2)?;
        }
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl std::fmt::Display for usp::mod_OperateResp::mod_OperationResult::CommandFailure<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}CommandFailure: {{", "", aby = aby)?;
        writeln!(
            f,
            "{:aby$}err_msg: {}",
            "",
            self.err_msg.clone().unwrap_or_else(|| "".into()),
            aby = aby2
        )?;
        writeln!(
            f,
            "{:aby$}err_code: {}",
            "",
            self.err_code.unwrap_or(0),
            aby = aby2
        )?;
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl std::fmt::Display for usp::NotifyResp<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        // TODO: Implement
        writeln!(f, "{:aby$}{:?}", "", self.subscription_id, aby = aby2)
    }
}

impl std::fmt::Display for usp::GetSupportedProtocolResp<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        // TODO: Implement
        writeln!(
            f,
            "{:aby$}{:?}",
            "",
            self.agent_supported_protocol_versions,
            aby = aby2
        )
    }
}

impl std::fmt::Display for usp::mod_DeleteResp::DeletedObjectResult<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}DeletedObjectResult: {{", "", aby = aby)?;
        writeln!(
            f,
            "{:aby$}requested_path: {}",
            "",
            self.requested_path.clone().unwrap_or_else(|| "".into()),
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

impl std::fmt::Display for usp::mod_DeleteResp::mod_DeletedObjectResult::OperationStatus<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        use
            crate::usp::mod_DeleteResp::mod_DeletedObjectResult::mod_OperationStatus::OneOfoper_status::*;

        match &self.oper_status {
            status => match status {
                oper_success(ref m) => write!(f, "{:aby$}", m, aby = aby),
                oper_failure(ref m) => write!(f, "{:aby$}", m, aby = aby),
                None => writeln!(f, "{:aby$}None", "", aby = aby2),
            },
        }
    }
}

impl std::fmt::Display
    for usp::mod_DeleteResp::mod_DeletedObjectResult::mod_OperationStatus::OperationSuccess<'_>
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;
        let aby3 = aby2 + INDENT;

        writeln!(f, "OperationSuccess: {{")?;
        writeln!(
            f,
            "{:aby$}instantiated_path: [ {} ]",
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

impl std::fmt::Display
    for usp::mod_DeleteResp::mod_DeletedObjectResult::mod_OperationStatus::OperationFailure<'_>
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f)?;
        writeln!(f, "{:aby$}OperationFailure: {{", "", aby = aby)?;
        writeln!(
            f,
            "{:aby$}err_msg: {}",
            "",
            self.err_msg.clone().unwrap_or_else(|| "".into()),
            aby = aby2
        )?;
        writeln!(
            f,
            "{:aby$}err_code: {}",
            "",
            self.err_code.unwrap_or(0),
            aby = aby2
        )?;
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl std::fmt::Display
    for usp::mod_DeleteResp::mod_DeletedObjectResult::mod_OperationStatus::UnaffectedPathError<'_>
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}UnaffectedPathError: {{", "", aby = aby)?;
        writeln!(
            f,
            "{:aby$}unaffected_path: {}",
            "",
            self.unaffected_path.clone().unwrap_or_else(|| "".into()),
            aby = aby2
        )?;
        writeln!(
            f,
            "{:aby$}err_code: {}",
            "",
            self.err_code.unwrap_or(0),
            aby = aby2
        )?;
        writeln!(
            f,
            "{:aby$}err_msg: {}",
            "",
            self.err_msg.clone().unwrap_or_else(|| "".into()),
            aby = aby2
        )?;
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl std::fmt::Display for usp::mod_GetResp::RequestedPathResult<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}RequestedPathResult: {{", "", aby = aby)?;
        writeln!(
            f,
            "{:aby$}requested_path: {}",
            "",
            self.requested_path.clone().unwrap_or_else(|| "".into()),
            aby = aby2
        )?;
        writeln!(
            f,
            "{:aby$}err_code: {}",
            "",
            self.err_code.unwrap_or(0),
            aby = aby2
        )?;
        writeln!(
            f,
            "{:aby$}err_msg: {}",
            "",
            self.err_msg.clone().unwrap_or_else(|| "".into()),
            aby = aby2
        )?;
        for result in self.resolved_path_results.iter() {
            write!(f, "{:aby$}", result, aby = aby2)?;
        }
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl std::fmt::Display for usp::mod_GetResp::ResolvedPathResult<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;
        let aby3 = aby2 + INDENT;

        writeln!(f, "{:aby$}ResolvedPathResult: {{", "", aby = aby)?;
        writeln!(
            f,
            "{:aby$}resolved_path: {}",
            "",
            self.resolved_path.clone().unwrap_or_else(|| "".into()),
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

impl std::fmt::Display for usp::AddResp<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}AddResp: [", "", aby = aby)?;
        for result in self.created_obj_results.iter() {
            write!(f, "{:aby$}", result, aby = aby2)?;
        }
        writeln!(f, "{:aby$}]", "", aby = aby)
    }
}

impl std::fmt::Display for usp::mod_AddResp::CreatedObjectResult<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f, "{:aby$}CreatedObjectResult: {{", "", aby = aby)?;
        writeln!(
            f,
            "{:aby$}requested_path: {}",
            "",
            self.requested_path.clone().unwrap_or_else(|| "".into()),
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

impl std::fmt::Display for usp::mod_AddResp::mod_CreatedObjectResult::OperationStatus<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        use
            crate::usp::mod_AddResp::mod_CreatedObjectResult::mod_OperationStatus::OneOfoper_status::*;

        match &self.oper_status {
            status => match status {
                oper_success(ref m) => write!(f, "{:aby$}", m, aby = aby),
                oper_failure(ref m) => write!(f, "{:aby$}", m, aby = aby),
                None => writeln!(f, "{:aby$}None", "", aby = aby2),
            },
        }
    }
}

impl std::fmt::Display
    for usp::mod_AddResp::mod_CreatedObjectResult::mod_OperationStatus::OperationFailure<'_>
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        writeln!(f)?;
        writeln!(f, "{:aby$}OperationFailure: {{", "", aby = aby)?;
        writeln!(
            f,
            "{:aby$}err_msg: {}",
            "",
            self.err_msg.clone().unwrap_or_else(|| "".into()),
            aby = aby2
        )?;
        writeln!(
            f,
            "{:aby$}err_code: {}",
            "",
            self.err_code.unwrap_or(0),
            aby = aby2
        )?;
        writeln!(f, "{:aby$}}}", "", aby = aby)
    }
}

impl std::fmt::Display
    for usp::mod_AddResp::mod_CreatedObjectResult::mod_OperationStatus::OperationSuccess<'_>
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;
        let aby3 = aby2 + INDENT;

        writeln!(f)?;
        writeln!(f, "{:aby$}OperationSuccess: {{", "", aby = aby)?;
        writeln!(
            f,
            "{:aby$}instantiated_path: {}",
            "",
            self.instantiated_path.clone().unwrap(),
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

impl std::fmt::Display
    for usp::mod_AddResp::mod_CreatedObjectResult::mod_OperationStatus::ParameterError<'_>
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let aby = f.width().unwrap_or(0);
        let aby2 = aby + INDENT;

        // TODO: Implement
        writeln!(f, "{:aby$?}{}", "", self, aby = aby2)
    }
}
