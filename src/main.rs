use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::{stdin, stdout, BufReader, Read, Write};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use rusp::{
    usp_decoder::{try_decode_msg, try_decode_record},
    usp_generator,
    usp_types::{NotifyType, PayloadSecurity},
};

#[derive(PartialEq)]
/// The supported output formats
enum OutputFormat {
    /// Our custom text representation
    Native,
    /// Valid JSON format
    Json,
    /// Protobuf output as C strings or Rust byarrays where non-ascii characters are replaced with
    /// backslashed escaped hex codes
    CStr,
    /// Protobuf output as C array with preview comments for inclusion in source code
    CArray,
    /// Naktive Protobuf binary output
    Protobuf,
}

#[derive(Parser)]
#[clap(author, version, name = "rusp", about = "the Rust USP toolkit")]
struct Rusp {
    #[clap(
        long = "carray",
        conflicts_with = "cstr",
        conflicts_with = "json",
        conflicts_with = "protobuf"
    )]
    /// Output as C array (and length) for inclusion in source code
    carray: bool,
    #[clap(
        long = "json",
        conflicts_with = "cstr",
        conflicts_with = "carray",
        conflicts_with = "protobuf"
    )]
    /// Output as JSON
    json: bool,
    #[clap(
        long = "cstr",
        conflicts_with = "json",
        conflicts_with = "carray",
        conflicts_with = "protobuf"
    )]
    /// Output binary as Protobuf in a C string / Rust bytearray representation
    cstr: bool,
    #[clap(
        long = "protobuf",
        conflicts_with = "json",
        conflicts_with = "carray",
        conflicts_with = "cstr"
    )]
    /// Output binary as native Protobuf binary
    protobuf: bool,
    #[clap(subcommand)]
    action: RuspAction,
}

