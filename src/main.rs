use quick_protobuf;
use serde_json;
use std::fs::File;
use std::io::{stdin, stdout, BufReader, Read, Write};
use std::path::PathBuf;
use structopt::*;

use rusp::{
    usp_decoder::{decode_msg, decode_record},
    usp_generator,
    usp_types::NotifyType,
};

#[derive(StructOpt)]
#[structopt(name = "rusp", about = "the Rust USP toolkit")]
enum Rusp {
    /// Decode a single raw USP message from standard input and print to standard output
    #[structopt(name = "decode_msg")]
    DecodeMsg {},
    /// Decode a multiple USP messages from specified filenames and print to standard output
    #[structopt(name = "decode_msg_files")]
    DecodeMsgFiles {
        #[structopt(parse(from_os_str), required = true)]
        /// Filenames of USP protobuf messages to decode
        files: Vec<PathBuf>,
    },
    /// Decode a single raw USP record from standard input and print to standard output
    #[structopt(name = "decode_record")]
    DecodeRecord {},
    /// Decode a multiple USP records from specified filenames and print to standard output
    #[structopt(name = "decode_record_files")]
    DecodeRecordFiles {
        #[structopt(parse(from_os_str), required = true)]
        /// Filenames of USP protobuf records to decode
        files: Vec<PathBuf>,
    },
    /// Encode command line input into a single raw USP message
    #[structopt(name = "encode_msg")]
    EncodeMsg {
        /// The message ID to use in the USP Msg header
        msgid: String,
        /// Filename (will output to standard output if omitted)
        #[structopt(parse(from_os_str), short = "f", long = "file")]
        /// Output filename of file to encode USP protobuf message to
        filename: Option<PathBuf>,
        /// Type of message
        #[structopt(subcommand)]
        typ: MsgType,
    },
    /// Encode command line input into a single raw USP message body
    #[structopt(name = "encode_msg_body")]
    EncodeMsgBody {
        /// Filename (will output to standard output if omitted)
        #[structopt(parse(from_os_str), short = "f", long = "file")]
        /// Output filename of file to encode USP protobuf message to
        filename: Option<PathBuf>,
        /// Type of message
        #[structopt(subcommand)]
        typ: MsgType,
    },
    /// Extract the USP message from an USP record
    #[structopt(name = "extract_msg")]
    ExtractMsg {
        #[structopt(parse(from_os_str))]
        /// Input filename of USP protobuf record to decode
        in_file: PathBuf,
        /// Output filename of USP protobuf message to write into
        #[structopt(parse(from_os_str))]
        out_file: PathBuf,
    },
    /// Extract the USP message body from an USP record
    #[structopt(name = "extract_msg_body")]
    ExtractMsgBody {
        #[structopt(parse(from_os_str))]
        /// Input filename of USP protobuf record to decode
        in_file: PathBuf,
        /// Output filename of USP protobuf message body to write into
        #[structopt(parse(from_os_str))]
        out_file: PathBuf,
    },
    /// Wrap msg from stdin into a single raw USP record
    #[structopt(name = "wrap_msg_raw")]
    WrapMsgRaw {
        #[structopt(long = "version")]
        /// USP specification version
        version: Option<String>,
        #[structopt(long = "from")]
        /// Sender Id
        from: Option<String>,
        #[structopt(long = "to")]
        /// Recipient Id
        to: Option<String>,
        /// Filename (will output to standard output if omitted)
        #[structopt(parse(from_os_str), short = "f", long = "file")]
        /// Output filename of file to encode USP protobuf record to
        filename: Option<PathBuf>,
    },
}

