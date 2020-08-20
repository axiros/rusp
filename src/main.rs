use std::fs::File;
use std::io::{stdin, stdout, BufReader, Read, Write};
use std::path::PathBuf;
use structopt::*;

use anyhow::{Context, Result};

use rusp::{
    usp_decoder::{try_decode_msg, try_decode_record},
    usp_generator,
    usp_types::NotifyType,
};

/// The supported output formats
enum OutputFormat {
    /// Our custom text representation
    Native,
    /// Valid JSON format
    JSON,
    /// Protobuf output as C strings or Rust byarrays where non-ascii characters are replaced with
    /// backslashed escaped hex codes
    CStr,
    /// Protobuf output as C array with preview comments for inclusion in source code
    CArray,
    /// Naktive Protobuf binary output
    Protobuf,
}

#[derive(StructOpt)]
#[structopt(name = "rusp", about = "the Rust USP toolkit")]
struct Rusp {
    #[structopt(
        long = "carray",
        conflicts_with = "cstr",
        conflicts_with = "json",
        conflicts_with = "protobuf"
    )]
    /// Output as C array (and length) for inclusion in source code
    carray: bool,
    #[structopt(
        long = "json",
        conflicts_with = "cstr",
        conflicts_with = "carray",
        conflicts_with = "protobuf"
    )]
    /// Output as JSON
    json: bool,
    #[structopt(
        long = "cstr",
        conflicts_with = "json",
        conflicts_with = "carray",
        conflicts_with = "protobuf"
    )]
    /// Output binary as Protobuf in a C string / Rust bytearray representation
    cstr: bool,
    #[structopt(
        long = "protobuf",
        conflicts_with = "json",
        conflicts_with = "carray",
        conflicts_with = "cstr"
    )]
    /// Output binary as native Protobuf binary
    protobuf: bool,
    #[structopt(flatten)]
    action: RuspAction,
}