#[derive(Subcommand)]
enum RuspAction {
    /// Decode a single raw USP message from standard input and print to standard output
    #[clap(name = "decode_msg")]
    DecodeMsg {},
    /// Decode ore or more USP messages from specified filenames and print to standard output
    #[clap(name = "decode_msg_files")]
    DecodeMsgFiles {
        #[clap(required = true)]
        /// Filenames of USP protobuf messages to decode
        files: Vec<PathBuf>,
    },
    /// Decode a single raw USP record from standard input and print to standard output
    #[clap(name = "decode_record")]
    DecodeRecord {},
    /// Decode one or more USP records from specified filenames and print to standard output
    #[clap(name = "decode_record_files")]
    DecodeRecordFiles {
        #[clap(required = true)]
        /// Filenames of USP protobuf records to decode
        files: Vec<PathBuf>,
    },
    /// Encode command line input into a single raw USP message
    #[clap(name = "encode_msg")]
    EncodeMsg {
        /// Output the serialised protobuf as C char array
        #[clap(short = 'c')]
        as_c_array: bool,
        /// The message ID to use in the USP Msg header
        msgid: String,
        /// Filename (will output to standard output if omitted)
        #[clap(short = 'f', long = "file")]
        /// Output filename of file to encode USP protobuf message to
        filename: Option<PathBuf>,
        /// Type of message
        #[clap(subcommand)]
        typ: MsgType,
    },
    /// Encode command line input into a single raw USP message body
    #[clap(name = "encode_msg_body")]
    EncodeMsgBody {
        /// Output the serialised protobuf as C char array
        #[clap(short = 'c')]
        as_c_array: bool,
        /// Filename (will output to standard output if omitted)
        #[clap(short = 'f', long = "file")]
        /// Output filename of file to encode USP protobuf message to
        filename: Option<PathBuf>,
        /// Type of message
        #[clap(subcommand)]
        typ: MsgType,
    },
    /// Extract the USP message from an USP record
    #[clap(name = "extract_msg")]
    ExtractMsg {
        /// Input filename of USP protobuf record to decode
        in_file: PathBuf,
        /// Output filename of USP protobuf message to write into, use `-` for stdout
        out_file: PathBuf,
    },
    /// Extract the USP message body from an USP record
    #[clap(name = "extract_msg_body")]
    ExtractMsgBody {
        /// Input filename of USP protobuf record to decode
        in_file: PathBuf,
        /// Output filename of USP protobuf message body to write into, use `-` for stdout
        out_file: PathBuf,
    },
    /// Wrap msg from stdin into a single no-session context USP record (this option is deprecated
    /// and will be removed in a future version, use `encode_no_session_record` instead)
    #[clap(name = "wrap_msg_raw")]
    WrapMsgRaw {
        /// Output the serialised protobuf as C char array
        #[clap(short = 'c')]
        as_c_array: bool,
        #[clap(long = "version", default_value = "1.1")]
        /// USP specification version
        version: String,
        #[clap(long = "from", default_value = "doc::from")]
        /// Sender Id
        from: String,
        #[clap(long = "to", default_value = "doc::to")]
        /// Recipient Id
        to: String,
        /// Filename (will output to standard output if omitted)
        #[clap(short = 'f', long = "file")]
        /// Output filename of file to encode USP protobuf record to
        filename: Option<PathBuf>,
    },
    /// Encode Msg payload provided via stdin into a single no-session context USP Record
    #[clap(name = "encode_no_session_record")]
    EncodeNoSessionRecord {
        #[clap(long = "version", default_value = "1.1")]
        /// USP specification version
        version: String,
        #[clap(long = "from", default_value = "doc::from")]
        /// Sender Id
        from: String,
        #[clap(long = "to", default_value = "doc::to")]
        /// Recipient Id
        to: String,
        /// Filename (will output to standard output if omitted)
        #[clap(short = 'f', long = "file")]
        /// Output filename of file to encode USP protobuf record to
        filename: Option<PathBuf>,
    },
    /// Encode Msg payload provided via stdin into a single session context USP Record
    #[clap(name = "encode_session_record")]
    EncodeSessionRecord {
        #[clap(long = "version", default_value = "1.2")]
        /// USP specification version
        version: String,
        #[clap(long = "from", default_value = "doc::from")]
        /// Sender Id
        from: String,
        #[clap(long = "to", default_value = "doc::to")]
        /// Recipient Id
        to: String,
        #[clap(long = "session_id", default_value = "1234")]
        /// The ID of the context session
        session_id: u64,
        #[clap(long = "sequence_id", default_value = "1")]
        /// The sequence number within the context session
        sequence_id: u64,
        #[clap(long = "expected_id", default_value = "2")]
        /// The expected next sequence number within the context session
        expected_id: u64,
        #[clap(long = "retransmit_id", default_value = "0")]
        /// The sequence number of the part which is being retransmitted
        retransmit_id: u64,
        /// Filename (will output to standard output if omitted)
        #[clap(short = 'f', long = "file")]
        /// Output filename of file to encode USP protobuf record to
        filename: Option<PathBuf>,
    },
}