#[derive(StructOpt, Debug)]
enum MsgType {
    /// Generate an USP Error message
    Error {
        /// The USP error code (MUST be between 7000 and 7999)
        code: u32,
        /// An (optional) error message. Standard error messages will be computed from the error
        /// code if not provided
        message: Option<String>,
    },
    /// Generate an USP Get request message
    Get {
        /// A JSON array of Strings resembling the paths for the Get operation
        #[structopt(multiple = true)]
        paths: Vec<String>,
    },
    /// Generate an USP GetResp response message
    GetResp {
        /// A JSON array of Strings resembling the result data for the GetResp operation
        #[structopt(multiple = true)]
        result: Vec<String>,
    },
    /// Generate an USP Notify "request" message
    Notify {
        /// Subscription ID
        sub_id: String,
        /// Do we expect a resonse?
        send_resp: bool,
        /// Type of notification
        #[structopt(subcommand)]
        typ: NotifyType,
    },
    /// Generate an USP Notify "response" message
    NotifyResp {
        /// Subscription ID
        sub_id: String,
    },
}

fn decode_msg_files(files: Vec<PathBuf>) {
    for file in files {
        let fp = File::open(&file).unwrap_or_else(|_| panic!("Could not open file {:?}", file));
        let mut buf_reader = BufReader::new(fp);
        let mut contents = Vec::new();
        buf_reader
            .read_to_end(&mut contents)
            .unwrap_or_else(|_| panic!("Could not read from file {:?}", file));

        println!("{}", decode_msg(&contents));
    }
}

fn decode_msg_stdin() {
    let mut contents = Vec::new();
    stdin()
        .read_to_end(&mut contents)
        .expect("Couldn't read USP Msg from stdin");

    println!("{}", decode_msg(&contents));
}

fn decode_record_files(files: Vec<PathBuf>) {
    for file in files {
        let fp = File::open(&file).unwrap_or_else(|_| panic!("Could not open file {:?}", file));
        let mut buf_reader = BufReader::new(fp);
        let mut contents = Vec::new();
        buf_reader
            .read_to_end(&mut contents)
            .unwrap_or_else(|_| panic!("Could not read from file {:?}", file));

        println!("{}", decode_record(&contents));
    }
}

fn decode_record_stdin() {
    let mut contents = Vec::new();
    stdin()
        .read_to_end(&mut contents)
        .expect("Couldn't read USP Record from stdin");

    println!("{}", decode_record(&contents));
}

fn encode_msg_body_buf(typ: MsgType) -> Vec<u8> {
    use quick_protobuf::serialize_into_vec;

    match typ {
        MsgType::Error { code, message } => {
            serialize_into_vec(&usp_generator::usp_simple_error(code, message))
        }
        MsgType::Get { paths } => {
            let paths = paths.join(" ");
            let v: Vec<&str> = serde_json::from_str(&paths).unwrap();
            serialize_into_vec(&usp_generator::usp_get_request(v.as_slice()))
        }
        MsgType::GetResp { result } => {
            let result = result.join(" ");
            let getresp_json: usp_generator::GetResp = serde_json::from_str(&result).unwrap();
            serialize_into_vec(&usp_generator::usp_get_response_from_json(&getresp_json))
        }
        MsgType::Notify {
            sub_id,
            send_resp,
            typ,
        } => serialize_into_vec(&usp_generator::usp_notify_request(&sub_id, send_resp, typ)),
        MsgType::NotifyResp { sub_id } => {
            serialize_into_vec(&usp_generator::usp_notify_response(&sub_id))
        }
    }
    .expect("Cannot encode message")
}

fn encode_msg_body(filename: Option<PathBuf>, typ: MsgType) {
    use quick_protobuf::{deserialize_from_slice, message::MessageWrite, Writer};

    let mut buf = Vec::new();
    let mut writer = Writer::new(&mut buf);

    let encoded_body = encode_msg_body_buf(typ);
    let body: rusp::usp::Body = deserialize_from_slice(&encoded_body).unwrap();
    body.write_message(&mut writer)
        .expect("Failed encoding USP Msg");
    if filename.is_some() {
        std::fs::write(filename.unwrap(), buf).unwrap();
    } else {
        stdout().write_all(&buf).unwrap();
    }
}

