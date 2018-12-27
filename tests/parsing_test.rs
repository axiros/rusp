mod tests {
    use quick_protobuf::{BytesReader, MessageRead};
    use rusp::usp::Msg;
    use rusp::usp_record::Record;

    #[test]
    fn create_obj() {
        let bytes: Vec<u8> = vec![
            0x0a, 0x03, 0x31, 0x2e, 0x30, 0x12, 0x23, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x3a, 0x3a,
            0x61, 0x78, 0x2d, 0x75, 0x73, 0x70, 0x2d, 0x61, 0x67, 0x65, 0x6e, 0x74, 0x2d, 0x6e,
            0x6f, 0x73, 0x73, 0x6c, 0x2d, 0x77, 0x65, 0x62, 0x73, 0x6f, 0x63, 0x6b, 0x65, 0x74,
            0x1a, 0x1e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x3a, 0x3a, 0x61, 0x78, 0x2d, 0x75, 0x73,
            0x70, 0x2d, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x2d, 0x6e,
            0x6f, 0x73, 0x73, 0x6c, 0x3a, 0x6e, 0x12, 0x6c, 0x0a, 0x1a, 0x0a, 0x16, 0x41, 0x58,
            0x53, 0x53, 0x2d, 0x31, 0x35, 0x34, 0x34, 0x31, 0x31, 0x34, 0x30, 0x38, 0x33, 0x2e,
            0x37, 0x36, 0x31, 0x35, 0x30, 0x38, 0x10, 0x08, 0x12, 0x4e, 0x0a, 0x4c, 0x2a, 0x4a,
            0x08, 0x01, 0x12, 0x46, 0x0a, 0x1d, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x4c,
            0x6f, 0x63, 0x61, 0x6c, 0x41, 0x67, 0x65, 0x6e, 0x74, 0x2e, 0x43, 0x6f, 0x6e, 0x74,
            0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x2e, 0x12, 0x0f, 0x0a, 0x05, 0x41, 0x6c, 0x69,
            0x61, 0x73, 0x12, 0x04, 0x74, 0x65, 0x73, 0x74, 0x18, 0x01, 0x12, 0x14, 0x0a, 0x0a,
            0x45, 0x6e, 0x64, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x49, 0x44, 0x12, 0x04, 0x74, 0x65,
            0x73, 0x74, 0x18, 0x01,
        ];

        use rusp::usp::mod_Body::OneOfmsg_body::request;
        use rusp::usp::mod_Header::MsgType::ADD;
        use rusp::usp::mod_Request::OneOfreq_type::add;
        use rusp::usp_record::mod_Record::OneOfrecord_type::no_session_context;

        let mut reader = BytesReader::from_bytes(&bytes);
        let record = Record::from_reader(&mut reader, &bytes).expect("Cannot read Record");

        assert!(record.version.clone().unwrap() == "1.0");
        assert!(record.to_id.clone().unwrap() == "proto::ax-usp-agent-nossl-websocket");
        assert!(record.from_id.clone().unwrap() == "proto::ax-usp-controller-nossl");
        assert!(record.payload_security == None);
        assert!(record.mac_signature == None);
        assert!(record.sender_cert == None);
        assert!(if let no_session_context(_) = record.record_type {
            true
        } else {
            false
        });

        if let no_session_context(context) = record.record_type {
            if let Some(payload) = context.payload {
                let mut reader = BytesReader::from_bytes(&payload);
                let msg = Msg::from_reader(&mut reader, &payload).expect("Cannot read Msg");
                assert!(msg.header.is_some());

                if let Some(header) = msg.header {
                    assert!(header.msg_id.clone().unwrap() == "AXSS-1544114083.761508");
                    assert!(header.msg_type == Some(ADD));
                }

                if let Some(body) = msg.body {
                    if let request(req) = body.msg_body {
                        assert!(if let add(_) = req.req_type {
                            true
                        } else {
                            false
                        });

                        if let add(a) = req.req_type {
                            assert!(a.allow_partial == Some(true));
                            assert!(a.create_objs.len() == 1);
                            let createobj = &a.create_objs[0];
                            assert!(
                                createobj.obj_path.clone().unwrap()
                                    == "Device.LocalAgent.Controller."
                            );
                            assert!(createobj.param_settings.len() == 2);
                            let param1 = createobj.param_settings[0].clone();
                            assert!(param1.param.clone().unwrap() == "Alias");
                            assert!(param1.value.clone().unwrap() == "test");
                            assert!(param1.required.clone().unwrap() == true);

                            let param2 = createobj.param_settings[1].clone();
                            assert!(param2.param.clone().unwrap() == "EndpointID");
                            assert!(param2.value.clone().unwrap() == "test");
                            assert!(param2.required.clone().unwrap() == true);
                        }
                    }
                }
            }
        };
    }

    #[test]
    fn delete_obj() {
        let bytes: Vec<u8> = vec![
            0x0a, 0x03, 0x31, 0x2e, 0x30, 0x12, 0x23, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x3a, 0x3a,
            0x61, 0x78, 0x2d, 0x75, 0x73, 0x70, 0x2d, 0x61, 0x67, 0x65, 0x6e, 0x74, 0x2d, 0x6e,
            0x6f, 0x73, 0x73, 0x6c, 0x2d, 0x77, 0x65, 0x62, 0x73, 0x6f, 0x63, 0x6b, 0x65, 0x74,
            0x1a, 0x1e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x3a, 0x3a, 0x61, 0x78, 0x2d, 0x75, 0x73,
            0x70, 0x2d, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x2d, 0x6e,
            0x6f, 0x73, 0x73, 0x6c, 0x3a, 0x4a, 0x12, 0x48, 0x0a, 0x1a, 0x0a, 0x16, 0x41, 0x58,
            0x53, 0x53, 0x2d, 0x31, 0x35, 0x34, 0x34, 0x31, 0x31, 0x34, 0x31, 0x30, 0x32, 0x2e,
            0x36, 0x36, 0x38, 0x34, 0x33, 0x39, 0x10, 0x0a, 0x12, 0x2a, 0x0a, 0x28, 0x32, 0x26,
            0x08, 0x01, 0x12, 0x22, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x4c, 0x6f, 0x63,
            0x61, 0x6c, 0x41, 0x67, 0x65, 0x6e, 0x74, 0x2e, 0x4d, 0x54, 0x50, 0x2e, 0x31, 0x2e,
            0x57, 0x65, 0x62, 0x53, 0x6f, 0x63, 0x6b, 0x65, 0x74, 0x2e,
        ];

        use rusp::usp::mod_Body::OneOfmsg_body::request;
        use rusp::usp::mod_Header::MsgType::DELETE;
        use rusp::usp::mod_Request::OneOfreq_type::delete;
        use rusp::usp_record::mod_Record::OneOfrecord_type::no_session_context;

        let mut reader = BytesReader::from_bytes(&bytes);
        let record = Record::from_reader(&mut reader, &bytes).expect("Cannot read Record");

        assert!(record.version.clone().unwrap() == "1.0");
        assert!(record.to_id.clone().unwrap() == "proto::ax-usp-agent-nossl-websocket");
        assert!(record.from_id.clone().unwrap() == "proto::ax-usp-controller-nossl");
        assert!(record.payload_security == None);
        assert!(record.mac_signature == None);
        assert!(record.sender_cert == None);
        assert!(if let no_session_context(_) = record.record_type {
            true
        } else {
            false
        });

        if let no_session_context(context) = record.record_type {
            if let Some(payload) = context.payload {
                let mut reader = BytesReader::from_bytes(&payload);
                let msg = Msg::from_reader(&mut reader, &payload).expect("Cannot read Msg");
                assert!(msg.header.is_some());

                if let Some(header) = msg.header {
                    assert!(header.msg_id.clone().unwrap() == "AXSS-1544114102.668439");
                    assert!(header.msg_type == Some(DELETE));
                }

                if let Some(body) = msg.body {
                    if let request(req) = body.msg_body {
                        assert!(if let delete(_) = req.req_type {
                            true
                        } else {
                            false
                        });

                        if let delete(a) = req.req_type {
                            assert!(a.allow_partial == Some(true));
                            assert!(a.obj_paths.len() == 1);
                            assert!(
                                &a.obj_paths[0].clone() == "Device.LocalAgent.MTP.1.WebSocket."
                            );
                        }
                    }
                }
            }
        };
    }

    #[test]
    fn get_obj() {
        let bytes: Vec<u8> = vec![
            0x0a, 0x03, 0x31, 0x2e, 0x30, 0x12, 0x23, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x3a, 0x3a,
            0x61, 0x78, 0x2d, 0x75, 0x73, 0x70, 0x2d, 0x61, 0x67, 0x65, 0x6e, 0x74, 0x2d, 0x6e,
            0x6f, 0x73, 0x73, 0x6c, 0x2d, 0x77, 0x65, 0x62, 0x73, 0x6f, 0x63, 0x6b, 0x65, 0x74,
            0x1a, 0x1e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x3a, 0x3a, 0x61, 0x78, 0x2d, 0x75, 0x73,
            0x70, 0x2d, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x2d, 0x6e,
            0x6f, 0x73, 0x73, 0x6c, 0x3a, 0x48, 0x12, 0x46, 0x0a, 0x1a, 0x0a, 0x16, 0x41, 0x58,
            0x53, 0x53, 0x2d, 0x31, 0x35, 0x34, 0x34, 0x31, 0x31, 0x34, 0x30, 0x34, 0x35, 0x2e,
            0x34, 0x34, 0x32, 0x35, 0x39, 0x36, 0x10, 0x01, 0x12, 0x28, 0x0a, 0x26, 0x0a, 0x24,
            0x0a, 0x22, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x4c, 0x6f, 0x63, 0x61, 0x6c,
            0x41, 0x67, 0x65, 0x6e, 0x74, 0x2e, 0x4d, 0x54, 0x50, 0x2e, 0x31, 0x2e, 0x57, 0x65,
            0x62, 0x53, 0x6f, 0x63, 0x6b, 0x65, 0x74, 0x2e,
        ];

        use rusp::usp::mod_Body::OneOfmsg_body::request;
        use rusp::usp::mod_Header::MsgType::GET;
        use rusp::usp::mod_Request::OneOfreq_type::get;
        use rusp::usp_record::mod_Record::OneOfrecord_type::no_session_context;

        let mut reader = BytesReader::from_bytes(&bytes);
        let record = Record::from_reader(&mut reader, &bytes).expect("Cannot read Record");

        assert!(record.version.clone().unwrap() == "1.0");
        assert!(record.to_id.clone().unwrap() == "proto::ax-usp-agent-nossl-websocket");
        assert!(record.from_id.clone().unwrap() == "proto::ax-usp-controller-nossl");
        assert!(record.payload_security == None);
        assert!(record.mac_signature == None);
        assert!(record.sender_cert == None);
        assert!(if let no_session_context(_) = record.record_type {
            true
        } else {
            false
        });

        if let no_session_context(context) = record.record_type {
            if let Some(payload) = context.payload {
                let mut reader = BytesReader::from_bytes(&payload);
                let msg = Msg::from_reader(&mut reader, &payload).expect("Cannot read Msg");
                assert!(msg.header.is_some());

                if let Some(header) = msg.header {
                    assert!(header.msg_id.clone().unwrap() == "AXSS-1544114045.442596");
                    assert!(header.msg_type == Some(GET));
                }

                if let Some(body) = msg.body {
                    if let request(req) = body.msg_body {
                        assert!(if let get(_) = req.req_type {
                            true
                        } else {
                            false
                        });

                        if let get(a) = req.req_type {
                            assert!(a.param_paths.len() == 1);
                            assert!(
                                &a.param_paths[0].clone() == "Device.LocalAgent.MTP.1.WebSocket."
                            );
                        }
                    }
                }
            }
        };
    }
}