#[derive(Parser, Debug)]
#[clap(rename_all = "verbatim")]
enum MsgType {
    /// Generate an USP Add request message
    #[clap(name = "Add")]
    USPAdd {
        /// Do we allow partial execution?
        allow_partial: bool,
        /// A JSON structure resesembling the input for a Add operation
        ///
        /// Example use: '[["Device.DeviceInfo.", [["ProvisioningCode", "configured", true]]]]'
        #[clap(num_args(1..))]
        args: Vec<String>,
    },
    /// Generate an USP Delete request message
    #[clap(name = "Delete")]
    USPDelete {
        /// Do we allow partial execution?
        allow_partial: bool,
        /// A JSON structure resesembling the input for a Delete operation
        ///
        /// Example use: '["Device.XMPP.Connection.1.", "Device.LocalAgent.Subscription.3."]'
        #[clap(num_args(1..))]
        obj_paths: Vec<String>,
    },
    /// Generate an USP Error message
    #[clap(name = "Error")]
    USPError {
        /// The USP error code (MUST be between 7000 and 7999)
        code: u32,
        /// An (optional) error message. Standard error messages will be computed from the error
        /// code if not provided
        message: Option<String>,
    },
    /// Generate an USP Get request message
    #[clap(name = "Get")]
    USPGet {
        /// A JSON array of Strings resembling the paths for the Get operation
        #[clap(num_args(1..))]
        paths: Vec<String>,
        #[clap(long = "max_depth")]
        max_depth: Option<u32>,
    },
    /// Generate an USP GetResp response message
    #[clap(name = "GetResp")]
    USPGetResp {
        /// A JSON array of Strings resembling the result data for the GetResp operation
        #[clap(num_args(1..))]
        result: Vec<String>,
    },
    /// Generate an USP GetInstances request message
    #[clap(name = "GetInstances")]
    USPGetInstances {
        /// Only return the first level of recursive structures?
        first_level_only: bool,
        /// A JSON array ressembling the object paths we're interested in
        ///
        /// Example use: '["Device.DeviceInfo.", "Device.LocalAgent."]'
        #[clap(num_args(1..))]
        obj_paths: Vec<String>,
    },
    /// Generate an USP GetSupportedDM request message
    #[clap(name = "GetSupportedDM")]
    USPGetSupportedDM {
        /// Only return the first level of recursive structures?
        first_level_only: bool,
        /// Return commands?
        return_commands: bool,
        /// Return events?
        return_events: bool,
        /// Return parameters?
        return_params: bool,
        /// A JSON array ressembling the paths we're interested in
        ///
        /// Example use: '["Device.DeviceInfo.", "Device.LocalAgent."]'
        #[clap(num_args(1..))]
        paths: Vec<String>,
    },
    /// Generate an USP GetSupportedProtocol request message
    #[clap(name = "GetSupportedProtocol")]
    USPGetSupportedProtocol {
        /// Controller Supported Protocol Version
        cspv: String,
    },
    /// Generate an USP Notify request message
    #[clap(name = "Notify")]
    USPNotify {
        /// Subscription ID
        sub_id: String,
        /// Do we expect a response?
        send_resp: bool,
        /// Type of notification
        #[clap(subcommand)]
        typ: NotifyType,
    },
    /// Generate an USP Notify response message
    #[clap(name = "NotifyResp")]
    USPNotifyResp {
        /// Subscription ID
        sub_id: String,
    },
    /// Generate an USP Operate request message
    #[clap(name = "Operate")]
    USPOperate {
        /// The full pathname of of the command to execute
        command: String,
        /// The command key to use in the request to allow later matching with a result
        command_key: String,
        /// A boolean indicating whether a response is expected in reply to this request
        send_resp: bool,
        /// A JSON array of arrays containing the command input arguments with path names and values
        #[clap(num_args(1..))]
        args: Vec<String>,
    },
    /// Generate an USP Set request message
    #[clap(name = "Set")]
    USPSet {
        /// Do we allow partial execution?
        allow_partial: bool,
        /// A JSON structure resesembling the input for a Set operation
        ///
        /// Example use: '[["Device.DeviceInfo.", [["ProvisioningCode", "configured", true]]]]'
        #[clap(num_args(1..))]
        args: Vec<String>,
    },
}

fn decode_msg_files(files: Vec<PathBuf>, format: OutputFormat) -> Result<()> {
    for file in files {
        let fp = File::open(&file)?;
        let mut buf_reader = BufReader::new(fp);
        let mut contents = Vec::new();
        buf_reader.read_to_end(&mut contents)?;

        // Try to parse bytes as a protobuf encoded USP Message
        let decoded = try_decode_msg(&contents)?;

        // Open stdout as output stream and write the USP Msg to it
        write_msg(decoded, get_out_stream(None)?, &format)?;
    }

    Ok(())
}

fn decode_msg_stdin(format: OutputFormat) -> Result<()> {
    let mut contents = Vec::new();
    stdin().read_to_end(&mut contents)?;

    // Try to parse bytes as a protobuf encoded USP Message
    let decoded = try_decode_msg(&contents)?;

    // Open stdout as output stream and write the USP Msg to it
    write_msg(decoded, get_out_stream(None)?, &format)
}

