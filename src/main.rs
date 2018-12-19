use quick_protobuf;

mod usp;
mod usp_decoder;
mod usp_formatter;
mod usp_generator;
mod usp_record;

use self::usp_decoder::*;
use self::usp_generator::*;

use quick_protobuf::Writer;

fn main() {
    println!(
        "{}",
        decode_record(&vec![
            0x0a, 0x03, 0x31, 0x2e, 0x30, 0x1a, 0x23, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x3a, 0x3a,
            0x61, 0x78, 0x2d, 0x75, 0x73, 0x70, 0x2d, 0x61, 0x67, 0x65, 0x6e, 0x74, 0x2d, 0x6e,
            0x6f, 0x73, 0x73, 0x6c, 0x2d, 0x77, 0x65, 0x62, 0x73, 0x6f, 0x63, 0x6b, 0x65, 0x74,
            0x3a, 0x4d, 0x12, 0x4b, 0x0a, 0x08, 0x0a, 0x04, 0x74, 0x65, 0x73, 0x74, 0x10, 0x03,
            0x12, 0x3f, 0x0a, 0x3d, 0x42, 0x3b, 0x0a, 0x0f, 0x73, 0x75, 0x62, 0x73, 0x63, 0x72,
            0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x42, 0x28, 0x0a, 0x03, 0x6f,
            0x75, 0x69, 0x12, 0x0d, 0x70, 0x72, 0x6f, 0x64, 0x75, 0x63, 0x74, 0x5f, 0x63, 0x6c,
            0x61, 0x73, 0x73, 0x1a, 0x0d, 0x73, 0x65, 0x72, 0x69, 0x61, 0x6c, 0x5f, 0x6e, 0x75,
            0x6d, 0x62, 0x65, 0x72, 0x22, 0x03, 0x31, 0x2e, 0x30,
        ])
    );

    println!(
        "{}",
        decode_record(&vec![
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
        ])
    );

    println!(
        "{}",
        decode_record(&vec![
            0x0a, 0x03, 0x31, 0x2e, 0x30, 0x12, 0x1e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x3a, 0x3a,
            0x61, 0x78, 0x2d, 0x75, 0x73, 0x70, 0x2d, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c,
            0x6c, 0x65, 0x72, 0x2d, 0x6e, 0x6f, 0x73, 0x73, 0x6c, 0x1a, 0x23, 0x70, 0x72, 0x6f,
            0x74, 0x6f, 0x3a, 0x3a, 0x61, 0x78, 0x2d, 0x75, 0x73, 0x70, 0x2d, 0x61, 0x67, 0x65,
            0x6e, 0x74, 0x2d, 0x6e, 0x6f, 0x73, 0x73, 0x6c, 0x2d, 0x77, 0x65, 0x62, 0x73, 0x6f,
            0x63, 0x6b, 0x65, 0x74, 0x3a, 0x66, 0x12, 0x64, 0x0a, 0x1a, 0x0a, 0x16, 0x41, 0x58,
            0x53, 0x53, 0x2d, 0x31, 0x35, 0x34, 0x34, 0x31, 0x31, 0x34, 0x30, 0x34, 0x35, 0x2e,
            0x34, 0x34, 0x32, 0x35, 0x39, 0x36, 0x10, 0x02, 0x12, 0x46, 0x12, 0x44, 0x0a, 0x42,
            0x0a, 0x40, 0x0a, 0x22, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x4c, 0x6f, 0x63,
            0x61, 0x6c, 0x41, 0x67, 0x65, 0x6e, 0x74, 0x2e, 0x4d, 0x54, 0x50, 0x2e, 0x31, 0x2e,
            0x57, 0x65, 0x62, 0x53, 0x6f, 0x63, 0x6b, 0x65, 0x74, 0x2e, 0x15, 0x62, 0x1b, 0x00,
            0x00, 0x1a, 0x15, 0x55, 0x6e, 0x73, 0x75, 0x70, 0x70, 0x6f, 0x72, 0x74, 0x65, 0x64,
            0x20, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72,
        ])
    );

    println!(
        "{}",
        decode_msg(&vec![
            10, 26, 10, 22, 65, 88, 83, 83, 45, 49, 53, 52, 52, 49, 49, 52, 48, 56, 51, 46, 55, 54,
            49, 53, 48, 56, 16, 8, 18, 78, 10, 76, 42, 74, 8, 1, 18, 70, 10, 29, 68, 101, 118, 105,
            99, 101, 46, 76, 111, 99, 97, 108, 65, 103, 101, 110, 116, 46, 67, 111, 110, 116, 114,
            111, 108, 108, 101, 114, 46, 18, 15, 10, 5, 65, 108, 105, 97, 115, 18, 4, 116, 101,
            115, 116, 24, 1, 18, 20, 10, 10, 69, 110, 100, 112, 111, 105, 110, 116, 73, 68, 18, 4,
            116, 101, 115, 116, 24, 1
        ])
    );

    println!(
        "{}",
        decode_msg(&vec![
            0x0a, 0x1a, 0x0a, 0x16, 0x41, 0x58, 0x53, 0x53, 0x2d, 0x31, 0x35, 0x34, 0x34, 0x31,
            0x31, 0x34, 0x30, 0x34, 0x35, 0x2e, 0x34, 0x34, 0x32, 0x35, 0x39, 0x36, 0x10, 0x01,
            0x12, 0x28, 0x0a, 0x26, 0x0a, 0x24, 0x0a, 0x22, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65,
            0x2e, 0x4c, 0x6f, 0x63, 0x61, 0x6c, 0x41, 0x67, 0x65, 0x6e, 0x74, 0x2e, 0x4d, 0x54,
            0x50, 0x2e, 0x31, 0x2e, 0x57, 0x65, 0x62, 0x53, 0x6f, 0x63, 0x6b, 0x65, 0x74, 0x2e
        ])
    );

    let newmsg = usp_msg(
        &"fancymsgid",
        usp_get_request(vec!["Device.", "Device.DeviceInfo."]),
    );

    println!("New message {}", newmsg);

    let newmsg = usp_msg(
        &"fancymsgid",
        usp_get_response(vec![
            ("Device.", Ok(vec![("Device.", vec![("Foo", "Bar")])])),
            ("Dev.", Err((7000, "Message failed"))),
        ]),
    );

    println!("New message {}", newmsg);

    let mut buf = Vec::new();
    let mut writer = Writer::new(&mut buf);
    writer.write_message(&newmsg).expect("Cannot write message");

    std::fs::write("foo", buf).unwrap();
}
