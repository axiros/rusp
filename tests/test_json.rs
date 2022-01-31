mod tests {
    use quick_protobuf::{BytesReader, MessageRead};
    use rusp::usp_record::Record;

    #[test]
    fn simple_notify() {
        let bytes: Vec<u8> = vec![
            0x0a, 0x03, 0x31, 0x2e, 0x30, 0x1a, 0x23, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x3a, 0x3a,
            0x61, 0x78, 0x2d, 0x75, 0x73, 0x70, 0x2d, 0x61, 0x67, 0x65, 0x6e, 0x74, 0x2d, 0x6e,
            0x6f, 0x73, 0x73, 0x6c, 0x2d, 0x77, 0x65, 0x62, 0x73, 0x6f, 0x63, 0x6b, 0x65, 0x74,
            0x3a, 0x4d, 0x12, 0x4b, 0x0a, 0x08, 0x0a, 0x04, 0x74, 0x65, 0x73, 0x74, 0x10, 0x03,
            0x12, 0x3f, 0x0a, 0x3d, 0x42, 0x3b, 0x0a, 0x0f, 0x73, 0x75, 0x62, 0x73, 0x63, 0x72,
            0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x42, 0x28, 0x0a, 0x03, 0x6f,
            0x75, 0x69, 0x12, 0x0d, 0x70, 0x72, 0x6f, 0x64, 0x75, 0x63, 0x74, 0x5f, 0x63, 0x6c,
            0x61, 0x73, 0x73, 0x1a, 0x0d, 0x73, 0x65, 0x72, 0x69, 0x61, 0x6c, 0x5f, 0x6e, 0x75,
            0x6d, 0x62, 0x65, 0x72, 0x22, 0x03, 0x31, 0x2e, 0x30,
        ];

        let mut reader = BytesReader::from_bytes(&bytes);
        let record = Record::from_reader(&mut reader, &bytes).expect("Cannot read Record");

        let serialized = serde_json::to_string_pretty(&record).unwrap();
        println!("serialized = {}", serialized);
        assert_eq!(
            serialized,
            r#"{
  "version": "1.0",
  "to_id": "",
  "from_id": "proto::ax-usp-agent-nossl-websocket",
  "payload_security": "PLAINTEXT",
  "mac_signature": [],
  "sender_cert": [],
  "payload": {
    "Header": {
      "msg_id": "test",
      "msg_type": "NOTIFY"
    },
    "Body": {
      "Request": {
        "Notify": {
          "subscription_id": "subscription_id",
          "send_resp": false,
          "on_board_req": {
            "oui": "oui",
            "product_class": "product_class",
            "serial_number": "serial_number",
            "agent_supported_protocol_versions": "1.0"
          }
        }
      }
    }
  }
}"#
        );
    }

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

        let mut reader = BytesReader::from_bytes(&bytes);
        let record = Record::from_reader(&mut reader, &bytes).expect("Cannot read Record");

        let serialized = serde_json::to_string_pretty(&record).unwrap();
        assert_eq!(
            serialized,
            r#"{
  "version": "1.0",
  "to_id": "proto::ax-usp-agent-nossl-websocket",
  "from_id": "proto::ax-usp-controller-nossl",
  "payload_security": "PLAINTEXT",
  "mac_signature": [],
  "sender_cert": [],
  "payload": {
    "Header": {
      "msg_id": "AXSS-1544114083.761508",
      "msg_type": "ADD"
    },
    "Body": {
      "Request": {
        "Add": {
          "allow_partial": true,
          "create_objs": [
            {
              "obj_path": "Device.LocalAgent.Controller.",
              "param_settings": [
                {
                  "param": "Alias",
                  "value": "test",
                  "required": true
                },
                {
                  "param": "EndpointID",
                  "value": "test",
                  "required": true
                }
              ]
            }
          ]
        }
      }
    }
  }
}"#
        );
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

        let mut reader = BytesReader::from_bytes(&bytes);
        let record = Record::from_reader(&mut reader, &bytes).expect("Cannot read Record");

        let serialized = serde_json::to_string_pretty(&record).unwrap();
        assert_eq!(
            serialized,
            r#"{
  "version": "1.0",
  "to_id": "proto::ax-usp-agent-nossl-websocket",
  "from_id": "proto::ax-usp-controller-nossl",
  "payload_security": "PLAINTEXT",
  "mac_signature": [],
  "sender_cert": [],
  "payload": {
    "Header": {
      "msg_id": "AXSS-1544114102.668439",
      "msg_type": "DELETE"
    },
    "Body": {
      "Request": {
        "Delete": {
          "allow_partial": true,
          "obj_paths": [
            "Device.LocalAgent.MTP.1.WebSocket."
          ]
        }
      }
    }
  }
}"#
        );
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

        let mut reader = BytesReader::from_bytes(&bytes);
        let record = Record::from_reader(&mut reader, &bytes).expect("Cannot read Record");

        let serialized = serde_json::to_string_pretty(&record).unwrap();
        println!("serialized = {}", serialized);
        assert_eq!(
            serialized,
            r#"{
  "version": "1.0",
  "to_id": "proto::ax-usp-agent-nossl-websocket",
  "from_id": "proto::ax-usp-controller-nossl",
  "payload_security": "PLAINTEXT",
  "mac_signature": [],
  "sender_cert": [],
  "payload": {
    "Header": {
      "msg_id": "AXSS-1544114045.442596",
      "msg_type": "GET"
    },
    "Body": {
      "Request": {
        "Get": {
          "param_paths": [
            "Device.LocalAgent.MTP.1.WebSocket."
          ],
          "max_depth": 0
        }
      }
    }
  }
}"#
        );
    }
}