fn decode_record_files(files: Vec<PathBuf>, format: OutputFormat) -> Result<()> {
    for file in files {
        let fp = File::open(&file)?;
        let mut buf_reader = BufReader::new(fp);
        let mut contents = Vec::new();
        buf_reader.read_to_end(&mut contents)?;

        // Try to parse bytes as a protobuf encoded USP Record
        let decoded = try_decode_record(&contents)?;

        // Open stdout as output stream and write the USP Record to it
        write_record(decoded, get_out_stream(None)?, &format)?;
    }

    Ok(())
}

fn decode_record_stdin(format: OutputFormat) -> Result<()> {
    let mut contents = Vec::new();
    stdin().read_to_end(&mut contents)?;

    // Try to parse bytes as a protobuf encoded USP Record
    let decoded = try_decode_record(&contents)?;

    // Open stdout as output stream and write the USP Record to it
    write_record(decoded, get_out_stream(None)?, &format)
}

fn encode_msg_body_buf(typ: MsgType) -> Result<Vec<u8>> {
    use quick_protobuf::serialize_into_vec;

    match typ {
        MsgType::USPAdd {
            allow_partial,
            args,
        } => {
            let args = args.join(" ");
            let v = serde_json::from_str::<Vec<(&str, Vec<(&str, &str, bool)>)>>(&args)
                .with_context(|| format!("Expected JSON data in the form \"[[<Object path>, [[<Parameter name>, <Parameter value>, <Required>], ...]], ...]\", got '{}'", args))?;
            serialize_into_vec(&usp_generator::usp_add_request(
                allow_partial,
                v.iter()
                    .map(|(path, par)| (*path, par.as_slice()))
                    .collect::<Vec<_>>()
                    .as_slice(),
            ))
        }
        MsgType::USPDelete {
            allow_partial,
            obj_paths,
        } => {
            let obj_paths = obj_paths.join(" ");
            let obj_paths = serde_json::from_str::<Vec<&str>>(&obj_paths)
                .with_context(|| format!("Expected JSON data in the form \"[<Object instance path>, ...]\", got '{}'", obj_paths))?;
            serialize_into_vec(&usp_generator::usp_delete_request(
                allow_partial,
                &obj_paths,
            ))
        }
        MsgType::USPError { code, message } => {
            serialize_into_vec(&usp_generator::usp_simple_error(code, message.as_deref()))
        }
        MsgType::USPGet { paths, max_depth } => {
            let paths = paths.join(" ");
            let v = serde_json::from_str::<Vec<&str>>(&paths)
                .with_context(|| format!("Expected JSON data in the form \"[<Path name>, ...]\",  got '{}'", paths))?;
            serialize_into_vec(&usp_generator::usp_get_request(v.as_slice(), max_depth.unwrap_or(0)))
        }
        MsgType::USPGetInstances {
            first_level_only,
            obj_paths,
        } => {
            let obj_paths = obj_paths.join(" ");
            let v = serde_json::from_str::<Vec<&str>>(&obj_paths)
                .with_context(|| format!("Expected JSON data in the form \"[<Object path>, ...]\",  got '{}'", obj_paths))?;
            serialize_into_vec(&usp_generator::usp_get_instances_request(
                v.as_slice(),
                first_level_only,
            ))
        }
        MsgType::USPGetSupportedDM {
            first_level_only,
            return_commands,
            return_events,
            return_params,
            paths,
        } => {
            let paths = paths.join(" ");
            let v = serde_json::from_str::<Vec<&str>>(&paths)
                .with_context(|| format!("Expected JSON data in the form \"[<Object path>, ...]\",  got '{}'", paths))?;
            serialize_into_vec(&usp_generator::usp_get_supported_dm_request(
                v.as_slice(),
                first_level_only,
                return_commands,
                return_events,
                return_params,
            ))
        }
        MsgType::USPGetSupportedProtocol { cspv } => {
            serialize_into_vec(&usp_generator::usp_get_supported_prototol_request(&cspv))
        }
        MsgType::USPGetResp { result } => {
            let result = result.join(" ");
            let getresp_json: usp_generator::GetResp = serde_json::from_str(&result)?;
            serialize_into_vec(&usp_generator::usp_get_response_from_json(&getresp_json))
        }
        MsgType::USPNotify {
            sub_id,
            send_resp,
            typ,
        } => serialize_into_vec(&usp_generator::usp_notify_request(&sub_id, send_resp, &typ)),
        MsgType::USPNotifyResp { sub_id } => {
            serialize_into_vec(&usp_generator::usp_notify_response(&sub_id))
        }
        MsgType::USPOperate {
            command,
            command_key,
            send_resp,
            args,
        } => {
            let args = args.join(" ");
            let v = if !args.is_empty() {
                serde_json::from_str::<Vec<(&str, &str)>>(&args)
                .with_context(|| format!("Expected JSON data in the form \"[[<Argument name>, <Argument value>], ...]\",  got '{}'", args))?
            } else {
                Vec::new()
            };
            serialize_into_vec(&usp_generator::usp_operate_request(
                &command,
                &command_key,
                send_resp,
                v.into_iter().collect::<Vec<_>>().as_slice(),
            ))
        }
        MsgType::USPSet {
            allow_partial,
            args,
        } => {
            let args = args.join(" ");
            let v = serde_json::from_str::<Vec<(&str, Vec<(&str, &str, bool)>)>>(&args)
                .with_context(|| format!("Expected JSON data in the form \"[[<Object path>, [[<Parameter name>, <Parameter value>, <Required>], ...]], ...]\",  got '{}'", args))?;
            serialize_into_vec(&usp_generator::usp_set_request(
                allow_partial,
                v.iter()
                    .map(|(path, par)| (*path, par.as_slice()))
                    .collect::<Vec<_>>()
                    .as_slice(),
            ))
        }
    }.context("While trying to encode message to ProtoBuf")
}

