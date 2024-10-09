#![allow(clippy::new_without_default)]

mod add;
mod delete;
mod deregister;
mod error;
mod get;
mod getinstances;
mod getsupporteddm;
mod getsupportedprotocol;
mod msg;
mod notify;
mod operate;
mod record;
mod register;
mod set;

pub use add::{
    AddBuilder, AddOperationStatus, AddRespBuilder, AddRespParameterError, CreateObjectBuilder,
    CreatedObjectResultsBuilder,
};
pub use delete::{
    DeleteBuilder, DeleteRespBuilder, DeleteRespOperationStatus, DeleteRespUnaffectedPathError,
    DeletedObjectResultsBuilder,
};
pub use deregister::{
    DeregisterBuilder, DeregisterOperationStatus, DeregisterRespBuilder,
    DeregisteredPathResultBuilder,
};
pub use error::ErrorBuilder;
pub use get::{
    GetBuilder, GetRespBuilder, ReqPathResultBuilder as GetReqPathResultBuilder,
    ResolvedPathResultBuilder,
};
pub use getinstances::{
    CurrInstanceBuilder, GetInstancesBuilder, GetInstancesRespBuilder,
    ReqPathResultBuilder as GetInstancesRespReqPathResultBuilder,
};
pub use getsupporteddm::{
    GSDMCommandResult, GSDMEventResult, GSDMParamResult, GSDMReqObjectResultBuilder,
    GSDMSupportedObjectResultBuilder, GetSupportedDMBuilder, GetSupportedDMRespBuilder,
};
pub use getsupportedprotocol::{GetSupportedProtocolBuilder, GetSupportedProtocolRespBuilder};
pub use msg::MsgBuilder;
pub use notify::{NotifyBuilder, NotifyRespBuilder};
pub use operate::{OperateBuilder, OperateRespBuilder, OperateRespResultBuilder};
pub use record::{RecordBuilder, SessionContextBuilder};
pub use register::{
    RegisterBuilder, RegisterOperationStatus, RegisterRespBuilder, RegisteredPathResultBuilder,
};
pub use set::{
    SetBuilder, SetOperationStatus, SetOperationSuccessBuilder, SetRespBuilder,
    SetRespParameterError, UpdateObjectBuilder, UpdatedInstanceFailureBuilder,
    UpdatedObjectResultsBuilder,
};

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashMap;

    use crate::usp::mod_Body::OneOfmsg_body;
    use crate::usp::mod_Response::OneOfresp_type;
    use crate::usp::Response;
    use crate::usp_decoder::try_decode_msg;
    use crate::usp_encoder::try_encode_msg;

    #[test]
    fn bytes_conversion() {
        let bytes = b"\
            \n\x07\n\x03add\x10\x08\x12'\n%*#\x08\x01\x12\x1f\n\x0bDevice.Foo.\x12\x10\n\x05\
            Param\x12\x05Value\x18\x01\
        ";

        let msg = try_decode_msg(bytes).unwrap();
        let bytes2 = try_encode_msg(&msg).unwrap();
        assert_eq!(bytes, &*bytes2);
        assert_eq!(msg.msg_id(), "add");

        let add_obj = CreateObjectBuilder::new("Device.Foo.".into()).with_param_settings(vec![(
            "Param".into(),
            "Value".into(),
            true,
        )]);
        let expected_body = AddBuilder::new()
            .with_allow_partial(true)
            .with_create_objs(vec![add_obj])
            .build()
            .unwrap();
        assert_eq!(msg.body.unwrap(), expected_body);
    }

    #[test]
    fn new_request_without_context() {
        let add_obj = CreateObjectBuilder::new("Device.Foo.".into()).with_param_settings(vec![(
            "Param".into(),
            "Value".into(),
            true,
        )]);
        let body = AddBuilder::new()
            .with_allow_partial(true)
            .with_create_objs(vec![add_obj])
            .build()
            .unwrap();

        let msg = MsgBuilder::new()
            .with_msg_id("add".into())
            .with_body(body)
            .build()
            .unwrap();
        let bytes = try_encode_msg(&msg).expect("serialisation failed");

        let rmsg = try_decode_msg(&bytes).expect("deserialisation failed");
        assert_eq!(rmsg.msg_id(), "add");
        assert_eq!(rmsg, msg);

        let bytes2 = try_encode_msg(&rmsg).expect("2nd serialisation failed");
        assert_eq!(bytes, bytes2);
    }

    #[test]
    fn roundtrip_delete_request() {
        let body = DeleteBuilder::new()
            .with_allow_partial(true)
            .with_obj_paths(vec!["Device.Foo.2.".into(), "Device.Bar.1.".into()])
            .build()
            .unwrap();

        let msg = MsgBuilder::new()
            .with_msg_id("delete".into())
            .with_body(body)
            .build()
            .unwrap();
        let bytes = try_encode_msg(&msg).expect("serialisation failed");

        let rmsg = try_decode_msg(&bytes).expect("deserialisation failed");
        assert_eq!(rmsg.msg_id(), "delete");
        assert_eq!(rmsg, msg);

        let bytes2 = try_encode_msg(&rmsg).expect("2nd serialisation failed");
        assert_eq!(bytes, bytes2);
    }

    #[test]
    fn roundtrip_get_request() {
        let body = GetBuilder::new()
            .with_max_depth(1)
            .with_params(vec!["Device.Foo.2.".into(), "Device.Bar.1.".into()])
            .build()
            .unwrap();

        let msg = MsgBuilder::new()
            .with_msg_id("get".into())
            .with_body(body)
            .build()
            .unwrap();
        let bytes = try_encode_msg(&msg).expect("serialisation failed");

        let rmsg = try_decode_msg(&bytes).expect("deserialisation failed");
        assert_eq!(rmsg.msg_id(), "get");
        assert_eq!(rmsg, msg);

        let bytes2 = try_encode_msg(&rmsg).expect("2nd serialisation failed");
        assert_eq!(bytes, bytes2);
    }

    #[test]
    fn roundtrip_get_instances_request() {
        let body = GetInstancesBuilder::new()
            .with_first_level_only(true)
            .with_obj_paths(vec!["Device.Foo.".into(), "Device.Bar.".into()])
            .build()
            .unwrap();

        let msg = MsgBuilder::new()
            .with_msg_id("get_instances".into())
            .with_body(body)
            .build()
            .unwrap();
        let bytes = try_encode_msg(&msg).expect("serialisation failed");

        let rmsg = try_decode_msg(&bytes).expect("deserialisation failed");
        assert_eq!(rmsg.msg_id(), "get_instances");
        assert_eq!(rmsg, msg);

        let bytes2 = try_encode_msg(&rmsg).expect("2nd serialisation failed");
        assert_eq!(bytes, bytes2);
    }

    #[test]
    fn roundtrip_get_supported_dm_request() {
        let body = GetSupportedDMBuilder::new()
            .with_first_level_only(true)
            .with_return_events(false)
            .with_return_commands(false)
            .with_return_params(false)
            .with_obj_paths(vec!["Device.Foo.".into(), "Device.Bar.".into()])
            .build()
            .unwrap();

        let msg = MsgBuilder::new()
            .with_msg_id("get_supported_dm".into())
            .with_body(body)
            .build()
            .unwrap();
        let bytes = try_encode_msg(&msg).expect("serialisation failed");

        let rmsg = try_decode_msg(&bytes).expect("deserialisation failed");
        assert_eq!(rmsg.msg_id(), "get_supported_dm");
        assert_eq!(rmsg, msg);

        let bytes2 = try_encode_msg(&rmsg).expect("2nd serialisation failed");
        assert_eq!(bytes, bytes2);
    }

    #[test]
    fn roundtrip_get_supported_protocol_request() {
        let body = GetSupportedProtocolBuilder::new("1.1".into())
            .build()
            .unwrap();

        let msg = MsgBuilder::new()
            .with_msg_id("gsp".into())
            .with_body(body)
            .build()
            .unwrap();
        let bytes = try_encode_msg(&msg).expect("serialisation failed");

        let rmsg = try_decode_msg(&bytes).expect("deserialisation failed");
        assert_eq!(rmsg.msg_id(), "gsp");
        assert_eq!(rmsg, msg);

        let bytes2 = try_encode_msg(&rmsg).expect("2nd serialisation failed");
        assert_eq!(bytes, bytes2);
    }

    #[test]
    fn roundtrip_operate_request() {
        let body = OperateBuilder::new("Device.Reset()".into())
            .with_command_key("reset".into())
            .with_send_resp(true)
            .with_input_args(vec![("Hard".into(), "Yes, please!".into())])
            .build()
            .unwrap();

        let msg = MsgBuilder::new()
            .with_msg_id("operate".into())
            .with_body(body)
            .build()
            .unwrap();
        let bytes = try_encode_msg(&msg).expect("serialisation failed");

        let rmsg = try_decode_msg(&bytes).expect("deserialisation failed");
        assert_eq!(rmsg.msg_id(), "operate");
        assert_eq!(rmsg, msg);

        let bytes2 = try_encode_msg(&rmsg).expect("2nd serialisation failed");
        assert_eq!(bytes, bytes2);
    }

    #[test]
    fn roundtrip_set_request() {
        let updated_objs = UpdateObjectBuilder::new("Device.Foo.1.".into())
            .with_param_settings(vec![("Param".into(), "Value".into(), true)]);
        let body = SetBuilder::new()
            .with_allow_partial(true)
            .with_update_objs(vec![updated_objs])
            .build()
            .unwrap();

        let msg = MsgBuilder::new()
            .with_msg_id("set".into())
            .with_body(body)
            .build()
            .unwrap();
        let bytes = try_encode_msg(&msg).expect("serialisation failed");

        let rmsg = try_decode_msg(&bytes).expect("deserialisation failed");
        assert_eq!(rmsg.msg_id(), "set");
        assert_eq!(rmsg, msg);

        let bytes2 = try_encode_msg(&rmsg).expect("2nd serialisation failed");
        assert_eq!(bytes, bytes2);
    }

    #[test]
    fn roundtrip_notify_response() {
        let body = NotifyRespBuilder::new("notif-foo".into()).build().unwrap();
        let msg = MsgBuilder::new()
            .with_msg_id("notif-foo".into())
            .with_body(body)
            .build()
            .unwrap();
        let bytes = try_encode_msg(&msg).expect("serialisation failed");

        let rmsg = try_decode_msg(&bytes).expect("deserialisation failed");
        assert_eq!(rmsg.msg_id(), "notif-foo");
        assert_eq!(rmsg, msg);

        let bytes2 = try_encode_msg(&rmsg).expect("2nd serialisation failed");
        assert_eq!(bytes, bytes2);
    }

    #[test]
    fn roundtrip_add_response() {
        let mut keys = HashMap::new();
        keys.insert("EndpointID".into(), "controller-temp".into());
        let oper_status = AddOperationStatus::new().set_success(
            "Device.LocalAgent.Controller.31185.".into(),
            Vec::new(),
            keys,
        );
        let created_obj_results =
            CreatedObjectResultsBuilder::new("Device.LocalAgent.Controller.".into(), oper_status);
        let body = AddRespBuilder::new()
            .with_created_obj_results(vec![created_obj_results])
            .build()
            .unwrap();

        let msg = MsgBuilder::new()
            .with_msg_id("add-rsp".into())
            .with_body(body)
            .build()
            .unwrap();
        let bytes = try_encode_msg(&msg).expect("serialisation failed");

        let rmsg = try_decode_msg(&bytes).expect("deserialisation failed");
        assert_eq!(msg.msg_id(), rmsg.msg_id());
        assert_eq!(msg, rmsg);

        let bytes2 = try_encode_msg(&rmsg).expect("2nd serialisation failed");
        assert_eq!(bytes, bytes2);
    }

    #[test]
    fn roundtrip_delete_response() {
        let deleted_objs = DeletedObjectResultsBuilder::new(
            "Device.LocalAgent.Controller.[EndpointID==\"controller-temp\"].".into(),
        )
        .set_success(
            vec!["Device.LocalAgent.Controller.31185.".into()],
            Vec::new(),
        );
        let body = DeleteRespBuilder::new()
            .with_deleted_obj_results(vec![deleted_objs])
            .build()
            .unwrap();

        let msg = MsgBuilder::new()
            .with_msg_id("delete-rsp".into())
            .with_body(body)
            .build()
            .unwrap();
        let bytes = try_encode_msg(&msg).expect("serialisation failed");

        let rmsg = try_decode_msg(&bytes).expect("deserialisation failed");
        assert_eq!(msg.msg_id(), rmsg.msg_id());
        assert_eq!(msg, rmsg);

        let bytes2 = try_encode_msg(&rmsg).expect("2nd serialisation failed");
        assert_eq!(bytes, bytes2);
    }

    #[test]
    fn roundtrip_get_response() {
        let resolved = ResolvedPathResultBuilder::new("Device.WiFi.SSID.1.".into())
            .with_result_params(vec![("Enable".into(), "True".into())]);
        let path_result = GetReqPathResultBuilder::new(
            "Device.WiFi.SSID.[SSID==\"Homenetwork\"&&BSSID=00:11:22:33:44:55].".into(),
        )
        .with_res_path_results(vec![resolved]);
        let body = GetRespBuilder::new()
            .with_req_path_results(vec![path_result])
            .build()
            .unwrap();

        let msg = MsgBuilder::new()
            .with_msg_id("get-rsp".into())
            .with_body(body)
            .build()
            .unwrap();
        let bytes = try_encode_msg(&msg).expect("serialisation failed");

        let rmsg = try_decode_msg(&bytes).expect("deserialisation failed");
        assert_eq!(msg.msg_id(), rmsg.msg_id());
        assert_eq!(msg, rmsg);

        let bytes2 = try_encode_msg(&rmsg).expect("2nd serialisation failed");
        assert_eq!(bytes, bytes2);
    }

    #[test]
    fn roundtrip_get_instances_response() {
        let curr_instances = CurrInstanceBuilder::new("Device.WiFi.SSID.1.".into())
            .with_unique_keys(vec![("Alias".into(), "UserWiFi1".into())]);
        let req_path_results =
            GetInstancesRespReqPathResultBuilder::new("Device.WiFi.SSID.".into())
                .with_curr_insts(vec![curr_instances]);
        let body = GetInstancesRespBuilder::new()
            .with_req_path_results(vec![req_path_results])
            .build()
            .unwrap();

        let msg = MsgBuilder::new()
            .with_msg_id("get-instances-rsp".into())
            .with_body(body)
            .build()
            .unwrap();
        let bytes = try_encode_msg(&msg).expect("serialisation failed");

        let rmsg = try_decode_msg(&bytes).expect("deserialisation failed");
        assert_eq!(msg.msg_id(), rmsg.msg_id());
        assert_eq!(msg, rmsg);

        let bytes2 = try_encode_msg(&rmsg).expect("2nd serialisation failed");
        assert_eq!(bytes, bytes2);
    }

    #[test]
    fn roundtrip_get_supported_dm_response() {
        let param_result = GSDMParamResult::new("Device.WiFi.Radio.{i}.".into())
            .set_access_read_only()
            .set_type_string()
            .set_value_change_allowed();
        let cmd_result = GSDMCommandResult::new("NeighboringWiFiDiagnostic()".into())
            .with_input_arg_names(vec!["a".into()])
            .with_output_arg_names(vec!["b".into()])
            .set_sync();
        let event_result = GSDMEventResult::new("EventA".into()).with_arg_names(vec!["a".into()]);

        let supported_obj_result = GSDMSupportedObjectResultBuilder::new("Device.WiFi.".into())
            .set_access_read_only()
            .with_is_multi_instance(false)
            .with_supported_params(vec![param_result])
            .with_supported_commands(vec![cmd_result])
            .with_supported_events(vec![event_result])
            .with_divergent_paths(vec!["DivergentPath".into()]);
        let req_obj_results = GSDMReqObjectResultBuilder::new("Device.WiFi.".into())
            .with_data_model_inst_uri("urn:broadband-forum-org:tr-181-2-12-0".into())
            .with_supported_objs(vec![supported_obj_result]);
        let body = GetSupportedDMRespBuilder::new()
            .with_req_obj_results(vec![req_obj_results])
            .build()
            .unwrap();

        let msg = MsgBuilder::new()
            .with_msg_id("get-supported-dm-rsp".into())
            .with_body(body)
            .build()
            .unwrap();
        let bytes = try_encode_msg(&msg).expect("serialisation failed");

        let rmsg = try_decode_msg(&bytes).expect("deserialisation failed");
        assert_eq!(msg.msg_id(), rmsg.msg_id());
        assert_eq!(msg, rmsg);

        let bytes2 = try_encode_msg(&rmsg).expect("2nd serialisation failed");
        assert_eq!(bytes, bytes2);
    }

    #[test]
    fn roundtrip_get_supported_protocol_response() {
        let body = GetSupportedProtocolRespBuilder::new("1.1".into())
            .build()
            .unwrap();

        let msg = MsgBuilder::new()
            .with_msg_id("get-supported-protocol-rsp".into())
            .with_body(body)
            .build()
            .unwrap();
        let bytes = try_encode_msg(&msg).expect("serialisation failed");

        let rmsg = try_decode_msg(&bytes).expect("deserialisation failed");
        assert_eq!(msg.msg_id(), rmsg.msg_id());
        assert_eq!(msg, rmsg);

        let bytes2 = try_encode_msg(&rmsg).expect("2nd serialisation failed");
        assert_eq!(bytes, bytes2);
    }

    #[test]
    fn roundtrip_operate_response() {
        let oper_sync = OperateRespResultBuilder::new(
            "Device.LocalAgent.Controller.1.SendOnBoardRequest()".into(),
        )
        .set_output_args(vec![("a".into(), "b".into())]);
        let oper_async =
            OperateRespResultBuilder::new("Device.LocalAgent.Controller.1.Send()".into())
                .set_path("Path".into());
        let oper_error =
            OperateRespResultBuilder::new("Device.LocalAgent.Controller.1.Wake()".into())
                .set_failure(7000, Some("SomeError".into()));
        let body = OperateRespBuilder::new()
            .with_operation_results(vec![oper_sync, oper_async, oper_error])
            .build()
            .unwrap();

        let msg = MsgBuilder::new()
            .with_msg_id("operate-rsp".into())
            .with_body(body)
            .build()
            .unwrap();
        let bytes = try_encode_msg(&msg).expect("serialisation failed");

        let rmsg = try_decode_msg(&bytes).expect("deserialisation failed");
        assert_eq!(msg.msg_id(), rmsg.msg_id());
        assert_eq!(msg, rmsg);

        let bytes2 = try_encode_msg(&rmsg).expect("2nd serialisation failed");
        assert_eq!(bytes, bytes2);
    }

    #[test]
    fn roundtrip_set_response() {
        let mut params = HashMap::new();
        params.insert("FriendlyName".into(), "MyDevicesFriendlyName".into());
        let success = SetOperationSuccessBuilder::new("Device.DeviceInfo.".into())
            .with_updated_params(params);

        // XXX: Should this be named `SetOperationStatusBuilder` ?
        let oper_status = SetOperationStatus::new().set_success(vec![success]);
        let updated_obj_results =
            UpdatedObjectResultsBuilder::new("Device.DeviceInfo.".into(), oper_status);
        let body = SetRespBuilder::new()
            .with_updated_obj_results(vec![updated_obj_results])
            .build()
            .unwrap();

        let msg = MsgBuilder::new()
            .with_msg_id("set-rsp".into())
            .with_body(body)
            .build()
            .unwrap();
        let bytes = try_encode_msg(&msg).expect("serialisation failed");

        let rmsg = try_decode_msg(&bytes).expect("deserialisation failed");
        assert_eq!(msg.msg_id(), rmsg.msg_id());
        assert_eq!(msg, rmsg);

        let bytes2 = try_encode_msg(&rmsg).expect("2nd serialisation failed");
        assert_eq!(bytes, bytes2);
    }

    #[test]
    fn get_response_fail() {
        let req_path_res = GetReqPathResultBuilder::new("Device.DeviceInfo.SerialNumber.".into())
            .set_err(7026, Some("Invalid path".into()));
        let body = GetRespBuilder::new()
            .with_req_path_results(vec![req_path_res])
            .build()
            .unwrap();

        let msg = MsgBuilder::new()
            .with_msg_id("o0qZQ4NaYPjRJyckiYlj8w==".into())
            .with_body(body)
            .build()
            .unwrap();
        let data = b"\
            \x0a\x1c\x0a\x18o0qZQ4NaYPjRJyckiYlj8w==\x10\x02\x12\x3a\x128\x0a6\x0a4\x0a\x1f\
            Device.DeviceInfo.SerialNumber.\x15r\x1b\x00\x00\x1a\x0cInvalid path\
        ";

        let rmsg = try_decode_msg(data).expect("deserialisation failed");
        assert_eq!(msg, rmsg)
    }

    #[test]
    fn get_response_success() {
        let resolved = ResolvedPathResultBuilder::new("Device.DeviceInfo.".into())
            .with_result_params(vec![("SerialNumber".into(), "000000000000".into())]);
        let req_path_res = GetReqPathResultBuilder::new("Device.DeviceInfo.SerialNumber".into())
            .with_res_path_results(vec![resolved]);
        let body = GetRespBuilder::new()
            .with_req_path_results(vec![req_path_res])
            .build()
            .unwrap();

        let msg = MsgBuilder::new()
            .with_msg_id("A204VyO0B0mNj1DKnIMGlA==".into())
            .with_body(body)
            .build()
            .unwrap();
        let data = b"\
            \x0a\x1c\x0a\x18A204VyO0B0mNj1DKnIMGlA==\x10\x02\x12Z\x12X\x0aV\x0aT\x0a\x1e\
            Device.DeviceInfo.SerialNumber\x222\x0a\x12Device.DeviceInfo.\x12\x1c\x0a\x0c\
            SerialNumber\x12\x0c000000000000\
        ";

        let rmsg = try_decode_msg(data).expect("deserialisation failed");
        assert_eq!(msg, rmsg)
    }

    #[test]
    fn get_instances_response_success() {
        let data = b"\
            \x0a\x1c\x0a\x18DupoyC6r4HNdfwJtWduvWA==\x10\x0f\x12\xbd\x05\x12\xba\x05\x1a\xb7\x05\
            \x0a\xb4\x05\x0a\x1dDevice.LocalAgent.Controller.\x22T\x0a\x1f\
            Device.LocalAgent.Controller.1.\x12\x0f\x0a\x05Alias\x12\x06cpe-01\x12 \x0a\x0a\
            EndpointID\x12\x12proto\x3a\x3acontroller2\x22T\x0a\x1fDevice.LocalAgent.Controller.2.\
            \x12\x0f\x0a\x05Alias\x12\x06cpe-02\x12 \x0a\x0aEndpointID\x12\x12proto\x3a\x3a\
            controller3\x22X\x0a\x1fDevice.LocalAgent.Controller.3.\x12\x0f\x0a\x05Alias\x12\x06\
            cpe-03\x12\x24\x0a\x0aEndpointID\x12\x16proto\x3a\x3acontroller-mqtt\x22N\x0a\x1f\
            Device.LocalAgent.Controller.4.\x12\x0f\x0a\x05Alias\x12\x06cpe-04\x12\x1a\x0a\x0a\
            EndpointID\x12\x0cproto\x3a\x3aaxess\x22O\x0a\x25Device.LocalAgent.Controller.1.MTP.1.\
            \x12\x0f\x0a\x05Alias\x12\x06cpe-01\x12\x15\x0a\x08Protocol\x12\x09WebSocket\x22O\x0a\
            \x25Device.LocalAgent.Controller.2.MTP.1.\x12\x0f\x0a\x05Alias\x12\x06cpe-01\
            \x12\x15\x0a\x08Protocol\x12\x09WebSocket\x22J\x0a\x25Device.LocalAgent.Controller.3.\
            MTP.1.\x12\x0f\x0a\x05Alias\x12\x06cpe-01\x12\x10\x0a\x08Protocol\x12\x04MQTT\x22O\
            \x0a\x25Device.LocalAgent.Controller.4.MTP.1.\x12\x0f\x0a\x05Alias\x12\x06cpe-01\
            \x12\x15\x0a\x08Protocol\x12\x09WebSocket\
        ";

        let msg = try_decode_msg(data).expect("deserialisation failed");
        assert!(matches!(
            msg.body.unwrap().msg_body,
            OneOfmsg_body::response(Response {
                resp_type: OneOfresp_type::get_instances_resp(_)
            })
        ));
    }

    #[test]
    fn add_response_success() {
        let data = b"\
            \x0a\x1c\x0a\x18icfeSgQ89emT4XgldRvLjw==\x10\x09\x12\x9d\x01\x12\x9a\x01*\x97\x01\x0a\
            \x94\x01\x0a\x1fDevice.LocalAgent.Subscription.\x12q\x12o\x0a!Device.LocalAgent.\
            Subscription.7.\x1a\x0c\x0a\x02ID\x12\x06random\x1a\x0f\x0a\x05Alias\x12\x06cpe-07\x1a\
            \x2b\x0a\x09Recipient\x12\x1eDevice.LocalAgent.Controller.1\
        ";

        let msg = try_decode_msg(data).expect("deserialisation failed");
        assert!(matches!(
            msg.body.unwrap().msg_body,
            OneOfmsg_body::response(Response {
                resp_type: OneOfresp_type::add_resp(_)
            }),
        ));
    }

    #[test]
    fn delete_response_success() {
        let data = b"\
            \x0a\x1c\x0a\x18ZkNuikL7esrADyE4qDDAYw==\x10\x0b\x12P\x12N2L\x0aJ\x0a!Device\
            .LocalAgent.Subscription.6.\x12\x25\x12\x23\x0a!Device.LocalAgent.Subscription.6.\
        ";

        let msg = try_decode_msg(data).expect("deserialisation failed");
        assert!(matches!(
            msg.body.unwrap().msg_body,
            OneOfmsg_body::response(Response {
                resp_type: OneOfresp_type::delete_resp(_)
            }),
        ));
    }

    #[test]
    fn delete_response_failure() {
        let data = b"\
            \x0a\x1c\x0a\x18FwSOgGIWdYgEtJJ\x2bIPi9wQ==\x10\x0b\x12e\x12c2a\x0a_\x0a!Device\
            .LocalAgent.Subscription.6.\x12\x3a\x128\x126\x0a!Device.LocalAgent.Subscription.6.\
            \x15r\x1b\x00\x00\x1a\x0cInvalid path\
        ";

        let msg = try_decode_msg(data).expect("deserialisation failed");
        assert!(matches!(
            msg.body.unwrap().msg_body,
            OneOfmsg_body::response(Response {
                resp_type: OneOfresp_type::delete_resp(_)
            }),
        ));
    }

    #[test]
    fn gsp_response() {
        let body = GetSupportedProtocolRespBuilder::new("1.1".into())
            .build()
            .unwrap();
        let msg = MsgBuilder::new()
            .with_msg_id("CXOue8V6Pg/hYC4bKfO8rw==".into())
            .with_body(body)
            .build()
            .unwrap();

        let data = b"\n\x1c\n\x18CXOue8V6Pg/hYC4bKfO8rw==\x10\x12\x12\t\x12\x07J\x05\n\x031.1";
        let rmsg = try_decode_msg(data).expect("deserialisation failed");

        assert_eq!(msg, rmsg)
    }

    #[test]
    fn gsdm_response() {
        let data = b"\
            \x0a\x1c\x0a\x18Kvcqv3OyewkSHpzRL9LD5Q==\x10\x0d\x12\xac\x02\x12\xa9\x02\x12\xa6\x02\
            \x0a\xa3\x02\x0a\x12Device.DeviceInfo.*\x8c\x02\x0a\x12Device.DeviceInfo.2\x0e\x0a\x0c\
            Manufacturer2\x11\x0a\x0fManufacturerOUI2\x0b\x0a\x09ModelName2\x0d\x0a\x0b\
            Description2\x0e\x0a\x0cProductClass2\x0e\x0a\x0cSerialNumber2\x11\x0a\x0f\
            HardwareVersion2\x11\x0a\x0fSoftwareVersion2\x14\x0a\x10ProvisioningCode\x10\x012\x08\
            \x0a\x06UpTime2\x0e\x0a\x0cFirstUseDate2!\x0a\x1fVendorConfigFileNumberOfEntries2\x1e\
            \x0a\x1cVendorLogFileNumberOfEntries\
        ";
        let msg = try_decode_msg(data).expect("deserialisation failed");

        assert!(matches!(
            msg.body.unwrap().msg_body,
            OneOfmsg_body::response(Response {
                resp_type: OneOfresp_type::get_supported_dm_resp(_)
            }),
        ));
    }

    #[test]
    fn operate_response() {
        let data = b"\
            \x0a\x1c\x0a\x18MuyKWcNrbc2lFvQJO0peHA==\x10\x07\x12\x19\x12\x17\x3a\x15\x0a\x13\x0a\
            \x0fDevice.Reboot()\x1a\x00\
        ";
        let msg = try_decode_msg(data).expect("deserialisation failed");

        assert!(matches!(
            msg.body.unwrap().msg_body,
            OneOfmsg_body::response(Response {
                resp_type: OneOfresp_type::operate_resp(_)
            }),
        ));
    }

    #[test]
    fn set_response() {
        let data = b"\
            \x0a\x1c\x0a\x18OCmrwerp73bqFKd2ocHhBw==\x10\x05\x12Z\x12X\x22V\x0aT\x0a\x18\
            Device.ManagementServer.\x128\x126\x0a4\x0a\x18Device.ManagementServer.\x1a\x18\x0a\
            \x13AutoCreateInstances\x12\x011\
        ";
        let msg = try_decode_msg(data).expect("deserialisation failed");

        assert!(matches!(
            msg.body.unwrap().msg_body,
            OneOfmsg_body::response(Response {
                resp_type: OneOfresp_type::set_resp(_)
            }),
        ));
    }
}
