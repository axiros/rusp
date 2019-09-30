use quick_protobuf;
use serde_json;
use std::error::Error;
use std::fs::File;
use std::io::{stdin, stdout, BufReader, Read, Write};
use std::path::PathBuf;
use structopt::*;

use rusp::{
    usp_decoder::{decode_msg, decode_record},
    usp_generator,
    usp_types::NotifyType,
};

type Bool = bool;

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
    /// Generate an USP Add request message
    USPAdd {
        /// Do we allow partial execution?
        allow_partial: Bool,
        /// A JSON structure resesembling the input for a Add operation
        ///
        /// Example use: '[["Device.DeviceInfo.", [["ProvisioningCode", "configured", true]]]]'
        #[structopt(multiple = true)]
        args: Vec<String>,
    },
    /// Generate an USP Error message
    USPError {
        /// The USP error code (MUST be between 7000 and 7999)
        code: u32,
        /// An (optional) error message. Standard error messages will be computed from the error
        /// code if not provided
        message: Option<String>,
    },
    /// Generate an USP Get request message
    USPGet {
        /// A JSON array of Strings resembling the paths for the Get operation
        #[structopt(multiple = true)]
        paths: Vec<String>,
    },
    /// Generate an USP GetResp response message
    USPGetResp {
        /// A JSON array of Strings resembling the result data for the GetResp operation
        #[structopt(multiple = true)]
        result: Vec<String>,
    },
    /// Generate an USP GetInstances request message
    USPGetInstances {
        /// Only return the first level of recursive structures?
        first_level_only: Bool,
        /// A JSON array ressembling the object paths we're interested in
        ///
        /// Example use: '["Device.DeviceInfo.", "Device.LocalAgent."]'
        #[structopt(multiple = true)]
        obj_paths: Vec<String>,
    },
    /// Generate an USP GetSupportedDM request message
    USPGetSupportedDM {
        /// Only return the first level of recursive structures?
        first_level_only: Bool,
        /// Return commands?
        return_commands: Bool,
        /// Return events?
        return_events: Bool,
        /// Return parameters?
        return_params: Bool,
        /// A JSON array ressembling the paths we're interested in
        ///
        /// Example use: '["Device.DeviceInfo.", "Device.LocalAgent."]'
        #[structopt(multiple = true)]
        paths: Vec<String>,
    },
    /// Generate an USP GetSupportedProtocol request message
    USPGetSupportedProtocol {
        /// Controller Supported Protocol Version
        cspv: String,
    },
    /// Generate an USP Notify request message
    USPNotify {
        /// Subscription ID
        sub_id: String,
        /// Do we expect a response?
        send_resp: Bool,
        /// Type of notification
        #[structopt(subcommand)]
        typ: NotifyType,
    },
    /// Generate an USP Notify response message
    USPNotifyResp {
        /// Subscription ID
        sub_id: String,
    },
    /// Generate an USP Operate request message
    USPOperate {
        /// The full pathname of of the command to execute
        command: String,
        /// * The command key to use in the request to allow later matching with a result
        command_key: String,
        /// A boolean indicating whether a response is expected in reply to this request
        send_resp: Bool,
        /// A JSON array of arrays containing the command input arguments with path names and values
        #[structopt(multiple = true)]
        args: Vec<String>,
    },
    /// Generate an USP Set request message
    USPSet {
        /// Do we allow partial execution?
        allow_partial: Bool,
        /// A JSON structure resesembling the input for a Set operation
        ///
        /// Example use: '[["Device.DeviceInfo.", [["ProvisioningCode", "configured", true]]]]'
        #[structopt(multiple = true)]
        args: Vec<String>,
    },
}

fn decode_msg_files(files: Vec<PathBuf>) -> Result<(), Box<dyn Error>> {
    for file in files {
        let fp = File::open(&file)?;
        let mut buf_reader = BufReader::new(fp);
        let mut contents = Vec::new();
        buf_reader.read_to_end(&mut contents)?;

        println!("{}", decode_msg(&contents));
    }

    Ok(())
}