fn get_out_stream(filename: Option<PathBuf>) -> Result<Box<dyn Write>> {
    Ok(if let Some(filename) = filename {
        Box::new(File::create(filename)?)
    } else {
        Box::new(stdout())
    })
}

fn write_c_array(mut out: Box<dyn Write>, buf: &[u8]) -> Result<()> {
    fn check_printable(c: u8) -> bool {
        match c as char {
            ' ' | '.' | '!' | '(' | ')' | '\'' | '"' | ',' | '*' | '[' | ']' | '=' | '<' | '>'
            | '-' | '_' => true,
            _ if c.is_ascii_alphanumeric() => true,
            _ => false,
        }
    }

    const CHUNK_LEN: usize = 8;
    writeln!(out, "unsigned int pb_len = {};", buf.len())?;
    writeln!(out, "const char pb[] = {{")?;
    for chunk in buf.chunks(CHUNK_LEN) {
        write!(out, "  ")?;
        for i in chunk {
            write!(out, "0x{:02x}, ", i)?;
        }

        for _ in chunk.len()..CHUNK_LEN {
            write!(out, "      ")?;
        }

        write!(out, "/* ")?;
        for i in chunk {
            if check_printable(*i) {
                write!(out, "{}", char::from(*i))?;
            } else {
                write!(out, "_")?;
            }
        }
        write!(out, " */")?;

        writeln!(out)?;
    }
    writeln!(out, "}};")?;

    Ok(())
}

fn write_c_str(mut out: Box<dyn Write>, buf: &[u8]) -> Result<()> {
    fn check_printable(c: u8) -> bool {
        match c as char {
            ' ' | '.' | '!' | '(' | ')' | '\'' | ',' | '*' | '[' | ']' | '=' | '<' | '>' | '-'
            | '_' => true,
            _ if c.is_ascii_alphanumeric() => true,
            _ => false,
        }
    }

    write!(out, "\"")?;
    for i in buf {
        if check_printable(*i) {
            write!(out, "{}", char::from(*i))?;
        } else {
            write!(out, "\\x{:02x}", i)?;
        }
    }

    writeln!(out, "\"")?;

    Ok(())
}