#[derive(StructOpt)]
enum RuspAction {
    /// Decode a single raw USP message from standard input and print to standard output
    #[structopt(name = "decode_msg")]
    DecodeMsg {},
    /// Decode ore or more USP messages from specified filenames and print to standard output
    #[structopt(name = "decode_msg_files")]
    DecodeMsgFiles {
        #[structopt(parse(from_os_str), required = true)]
        /// Filenames of USP protobuf messages to decode
        files: Vec<PathBuf>,
    },
    /// Decode a single raw USP record from standard input and print to standard output
    #[structopt(name = "decode_record")]
    DecodeRecord {},
    /// Decode one or more USP records from specified filenames and print to standard output
    #[structopt(name = "decode_record_files")]
    DecodeRecordFiles {
        #[structopt(parse(from_os_str), required = true)]
        /// Filenames of USP protobuf records to decode
        files: Vec<PathBuf>,
    },
    /// Encode command line input into a single raw USP message
    #[structopt(name = "encode_msg")]
    EncodeMsg {
        /// Output the serialised protobuf as C char array
        #[structopt(short = "c")]
        as_c_array: bool,
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
        /// Output the serialised protobuf as C char array
        #[structopt(short = "c")]
        as_c_array: bool,
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
        /// Output the serialised protobuf as C char array
        #[structopt(short = "c")]
        as_c_array: bool,
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
#[structopt(rename_all = "verbatim")]
enum MsgType {
    /// Generate an USP Add request message
    #[structopt(name = "Add")]
    USPAdd {
        /// Do we allow partial execution?
        #[structopt(parse(try_from_str))]
        allow_partial: bool,
        /// A JSON structure resesembling the input for a Add operation
        ///
        /// Example use: '[["Device.DeviceInfo.", [["ProvisioningCode", "configured", true]]]]'
        #[structopt(multiple = true)]
        args: Vec<String>,
    },
    /// Generate an USP Delete request message
    #[structopt(name = "Delete")]
    USPDelete {
        /// Do we allow partial execution?
        #[structopt(parse(try_from_str))]
        allow_partial: bool,
        /// A JSON structure resesembling the input for a Delete operation
        ///
        /// Example use: '["Device.XMPP.Connection.1.", "Device.LocalAgent.Subscription.3."]'
        #[structopt(multiple = true)]
        obj_paths: Vec<String>,
    },
    /// Generate an USP Error message
    #[structopt(name = "Error")]
    USPError {
        /// The USP error code (MUST be between 7000 and 7999)
        code: u32,
        /// An (optional) error message. Standard error messages will be computed from the error
        /// code if not provided
        message: Option<String>,
    },
    /// Generate an USP Get request message
    #[structopt(name = "Get")]
    USPGet {
        /// A JSON array of Strings resembling the paths for the Get operation
        #[structopt(multiple = true)]
        paths: Vec<String>,
    },
    /// Generate an USP GetResp response message
    #[structopt(name = "GetResp")]
    USPGetResp {
        /// A JSON array of Strings resembling the result data for the GetResp operation
        #[structopt(multiple = true)]
        result: Vec<String>,
    },
    /// Generate an USP GetInstances request message
    #[structopt(name = "GetInstances")]
    USPGetInstances {
        /// Only return the first level of recursive structures?
        #[structopt(parse(try_from_str))]
        first_level_only: bool,
        /// A JSON array ressembling the object paths we're interested in
        ///
        /// Example use: '["Device.DeviceInfo.", "Device.LocalAgent."]'
        #[structopt(multiple = true)]
        obj_paths: Vec<String>,
    },
    /// Generate an USP GetSupportedDM request message
    #[structopt(name = "GetSupportedDM")]
    USPGetSupportedDM {
        /// Only return the first level of recursive structures?
        #[structopt(parse(try_from_str))]
        first_level_only: bool,
        /// Return commands?
        #[structopt(parse(try_from_str))]
        return_commands: bool,
        /// Return events?
        #[structopt(parse(try_from_str))]
        return_events: bool,
        /// Return parameters?
        #[structopt(parse(try_from_str))]
        return_params: bool,
        /// A JSON array ressembling the paths we're interested in
        ///
        /// Example use: '["Device.DeviceInfo.", "Device.LocalAgent."]'
        #[structopt(multiple = true)]
        paths: Vec<String>,
    },
    /// Generate an USP GetSupportedProtocol request message
    #[structopt(name = "GetSupportedProtocol")]
    USPGetSupportedProtocol {
        /// Controller Supported Protocol Version
        cspv: String,
    },
    /// Generate an USP Notify request message
    #[structopt(name = "Notify")]
    USPNotify {
        /// Subscription ID
        sub_id: String,
        /// Do we expect a response?
        #[structopt(parse(try_from_str))]
        send_resp: bool,
        /// Type of notification
        #[structopt(subcommand)]
        typ: NotifyType,
    },
    /// Generate an USP Notify response message
    #[structopt(name = "NotifyResp")]
    USPNotifyResp {
        /// Subscription ID
        sub_id: String,
    },
    /// Generate an USP Operate request message
    #[structopt(name = "Operate")]
    USPOperate {
        /// The full pathname of of the command to execute
        command: String,
        /// * The command key to use in the request to allow later matching with a result
        command_key: String,
        /// A boolean indicating whether a response is expected in reply to this request
        #[structopt(parse(try_from_str))]
        send_resp: bool,
        /// A JSON array of arrays containing the command input arguments with path names and values
        #[structopt(multiple = true)]
        args: Vec<String>,
    },
    /// Generate an USP Set request message
    #[structopt(name = "Set")]
    USPSet {
        /// Do we allow partial execution?
        #[structopt(parse(try_from_str))]
        allow_partial: bool,
        /// A JSON structure resesembling the input for a Set operation
        ///
        /// Example use: '[["Device.DeviceInfo.", [["ProvisioningCode", "configured", true]]]]'
        #[structopt(multiple = true)]
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

        // Open output stream
        let mut out = get_out_stream(None)?;

        match format {
            OutputFormat::JSON => {
                writeln!(
                    out,
                    "{}",
                    serde_json::to_string_pretty(&decoded).context("Failed to serialize JSON")?
                )?;
            }
            OutputFormat::Native => {
                writeln!(out, "{}", &decoded)?;
            }
            OutputFormat::CStr => {
                write_c_str(out, contents.as_slice())?;
            }
            OutputFormat::CArray => {
                write_c_array(out, contents.as_slice())?;
            }
            OutputFormat::Protobuf => {
                out.write_all(&contents.as_slice())?;
            }
        }
    }

    Ok(())
}

fn decode_msg_stdin(format: OutputFormat) -> Result<()> {
    let mut contents = Vec::new();
    stdin().read_to_end(&mut contents)?;

    // Try to parse bytes as a protobuf encoded USP Message
    let decoded = try_decode_msg(&contents)?;

    // Open output stream
    let mut out = get_out_stream(None)?;

    match format {
        OutputFormat::JSON => {
            writeln!(
                out,
                "{}",
                serde_json::to_string_pretty(&decoded).context("Failed to serialize JSON")?
            )?;
        }
        OutputFormat::Native => {
            writeln!(out, "{}", &decoded)?;
        }
        OutputFormat::CStr => {
            write_c_str(out, contents.as_slice())?;
        }
        OutputFormat::CArray => {
            write_c_array(out, contents.as_slice())?;
        }
        OutputFormat::Protobuf => {
            out.write_all(&contents.as_slice())?;
        }
    }

    Ok(())
}

fn decode_record_files(files: Vec<PathBuf>, format: OutputFormat) -> Result<()> {
    for file in files {
        let fp = File::open(&file)?;
        let mut buf_reader = BufReader::new(fp);
        let mut contents = Vec::new();
        buf_reader.read_to_end(&mut contents)?;

        // Try to parse bytes as a protobuf encoded USP Record
        let decoded = try_decode_record(&contents)?;

        // Open output stream
        let mut out = get_out_stream(None)?;

        match format {
            OutputFormat::JSON => {
                writeln!(
                    &mut out,
                    "{}",
                    serde_json::to_string_pretty(&decoded).context("Failed to serialize JSON")?
                )?;
            }
            OutputFormat::Native => {
                writeln!(out, "{}", &decoded)?;
            }
            OutputFormat::CStr => {
                write_c_str(out, contents.as_slice())?;
            }
            OutputFormat::CArray => {
                write_c_array(out, contents.as_slice())?;
            }
            OutputFormat::Protobuf => {
                out.write_all(&contents.as_slice())?;
            }
        }
    }

    Ok(())
}

fn decode_record_stdin(format: OutputFormat) -> Result<()> {
    let mut contents = Vec::new();
    stdin().read_to_end(&mut contents)?;

    // Try to parse bytes as a protobuf encoded USP Record
    let decoded = try_decode_record(&contents)?;

    // Open output stream
    let mut out = get_out_stream(None)?;

    match format {
        OutputFormat::JSON => {
            writeln!(
                out,
                "{}",
                serde_json::to_string_pretty(&decoded).context("Failed to serialize JSON")?
            )?;
        }
        OutputFormat::Native => {
            writeln!(out, "{}", &decoded)?;
        }
        OutputFormat::CStr => {
            write_c_str(out, contents.as_slice())?;
        }
        OutputFormat::CArray => {
            write_c_array(out, contents.as_slice())?;
        }
        OutputFormat::Protobuf => {
            out.write_all(&contents.as_slice())?;
        }
    }

    Ok(())
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
            serialize_into_vec(&usp_generator::usp_simple_error(code, message))
        }
        MsgType::USPGet { paths } => {
            let paths = paths.join(" ");
            let v = serde_json::from_str::<Vec<&str>>(&paths)
                .with_context(|| format!("Expected JSON data in the form \"[<Path name>, ...]\",  got '{}'", paths))?;
            serialize_into_vec(&usp_generator::usp_get_request(v.as_slice()))
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

fn encode_msg_body(filename: Option<PathBuf>, typ: MsgType, as_c_array: bool) -> Result<()> {
    use quick_protobuf::{deserialize_from_slice, message::MessageWrite, Writer};

    let mut buf = Vec::new();
    let mut writer = Writer::new(&mut buf);

    let encoded_body = encode_msg_body_buf(typ)?;
    let body: rusp::usp::Body =
        deserialize_from_slice(&encoded_body).context("Failed trying to deserialise Msg body")?;

    body.write_message(&mut writer)
        .context("Failed encoding USP Msg")?;

    // Open output stream
    let mut out = get_out_stream(filename)?;

    if as_c_array {
        write_c_array(out, &buf)
    } else {
        Ok(out.write_all(&buf)?)
    }
}

fn encode_msg(
    msgid: String,
    filename: Option<PathBuf>,
    typ: MsgType,
    as_c_array: bool,
) -> Result<()> {
    use quick_protobuf::{deserialize_from_slice, message::MessageWrite, Writer};

    let mut buf = Vec::new();
    let mut writer = Writer::new(&mut buf);

    let encoded_body = encode_msg_body_buf(typ)?;
    let body: rusp::usp::Body =
        deserialize_from_slice(&encoded_body).context("Failed trying to deserialise Msg body")?;
    usp_generator::usp_msg(msgid, body)
        .write_message(&mut writer)
        .context("Failed encoding USP Msg")?;

    // Open output stream
    let mut out = get_out_stream(filename)?;

    if as_c_array {
        write_c_array(out, &buf)
    } else {
        Ok(out.write_all(&buf)?)
    }
}

fn extract_msg(in_file: &PathBuf, out_file: &PathBuf) -> Result<()> {
    use rusp::usp_record::mod_Record::OneOfrecord_type::*;

    let fp = File::open(&in_file)?;
    let mut buf_reader = BufReader::new(fp);
    let mut contents = Vec::new();
    buf_reader.read_to_end(&mut contents)?;

    let record = try_decode_record(&contents)?;

    match record.record_type {
        no_session_context(context) => {
            let msg = context.payload;
            std::fs::write(&out_file, &msg)?;
        }
        session_context(_) => unreachable!(),
        None => unreachable!(),
    }

    Ok(())
}

fn extract_msg_body(in_file: &PathBuf, out_file: &PathBuf) -> Result<()> {
    use quick_protobuf::{message::MessageWrite, Writer};
    use rusp::usp_record::mod_Record::OneOfrecord_type::*;

    let fp = File::open(&in_file)?;
    let mut buf_reader = BufReader::new(fp);
    let mut contents = Vec::new();
    buf_reader.read_to_end(&mut contents)?;

    let record = try_decode_record(&contents)?;

    match record.record_type {
        no_session_context(context) => {
            let mut buf = Vec::new();
            let mut writer = Writer::new(&mut buf);

            let payload = context.payload;
            let msg = try_decode_msg(&payload)?;
            let body = msg.body.context("Failed extracting USP Msg body")?;
            body.write_message(&mut writer)
                .context("Failed encoding USP Msg body")?;
            std::fs::write(&out_file, buf)?;
        }
        session_context(_) => unreachable!(),
        None => unreachable!(),
    }

    Ok(())
}

fn wrap_msg_raw(
    version: Option<String>,
    from: Option<String>,
    to: Option<String>,
    filename: Option<PathBuf>,
    as_c_array: bool,
) -> Result<()> {
    use quick_protobuf::{message::MessageWrite, Writer};

    let mut msg = Vec::new();
    stdin().read_to_end(&mut msg)?;

    let mut buf = Vec::new();
    let mut writer = Writer::new(&mut buf);

    usp_generator::usp_no_session_context_record(
        version.as_deref().unwrap(),
        from.as_deref().unwrap(),
        to.as_deref().unwrap(),
        &msg,
    )
    .write_message(&mut writer)
    .context("Failed encoding USP Record")?;

    // Open output stream
    let mut out = get_out_stream(filename)?;

    if as_c_array {
        write_c_array(out, &buf)
    } else {
        Ok(out.write_all(&buf)?)
    }
}

fn main() -> Result<()> {
    let Rusp {
        action,
        json,
        cstr,
        carray,
        protobuf,
    } = Rusp::from_args();

    // Pass on the user chosen format to use for the output
    let format = {
        if json == true {
            OutputFormat::JSON
        } else if carray == true {
            OutputFormat::CArray
        } else if cstr == true {
            OutputFormat::CStr
        } else if protobuf == true {
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
        } => encode_msg_body(filename, typ, as_c_array),
        RuspAction::EncodeMsg {
            msgid,
            filename,
            typ,
            as_c_array,
        } => encode_msg(msgid, filename, typ, as_c_array),
        RuspAction::ExtractMsg { in_file, out_file } => extract_msg(&in_file, &out_file),
        RuspAction::ExtractMsgBody { in_file, out_file } => extract_msg_body(&in_file, &out_file),
        RuspAction::WrapMsgRaw {
            version,
            from,
            to,
            filename,
            as_c_array,
        } => wrap_msg_raw(version, from, to, filename, as_c_array),
    }?;

    Ok(())
}