fn decode_msg_stdin() -> Result<(), Box<dyn Error>> {
    let mut contents = Vec::new();
    stdin().read_to_end(&mut contents)?;

    println!("{}", decode_msg(&contents));

    Ok(())
}

fn decode_record_files(files: Vec<PathBuf>) -> Result<(), Box<dyn Error>> {
    for file in files {
        let fp = File::open(&file)?;
        let mut buf_reader = BufReader::new(fp);
        let mut contents = Vec::new();
        buf_reader.read_to_end(&mut contents)?;

        println!("{}", decode_record(&contents));
    }

    Ok(())
}

fn decode_record_stdin() -> Result<(), Box<dyn Error>> {
    let mut contents = Vec::new();
    stdin().read_to_end(&mut contents)?;

    println!("{}", decode_record(&contents));

    Ok(())
}

fn encode_msg_body_buf(typ: MsgType) -> Result<Vec<u8>, Box<dyn Error>> {
    use quick_protobuf::serialize_into_vec;

    match typ {
        MsgType::USPAdd {
            allow_partial,
            args,
        } => {
            let args = args.join(" ");
            let v = serde_json::from_str::<Vec<(&str, Vec<(&str, &str, bool)>)>>(&args).map_err(
                |e| {
                    format!(
                        "Please provide an appropriate JSON datastructure, got '{}': {}",
                        args.trim(),
                        e
                    )
                },
            )?;
            serialize_into_vec(&usp_generator::usp_add_request(
                allow_partial,
                v.iter()
                    .map(|(path, par)| (*path, par.as_slice()))
                    .collect::<Vec<_>>()
                    .as_slice(),
            ))
        }
        MsgType::USPError { code, message } => {
            serialize_into_vec(&usp_generator::usp_simple_error(code, message))
        }
        MsgType::USPGet { paths } => {
            let paths = paths.join(" ");
            let v = serde_json::from_str::<Vec<&str>>(&paths).map_err(|e| {
                format!(
                    "Please provide a JSON array with datamodel paths, got '{}': {}",
                    paths.trim(),
                    e
                )
            })?;
            serialize_into_vec(&usp_generator::usp_get_request(v.as_slice()))
        }
        MsgType::USPGetInstances {
            first_level_only,
            obj_paths,
        } => {
            let obj_paths = obj_paths.join(" ");
            let v = serde_json::from_str::<Vec<&str>>(&obj_paths).map_err(|e| {
                format!(
                    "Please provide a JSON array with datamodel paths, got '{}': {}",
                    obj_paths.trim(),
                    e
                )
            })?;
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
            let v = serde_json::from_str::<Vec<&str>>(&paths).map_err(|e| {
                format!(
                    "Please provide a JSON array with datamodel paths, got '{}': {}",
                    paths.trim(),
                    e
                )
            })?;
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
            let getresp_json: usp_generator::GetResp =
                serde_json::from_str(&result).map_err(|e| {
                    format!(
                        "Please provide an appropriate JSON datastructure, got '{}': {}",
                        result.trim(),
                        e
                    )
                })?;
            serialize_into_vec(&usp_generator::usp_get_response_from_json(&getresp_json))
        }
        MsgType::USPNotify {
            sub_id,
            send_resp,
            typ,
        } => serialize_into_vec(&usp_generator::usp_notify_request(&sub_id, send_resp, typ)),
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
            let v = if args.len() > 0 {
                serde_json::from_str::<Vec<(&str, &str)>>(&args).map_err(|e| {
                    format!(
                        "Please provide an appropriate JSON datastructure, got '{}': {}",
                        args.trim(),
                        e
                    )
                })?
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
            let v = serde_json::from_str::<Vec<(&str, Vec<(&str, &str, bool)>)>>(&args).map_err(
                |e| {
                    format!(
                        "Please provide an appropriate JSON datastructure, got '{}': {}",
                        args.trim(),
                        e
                    )
                },
            )?;
            serialize_into_vec(&usp_generator::usp_set_request(
                allow_partial,
                v.iter()
                    .map(|(path, par)| (*path, par.as_slice()))
                    .collect::<Vec<_>>()
                    .as_slice(),
            ))
        }
    }
    .map_err(|_| "Cannot encode message".into())
}