/// Write the given USP Msg to the output stream in the specified format
fn write_msg(msg: rusp::usp::Msg, mut out: Box<dyn Write>, format: &OutputFormat) -> Result<()> {
    use quick_protobuf::{message::MessageWrite, Writer};

    let mut buf = Vec::new();
    let mut writer = Writer::new(&mut buf);
    msg.write_message(&mut writer)
        .context("Failed encoding USP Msg")?;

    match format {
        OutputFormat::Json => {
            writeln!(
                out,
                "{}",
                serde_json::to_string_pretty(&msg).context("Failed to serialize JSON")?
            )?;
        }
        OutputFormat::Native => {
            writeln!(out, "{}", &msg)?;
        }
        OutputFormat::CStr => {
            write_c_str(out, buf.as_slice())?;
        }
        OutputFormat::CArray => {
            write_c_array(out, buf.as_slice())?;
        }
        OutputFormat::Protobuf => {
            out.write_all(buf.as_slice())?;
        }
    }

    Ok(())
}

/// Write the given USP Record to the output stream in the specified format
fn write_record(
    record: rusp::usp_record::Record,
    mut out: Box<dyn Write>,
    format: &OutputFormat,
) -> Result<()> {
    use quick_protobuf::{message::MessageWrite, Writer};

    let mut buf = Vec::new();
    let mut writer = Writer::new(&mut buf);
    record
        .write_message(&mut writer)
        .context("Failed encoding USP Record")?;

    match format {
        OutputFormat::Json => {
            writeln!(
                out,
                "{}",
                serde_json::to_string_pretty(&record).context("Failed to serialize JSON")?
            )?;
        }
        OutputFormat::Native => {
            writeln!(out, "{}", &record)?;
        }
        OutputFormat::CStr => {
            write_c_str(out, buf.as_slice())?;
        }
        OutputFormat::CArray => {
            write_c_array(out, buf.as_slice())?;
        }
        OutputFormat::Protobuf => {
            out.write_all(buf.as_slice())?;
        }
    }

    Ok(())
}

/// Write the given USP Msg Bodyto the output stream in the specified format
fn write_body(msg: rusp::usp::Body, mut out: Box<dyn Write>, format: &OutputFormat) -> Result<()> {
    use quick_protobuf::{message::MessageWrite, Writer};

    let mut buf = Vec::new();
    let mut writer = Writer::new(&mut buf);
    msg.write_message(&mut writer)
        .context("Failed encoding USP Msg Body")?;

    match format {
        OutputFormat::Json => {
            writeln!(
                out,
                "{}",
                serde_json::to_string_pretty(&msg).context("Failed to serialize JSON")?
            )?;
        }
        OutputFormat::Native => {
            writeln!(out, "{}", &msg)?;
        }
        OutputFormat::CStr => {
            write_c_str(out, buf.as_slice())?;
        }
        OutputFormat::CArray => {
            write_c_array(out, buf.as_slice())?;
        }
        OutputFormat::Protobuf => {
            out.write_all(buf.as_slice())?;
        }
    }

    Ok(())
}

fn encode_msg_body(filename: Option<PathBuf>, typ: MsgType, format: OutputFormat) -> Result<()> {
    use quick_protobuf::{deserialize_from_slice, message::MessageWrite, Writer};

    let mut buf = Vec::new();
    let mut writer = Writer::new(&mut buf);

    let encoded_body = encode_msg_body_buf(typ)?;
    let body: rusp::usp::Body =
        deserialize_from_slice(&encoded_body).context("Failed trying to deserialise Msg body")?;

    body.write_message(&mut writer)
        .context("Failed encoding USP Msg Body")?;

    // Open output stream
    let mut out = get_out_stream(filename)?;

    match format {
        OutputFormat::Json => {
            writeln!(
                out,
                "{}",
                serde_json::to_string_pretty(&body).context("Failed to serialize JSON")?
            )?;
        }
        OutputFormat::Native => {
            writeln!(out, "{}", &body)?;
        }
        OutputFormat::CStr => {
            write_c_str(out, buf.as_slice())?;
        }
        OutputFormat::CArray => {
            write_c_array(out, buf.as_slice())?;
        }
        OutputFormat::Protobuf => {
            out.write_all(buf.as_slice())?;
        }
    }

    Ok(())
}