fn encode_msg(msgid: String, filename: Option<PathBuf>, typ: MsgType) {
    use quick_protobuf::{deserialize_from_slice, message::MessageWrite, Writer};

    let mut buf = Vec::new();
    let mut writer = Writer::new(&mut buf);

    let encoded_body = encode_msg_body_buf(typ);
    let body: rusp::usp::Body = deserialize_from_slice(&encoded_body).unwrap();
    usp_generator::usp_msg(msgid, body)
        .write_message(&mut writer)
        .expect("Failed encoding USP Msg");

    if filename.is_some() {
        std::fs::write(filename.unwrap(), buf).unwrap();
    } else {
        stdout().write_all(&buf).unwrap();
    }
}

fn extract_msg(in_file: &PathBuf, out_file: &PathBuf) {
    use rusp::usp_record::mod_Record::OneOfrecord_type::*;

    let fp = File::open(&in_file).unwrap_or_else(|_| panic!("Could not open file {:?}", in_file));
    let mut buf_reader = BufReader::new(fp);
    let mut contents = Vec::new();
    buf_reader
        .read_to_end(&mut contents)
        .unwrap_or_else(|_| panic!("Could not read from file {:?}", in_file));

    let record = decode_record(&contents);

    match record.record_type {
        no_session_context(context) => {
            let msg = context.payload.unwrap();
            std::fs::write(&out_file, &msg).unwrap();
        }
        session_context(_) => unreachable!(),
        None => unreachable!(),
    }
}

fn extract_msg_body(in_file: &PathBuf, out_file: &PathBuf) {
    use quick_protobuf::{message::MessageWrite, Writer};
    use rusp::usp_record::mod_Record::OneOfrecord_type::*;

    let fp = File::open(&in_file).unwrap_or_else(|_| panic!("Could not open file {:?}", in_file));
    let mut buf_reader = BufReader::new(fp);
    let mut contents = Vec::new();
    buf_reader
        .read_to_end(&mut contents)
        .unwrap_or_else(|_| panic!("Could not read from file {:?}", in_file));

    let record = decode_record(&contents);

    match record.record_type {
        no_session_context(context) => {
            let mut buf = Vec::new();
            let mut writer = Writer::new(&mut buf);

            let payload = context.payload.unwrap();
            let msg = decode_msg(&payload);
            let body = msg.body.unwrap();
            body.write_message(&mut writer)
                .expect("Cannot encode message");
            std::fs::write(&out_file, buf).unwrap();
        }
        session_context(_) => unreachable!(),
        None => unreachable!(),
    }
}

fn wrap_msg_raw(
    version: Option<String>,
    from: Option<String>,
    to: Option<String>,
    filename: Option<PathBuf>,
) {
    use quick_protobuf::{message::MessageWrite, Writer};

    let mut msg = Vec::new();
    stdin()
        .read_to_end(&mut msg)
        .expect("Couldn't read USP Msg from stdin");

    let mut buf = Vec::new();
    let mut writer = Writer::new(&mut buf);

    usp_generator::usp_no_session_context_record(
        version.as_ref().map(String::as_str),
        from.as_ref().map(String::as_str),
        to.as_ref().map(String::as_str),
        &msg,
    )
    .write_message(&mut writer)
    .expect("Failed encoding USP Record");

    if filename.is_some() {
        std::fs::write(filename.unwrap(), buf).unwrap();
    } else {
        stdout().write_all(&buf).unwrap();
    }
}

fn main() {
    color_backtrace::install();

    let opt = Rusp::from_args();

    match opt {
        Rusp::DecodeRecordFiles { files } => decode_record_files(files),
        Rusp::DecodeRecord {} => decode_record_stdin(),
        Rusp::DecodeMsgFiles { files } => decode_msg_files(files),
        Rusp::DecodeMsg {} => decode_msg_stdin(),
        Rusp::EncodeMsgBody { filename, typ } => encode_msg_body(filename, typ),
        Rusp::EncodeMsg {
            msgid,
            filename,
            typ,
        } => encode_msg(msgid, filename, typ),
        Rusp::ExtractMsg { in_file, out_file } => extract_msg(&in_file, &out_file),
        Rusp::ExtractMsgBody { in_file, out_file } => extract_msg_body(&in_file, &out_file),
        Rusp::WrapMsgRaw {
            version,
            from,
            to,
            filename,
        } => wrap_msg_raw(version, from, to, filename),
    }
}