fn encode_msg_body(filename: Option<PathBuf>, typ: MsgType) -> Result<(), Box<dyn Error>> {
    use quick_protobuf::{deserialize_from_slice, message::MessageWrite, Writer};

    let mut buf = Vec::new();
    let mut writer = Writer::new(&mut buf);

    let encoded_body = encode_msg_body_buf(typ)?;
    let body: rusp::usp::Body = deserialize_from_slice(&encoded_body)
        .map_err(|e| format!("Could not deserialise Msg body: {}", e))?;
    body.write_message(&mut writer)
        .expect("Failed encoding USP Msg");

    if let Some(filename) = filename {
        std::fs::write(filename, buf)?;
    } else {
        stdout().write_all(&buf)?;
    }

    Ok(())
}

fn encode_msg(
    msgid: String,
    filename: Option<PathBuf>,
    typ: MsgType,
) -> Result<(), Box<dyn Error>> {
    use quick_protobuf::{deserialize_from_slice, message::MessageWrite, Writer};

    let mut buf = Vec::new();
    let mut writer = Writer::new(&mut buf);

    let encoded_body = encode_msg_body_buf(typ)?;
    let body: rusp::usp::Body = deserialize_from_slice(&encoded_body)
        .map_err(|e| format!("Could not deserialise Msg body: {}", e))?;
    usp_generator::usp_msg(msgid, body)
        .write_message(&mut writer)
        .expect("Failed encoding USP Msg");

    if let Some(filename) = filename {
        std::fs::write(filename, buf)?;
    } else {
        stdout().write_all(&buf)?;
    }

    Ok(())
}

fn extract_msg(in_file: &PathBuf, out_file: &PathBuf) -> Result<(), Box<dyn Error>> {
    use rusp::usp_record::mod_Record::OneOfrecord_type::*;

    let fp = File::open(&in_file)?;
    let mut buf_reader = BufReader::new(fp);
    let mut contents = Vec::new();
    buf_reader.read_to_end(&mut contents)?;

    let record = decode_record(&contents);

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

fn extract_msg_body(in_file: &PathBuf, out_file: &PathBuf) -> Result<(), Box<dyn Error>> {
    use quick_protobuf::{message::MessageWrite, Writer};
    use rusp::usp_record::mod_Record::OneOfrecord_type::*;

    let fp = File::open(&in_file)?;
    let mut buf_reader = BufReader::new(fp);
    let mut contents = Vec::new();
    buf_reader.read_to_end(&mut contents)?;

    let record = decode_record(&contents);

    match record.record_type {
        no_session_context(context) => {
            let mut buf = Vec::new();
            let mut writer = Writer::new(&mut buf);

            let payload = context.payload;
            let msg = decode_msg(&payload);
            let body = msg.body.unwrap();
            body.write_message(&mut writer)
                .expect("Cannot encode message");
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
) -> Result<(), Box<dyn Error>> {
    use quick_protobuf::{message::MessageWrite, Writer};

    let mut msg = Vec::new();
    stdin().read_to_end(&mut msg)?;

    let mut buf = Vec::new();
    let mut writer = Writer::new(&mut buf);

    usp_generator::usp_no_session_context_record(
        version.as_ref().map(String::as_str).unwrap(),
        from.as_ref().map(String::as_str).unwrap(),
        to.as_ref().map(String::as_str).unwrap(),
        &msg,
    )
    .write_message(&mut writer)
    .expect("Failed encoding USP Record");

    if let Some(filename) = filename {
        std::fs::write(filename, buf)?;
    } else {
        stdout().write_all(&buf)?;
    }

    Ok(())
}

#[paw::main]
fn main(opt: Rusp) -> Result<(), Box<dyn Error>> {
    color_backtrace::install();

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
    .map_err(|e| {
        eprintln!("Whoopsiedoodles! Something went wrong, this is what we've got:");
        e
    })?;

    Ok(())
}