fn encode_msg(
    msgid: String,
    filename: Option<PathBuf>,
    typ: MsgType,
    format: OutputFormat,
) -> Result<()> {
    use quick_protobuf::deserialize_from_slice;

    let encoded_body = encode_msg_body_buf(typ)?;
    let body: rusp::usp::Body =
        deserialize_from_slice(&encoded_body).context("Failed trying to deserialise Msg body")?;
    let msg = usp_generator::usp_msg(msgid, body);

    // Open the specified file (or stdout) as output stream and write the USP Msg to it
    write_msg(msg, get_out_stream(filename)?, &format)
}

fn extract_msg(in_file: &Path, out_file: &Path, format: OutputFormat) -> Result<()> {
    use rusp::usp_record::mod_Record::OneOfrecord_type;

    let fp = File::open(&in_file)?;
    let mut buf_reader = BufReader::new(fp);
    let mut contents = Vec::new();
    buf_reader.read_to_end(&mut contents)?;

    let record = try_decode_record(&contents)?;

    match record.record_type {
        OneOfrecord_type::no_session_context(context) => {
            let msg = try_decode_msg(&context.payload)?;
            let out = if let Some("-") = out_file.to_str() {
                get_out_stream(None)?
            } else {
                get_out_stream(Some(out_file.to_path_buf()))?
            };
            let format = if format == OutputFormat::Native {
                OutputFormat::Protobuf
            } else {
                format
            };
            write_msg(msg, out, &format)?;
        }
        OneOfrecord_type::session_context(_) => unreachable!(),
        OneOfrecord_type::websocket_connect(_) => unimplemented!(),
        OneOfrecord_type::mqtt_connect(_) => unimplemented!(),
        OneOfrecord_type::stomp_connect(_) => unimplemented!(),
        OneOfrecord_type::disconnect(_) => unimplemented!(),
        OneOfrecord_type::None => unreachable!(),
    }

    Ok(())
}

fn extract_msg_body(in_file: &Path, out_file: &Path, format: OutputFormat) -> Result<()> {
    use rusp::usp_record::mod_Record::OneOfrecord_type;

    let fp = File::open(&in_file)?;
    let mut buf_reader = BufReader::new(fp);
    let mut contents = Vec::new();
    buf_reader.read_to_end(&mut contents)?;

    let record = try_decode_record(&contents)?;

    match record.record_type {
        OneOfrecord_type::no_session_context(context) => {
            let msg = try_decode_msg(&context.payload)?;
            let body = msg.body.context("Failed extracting USP Msg body")?;

            let out = if let Some("-") = out_file.to_str() {
                get_out_stream(None)?
            } else {
                get_out_stream(Some(out_file.to_path_buf()))?
            };

            let format = if format == OutputFormat::Native {
                OutputFormat::Protobuf
            } else {
                format
            };

            write_body(body, out, &format)?;
        }
        OneOfrecord_type::session_context(_) => unreachable!(),
        OneOfrecord_type::websocket_connect(_) => unimplemented!(),
        OneOfrecord_type::mqtt_connect(_) => unimplemented!(),
        OneOfrecord_type::stomp_connect(_) => unimplemented!(),
        OneOfrecord_type::disconnect(_) => unimplemented!(),
        OneOfrecord_type::None => unreachable!(),
    }

    Ok(())
}

fn encode_no_session_record(
    version: String,
    from: String,
    to: String,
    filename: Option<PathBuf>,
    format: OutputFormat,
) -> Result<()> {
    let mut msg = Vec::new();
    stdin().read_to_end(&mut msg)?;

    let record = usp_generator::usp_no_session_context_record(
        &version,
        &to,
        &from,
        PayloadSecurity::PLAINTEXT,
        &[],
        &[],
        &msg,
    );

    // Open output stream
    let out = get_out_stream(filename)?;

    let format = if format == OutputFormat::Native {
        OutputFormat::Protobuf
    } else {
        format
    };

    write_record(record, out, &format)
}

fn encode_session_record(
    version: String,
    from: String,
    to: String,
    session_id: u64,
    sequence_id: u64,
    expected_id: u64,
    retransmit_id: u64,
    filename: Option<PathBuf>,
    format: OutputFormat,
) -> Result<()> {
    let mut msg = Vec::new();
    stdin().read_to_end(&mut msg)?;

    let record = usp_generator::usp_session_context_record(
        &version,
        &to,
        &from,
        PayloadSecurity::PLAINTEXT,
        &[],
        &[],
        session_id,
        sequence_id,
        expected_id,
        retransmit_id,
        usp_generator::PayloadSARState::NONE,
        usp_generator::PayloadSARState::NONE,
        &msg,
    );

    // Open output stream
    let out = get_out_stream(filename)?;

    let format = if format == OutputFormat::Native {
        OutputFormat::Protobuf
    } else {
        format
    };

    write_record(record, out, &format)
}

fn main() -> Result<()> {
    let Rusp {
        action,
        json,
        cstr,
        carray,
        protobuf,
    } = Rusp::parse();

    // Pass on the user chosen format to use for the output
    let format = {
        if json {
            OutputFormat::Json
        } else if carray {
            OutputFormat::CArray
        } else if cstr {
            OutputFormat::CStr
        } else if protobuf {
            OutputFormat::Protobuf
        } else {
            OutputFormat::Native
        }
    };

    match action {
        RuspAction::DecodeRecordFiles { files } => decode_record_files(files, format),
        RuspAction::DecodeRecord {} => decode_record_stdin(format),
        RuspAction::DecodeMsgFiles { files } => decode_msg_files(files, format),
        RuspAction::DecodeMsg {} => decode_msg_stdin(format),
        RuspAction::EncodeMsgBody {
            filename,
            typ,
            as_c_array,
        } => {
            let format = if as_c_array {
                eprintln!("Warning: The '-c' option is deprecated and will be removed in a future version, use the global '--carray' option instead.");
                OutputFormat::CArray
            } else {
                format
            };
            encode_msg_body(filename, typ, format)
        }
        RuspAction::EncodeMsg {
            msgid,
            filename,
            typ,
            as_c_array,
        } => {
            let format = if as_c_array {
                eprintln!("Warning: The '-c' option is deprecated and will be removed in a future version, use the global '--carray' option instead.");
                OutputFormat::CArray
            } else {
                format
            };

            encode_msg(msgid, filename, typ, format)
        }
        RuspAction::ExtractMsg { in_file, out_file } => extract_msg(&in_file, &out_file, format),
        RuspAction::ExtractMsgBody { in_file, out_file } => {
            extract_msg_body(&in_file, &out_file, format)
        }
        RuspAction::WrapMsgRaw {
            version,
            from,
            to,
            filename,
            as_c_array,
        } => {
            let format = if as_c_array {
                eprintln!("Warning: The '-c' option is deprecated and will be removed in a future version, use the global '--carray' option instead.");
                OutputFormat::CArray
            } else {
                format
            };

            encode_no_session_record(version, from, to, filename, format)
        }
        RuspAction::EncodeNoSessionRecord {
            version,
            from,
            to,
            filename,
        } => encode_no_session_record(version, from, to, filename, format),
        RuspAction::EncodeSessionRecord {
            version,
            from,
            to,
            filename,
            session_id,
            sequence_id,
            expected_id,
            retransmit_id,
        } => encode_session_record(
            version,
            from,
            to,
            session_id,
            sequence_id,
            expected_id,
            retransmit_id,
            filename,
            format,
        ),
    }?;

    Ok(())
}
