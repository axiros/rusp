#![allow(clippy::type_complexity)]

use clap::{Parser, Subcommand};
use rusp::usp_builder;
use rusp::usp_record::mod_MQTTConnectRecord::MQTTVersion;
use std::collections::HashMap;
use std::convert::Infallible;
use std::fs::File;
use std::io::{stdin, stdout, BufReader, Read, Write};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use rusp::{
    usp_decoder::{try_decode_msg, try_decode_record},
    usp_types::{NotifyType as RuspNotifyType, OperateResponse as RuspOperateResponse},
};

#[derive(PartialEq)]
/// The supported output formats
enum OutputFormat {
    /// Valid JSON format
    Json,
    /// Protobuf output as C strings or Rust byte arrays where non-ASCII characters are replaced with
    /// backslashed escaped hex codes
    CStr,
    /// Protobuf output as C array with preview comments for inclusion in source code
    CArray,
    /// Native Protobuf binary output
    Protobuf,
}

#[derive(Parser)]
#[command(author, version, name = "rusp", about = "the Rust USP toolkit")]
struct Rusp {
    #[arg(
        long = "carray",
        conflicts_with = "cstr",
        conflicts_with = "json",
        conflicts_with = "protobuf"
    )]
    /// Output as C array (and length) for inclusion in source code
    carray: bool,
    #[arg(
        long = "json",
        conflicts_with = "cstr",
        conflicts_with = "carray",
        conflicts_with = "protobuf"
    )]
    /// Output as JSON (the default format as of version 0.14)
    json: bool,
    #[arg(
        long = "cstr",
        conflicts_with = "json",
        conflicts_with = "carray",
        conflicts_with = "protobuf"
    )]
    /// Output binary as Protobuf in a C string / Rust byte array representation
    cstr: bool,
    #[arg(
        long = "protobuf",
        conflicts_with = "json",
        conflicts_with = "carray",
        conflicts_with = "cstr"
    )]
    /// Output binary as native Protobuf binary
    protobuf: bool,
    #[command(subcommand)]
    action: RuspAction,
}

#[derive(Subcommand)]
enum RuspAction {
    /// Decode a single raw USP message from standard input and print to standard output
    #[command(name = "decode_msg")]
    DecodeMsg {},
    /// Decode ore or more USP messages from specified filenames and print to standard output
    #[command(name = "decode_msg_files")]
    DecodeMsgFiles {
        #[arg(required = true)]
        /// Filenames of USP Protobuf messages to decode
        files: Vec<PathBuf>,
    },
    /// Decode a single raw USP record from standard input and print to standard output
    #[command(name = "decode_record")]
    DecodeRecord {},
    /// Decode one or more USP records from specified filenames and print to standard output
    #[command(name = "decode_record_files")]
    DecodeRecordFiles {
        #[arg(required = true)]
        /// Filenames of USP Protobuf records to decode
        files: Vec<PathBuf>,
    },
    /// Encode command line input into a single raw USP message
    #[command(name = "encode_msg")]
    EncodeMsg {
        /// The message ID to use in the USP Msg header
        msgid: String,
        /// Filename (will output to standard output if omitted)
        #[arg(short = 'f', long = "file")]
        /// Output filename of file to encode USP Protobuf message to
        filename: Option<PathBuf>,
        /// Type of message
        #[command(subcommand)]
        typ: MsgType,
    },
    /// Extract the USP message from an USP record
    #[command(name = "extract_msg")]
    ExtractMsg {
        /// Input filename of USP Protobuf record to decode
        in_file: PathBuf,
        /// Output filename of USP Protobuf message to write into, use `-` for stdout
        out_file: PathBuf,
    },
    /// Encode Msg payload provided via stdin into a single no-session context USP Record
    #[command(name = "encode_no_session_record")]
    EncodeNoSessionRecord {
        #[arg(long = "version", default_value = "1.3")]
        /// USP specification version
        version: String,
        #[arg(long = "from", default_value = "doc::from")]
        /// Sender Id
        from: String,
        #[arg(long = "to", default_value = "doc::to")]
        /// Recipient Id
        to: String,
        /// Filename (will output to standard output if omitted)
        #[arg(short = 'f', long = "file")]
        /// Output filename of file to encode USP Protobuf record to
        filename: Option<PathBuf>,
    },
    /// Encode Msg payload provided via stdin into a single session context USP Record
    #[command(name = "encode_session_record")]
    EncodeSessionRecord {
        #[arg(long = "version", default_value = "1.3")]
        /// USP specification version
        version: String,
        #[arg(long = "from", default_value = "doc::from")]
        /// Sender Id
        from: String,
        #[arg(long = "to", default_value = "doc::to")]
        /// Recipient Id
        to: String,
        #[arg(long = "session_id", default_value = "1234")]
        /// The ID of the context session
        session_id: u64,
        #[arg(long = "sequence_id", default_value = "1")]
        /// The sequence number within the context session
        sequence_id: u64,
        #[arg(long = "expected_id", default_value = "2")]
        /// The expected next sequence number within the context session
        expected_id: u64,
        #[arg(long = "retransmit_id", default_value = "0")]
        /// The sequence number of the part which is being retransmitted
        retransmit_id: u64,
        /// Filename (will output to standard output if omitted)
        #[arg(short = 'f', long = "file")]
        /// Output filename of file to encode USP Protobuf record to
        filename: Option<PathBuf>,
    },
    /// Encode a USP Record of type MQTT Connect
    #[command(name = "create_mqtt_connect_record")]
    CreateMQTTConnectRecord {
        #[arg(long = "version", default_value = "1.3")]
        /// USP specification version
        version: String,
        #[arg(long = "from", default_value = "doc::from")]
        /// Sender Id
        from: String,
        #[arg(long = "to", default_value = "doc::to")]
        /// Recipient Id
        to: String,
        /// Indicate that we're using MQTT v3.11 instead of the default MQTT 5
        #[arg(short = '4', long = "mqtt311")]
        mqtt311: bool,
        /// The subscribed topic the MQTT client is expecting to receive the messages for
        #[arg(short = 's')]
        subscribed_topic: String,
        /// Filename (will output to standard output if omitted)
        #[arg(short = 'f', long = "file")]
        /// Output filename of file to encode USP Protobuf record to
        filename: Option<PathBuf>,
    },
}

/// Parse a JSON object into a Rust `HashMap`
fn parse_key_val_json(s: &str) -> Result<HashMap<String, String>, String> {
    serde_json::from_str::<HashMap<String, String>>(s).map_err(|e| e.to_string())
}

#[allow(dead_code)]
#[derive(PartialEq, Eq)]
enum OperateResponse {
    OutputArgs(HashMap<String, String>),
    CommandFailure(u32, String),
}

impl Default for OperateResponse {
    fn default() -> Self {
        Self::OutputArgs(HashMap::new())
    }
}

#[derive(Parser, PartialEq, Eq)]
enum NotifyType {
    /// USP `OnBoardRequest` notification
    OnBoardRequest {
        /// The OUI associated with the manufacturer of the device
        oui: String,

        /// The product class associated with the device
        product_class: String,

        /// The serial number of the device
        serial_number: String,

        /// A comma separated list of supported USP versions
        agent_supported_protocol_versions: String,
    },
    /// USP `ValueChange` notification
    ValueChange {
        /// The path of the changed parameter
        param_path: String,
        /// The new value of the changed parameter
        param_value: String,
    },
    /// USP Event notification
    Event {
        /// The path of the event
        obj_path: String,
        /// The name of the event
        event_name: String,
        /// A stringified JSON object containing the output arguments of the USP Event
        #[arg(value_parser = parse_key_val_json)]
        params: HashMap<String, String>,
    },
    /// USP `ObjectCreation` notification
    ObjectCreation {
        /// The path of the created object
        obj_path: String,
        /// A stringified JSON object containing the `unique_keys` and values of the created Object
        #[arg(value_parser = parse_key_val_json)]
        unique_keys: HashMap<String, String>,
    },
    /// USP `ObjectDeletion` notification
    ObjectDeletion {
        /// The path of the deleted object
        obj_path: String,
    },

    /// USP `OperationComplete` notification
    OperationComplete {
        /// The path of the operation object
        obj_path: String,
        /// The name of the operated command
        command_name: String,
        /// The command key associated with the operation
        command_key: String,
        /// The result of the operation
        #[structopt(skip)]
        operation_resp: OperateResponse,
    },
}

impl TryFrom<NotifyType> for RuspNotifyType {
    type Error = Infallible;

    fn try_from(notify: NotifyType) -> Result<Self, Self::Error> {
        Ok(match notify {
            NotifyType::OnBoardRequest {
                oui,
                product_class,
                serial_number,
                agent_supported_protocol_versions,
            } => Self::OnBoardRequest {
                oui,
                product_class,
                serial_number,
                agent_supported_protocol_versions,
            },
            NotifyType::ValueChange {
                param_path,
                param_value,
            } => Self::ValueChange {
                param_path,
                param_value,
            },
            NotifyType::Event {
                obj_path,
                event_name,
                params,
            } => Self::Event {
                obj_path,
                event_name,
                params,
            },
            NotifyType::ObjectCreation {
                obj_path,
                unique_keys,
            } => Self::ObjectCreation {
                obj_path,
                unique_keys,
            },
            NotifyType::ObjectDeletion { obj_path } => Self::ObjectDeletion { obj_path },
            NotifyType::OperationComplete {
                obj_path,
                command_name,
                command_key,
                operation_resp,
            } => Self::OperationComplete {
                obj_path,
                command_name,
                command_key,
                operation_resp: match operation_resp {
                    OperateResponse::OutputArgs(a) => RuspOperateResponse::OutputArgs(a),
                    OperateResponse::CommandFailure(code, msg) => {
                        RuspOperateResponse::CommandFailure(code, msg)
                    }
                },
            },
        })
    }
}

#[derive(Parser)]
#[command(rename_all = "verbatim")]
enum MsgType {
    /// Generate an USP Add Request Message
    #[command(name = "Add")]
    USPAdd {
        /// Do we allow partial execution?
        #[arg(action = clap::ArgAction::Set)]
        allow_partial: bool,
        /// A JSON structure resembling the input for a Add operation
        ///
        /// Example use: '[["Device.DeviceInfo.", [["ProvisioningCode", "configured", true]]]]'
        #[arg(num_args(1..))]
        args: Vec<String>,
    },
    /// Generate an USP Delete Request Message
    #[command(name = "Delete")]
    USPDelete {
        /// Do we allow partial execution?
        #[arg(action = clap::ArgAction::Set)]
        allow_partial: bool,
        /// A JSON structure resembling the input for a Delete operation
        ///
        /// Example use: '["Device.XMPP.Connection.1.", "Device.LocalAgent.Subscription.3."]'
        #[arg(num_args(1..))]
        obj_paths: Vec<String>,
    },
    /// Generate an USP Register Request Message
    #[command(name = "Deregister")]
    USPDeregister {
        /// A JSON structure resembling the input for a Deregister operation
        ///
        /// Example use: '["Device.DeviceInfo.", "Device.Services.UPSService.1."]'
        #[arg(num_args(1..))]
        paths: Vec<String>,
    },
    /// Generate an USP Error message
    #[command(name = "Error")]
    USPError {
        /// The USP error code (MUST be between 7000 and 7999)
        code: u32,
        /// An (optional) error message. Standard error messages will be computed from the error
        /// code if not provided
        message: Option<String>,
    },
    /// Generate an USP Get Request Message
    #[command(name = "Get")]
    USPGet {
        /// A JSON array of Strings resembling the paths for the Get operation
        #[arg(num_args(1..))]
        paths: Vec<String>,
        #[arg(long = "max_depth")]
        max_depth: Option<u32>,
    },
    /// Generate an USP Get Response Message
    #[command(name = "GetResp")]
    USPGetResp {
        /// A JSON array of Strings resembling the result data for the GetResp operation
        #[arg(num_args(1..))]
        result: Vec<String>,
    },
    /// Generate an USP GetInstances Request Message
    #[command(name = "GetInstances")]
    USPGetInstances {
        /// Only return the first level of recursive structures?
        #[arg(action = clap::ArgAction::Set)]
        first_level_only: bool,
        /// A JSON array resembling the object paths we're interested in
        ///
        /// Example use: '["Device.DeviceInfo.", "Device.LocalAgent."]'
        #[arg(num_args(1..))]
        obj_paths: Vec<String>,
    },
    /// Generate an USP GetSupportedDM Request Message
    #[command(name = "GetSupportedDM")]
    USPGetSupportedDM {
        /// Only return the first level of recursive structures?
        #[arg(action = clap::ArgAction::Set)]
        first_level_only: bool,
        /// Return commands?
        #[arg(action = clap::ArgAction::Set)]
        return_commands: bool,
        /// Return events?
        #[arg(action = clap::ArgAction::Set)]
        return_events: bool,
        /// Return parameters?
        #[arg(action = clap::ArgAction::Set)]
        return_params: bool,
        /// Return unique key sets?
        #[arg(action = clap::ArgAction::Set)]
        return_unique_key_sets: bool,
        /// A JSON array resembling the paths we're interested in
        ///
        /// Example use: '["Device.DeviceInfo.", "Device.LocalAgent."]'
        #[arg(num_args(1..))]
        paths: Vec<String>,
    },
    /// Generate an USP GetSupportedProtocol Request Message
    #[command(name = "GetSupportedProtocol")]
    USPGetSupportedProtocol {
        /// Controller Supported Protocol Version
        cspv: String,
    },
    /// Generate an USP Notify Request Message
    #[command(name = "Notify")]
    USPNotify {
        /// Subscription ID
        sub_id: String,
        /// Do we expect a response?
        #[arg(action = clap::ArgAction::Set)]
        send_resp: bool,
        /// Type of notification
        #[command(subcommand)]
        typ: NotifyType,
    },
    /// Generate an USP Notify Response Message
    #[command(name = "NotifyResp")]
    USPNotifyResp {
        /// Subscription ID
        sub_id: String,
    },
    /// Generate an USP Operate Request Message
    #[command(name = "Operate")]
    USPOperate {
        /// The full pathname of of the command to execute
        command: String,
        /// The command key to use in the request to allow later matching with a result
        command_key: String,
        /// A boolean indicating whether a response is expected in reply to this request
        #[arg(action = clap::ArgAction::Set)]
        send_resp: bool,
        /// A JSON array of arrays containing the command input arguments with path names and values
        #[arg(num_args(1..))]
        args: Vec<String>,
    },
    /// Generate an USP Set Request Message
    #[command(name = "Set")]
    USPSet {
        /// Do we allow partial execution?
        #[arg(action = clap::ArgAction::Set)]
        allow_partial: bool,
        /// A JSON structure resembling the input for a Set operation
        ///
        /// Example use: '[["Device.DeviceInfo.", [["ProvisioningCode", "configured", true]]]]'
        #[arg(num_args(1..))]
        args: Vec<String>,
    },
    /// Generate an USP Register Request Message
    #[command(name = "Register")]
    USPRegister {
        /// Do we allow partial execution?
        #[arg(action = clap::ArgAction::Set)]
        allow_partial: bool,
        /// A JSON structure resembling the input for a Register operation
        ///
        /// Example use: '["Device.DeviceInfo.", "Device.Services.UPSService.1."]'
        #[arg(num_args(1..))]
        reg_paths: Vec<String>,
    },
}

fn decode_msg_files(files: Vec<PathBuf>, format: &OutputFormat) -> Result<()> {
    for file in files {
        let fp = File::open(&file)?;
        let mut buf_reader = BufReader::new(fp);
        let mut contents = Vec::new();
        buf_reader.read_to_end(&mut contents)?;

        // Try to parse bytes as a Protobuf encoded USP Message
        let decoded = try_decode_msg(&contents)?;

        // Open stdout as output stream and write the USP Msg to it
        write_msg(&decoded, get_out_stream(None)?, format)?;
    }

    Ok(())
}

fn decode_msg_stdin(format: &OutputFormat) -> Result<()> {
    let mut contents = Vec::new();
    stdin().read_to_end(&mut contents)?;

    // Try to parse bytes as a Protobuf encoded USP Message
    let decoded = try_decode_msg(&contents)?;

    // Open stdout as output stream and write the USP Msg to it
    write_msg(&decoded, get_out_stream(None)?, format)
}

fn decode_record_files(files: Vec<PathBuf>, format: &OutputFormat) -> Result<()> {
    for file in files {
        let fp = File::open(&file)?;
        let mut buf_reader = BufReader::new(fp);
        let mut contents = Vec::new();
        buf_reader.read_to_end(&mut contents)?;

        // Try to parse bytes as a Protobuf encoded USP Record
        let decoded = try_decode_record(&contents)?;

        // Open stdout as output stream and write the USP Record to it
        write_record(&decoded, get_out_stream(None)?, format)?;
    }

    Ok(())
}

fn decode_record_stdin(format: &OutputFormat) -> Result<()> {
    let mut contents = Vec::new();
    stdin().read_to_end(&mut contents)?;

    // Try to parse bytes as a Protobuf encoded USP Record
    let decoded = try_decode_record(&contents)?;

    // Open stdout as output stream and write the USP Record to it
    write_record(&decoded, get_out_stream(None)?, format)
}

#[allow(clippy::too_many_lines)]
fn encode_msg_body_buf(typ: MsgType) -> Result<Vec<u8>> {
    use quick_protobuf::serialize_into_vec;

    match typ {
        MsgType::USPAdd {
            allow_partial,
            args,
        } => {
            let args = args.join(" ");
            let v = serde_json::from_str::<Vec<(String, Vec<(String, String, bool)>)>>(&args)
                .with_context(|| format!("Expected JSON data in the form \"[[<Object path>, [[<Parameter name>, <Parameter value>, <Required>], ...]], ...]\", got {args}"))?;

            let builder = usp_builder::AddBuilder::new().with_allow_partial(allow_partial);

            let create_objs = v.into_iter().map(|o| {
                usp_builder::CreateObjectBuilder::new(o.0).with_param_settings(o.1)
            }).collect::<Vec<_>>();

            serialize_into_vec(&builder.with_create_objs(create_objs).build()?)
        }
        MsgType::USPDelete {
            allow_partial,
            obj_paths,
        } => {
            let obj_paths = obj_paths.join(" ");
            let obj_paths = serde_json::from_str::<Vec<String>>(&obj_paths)
                .with_context(|| format!("Expected JSON data in the form \"[<Object instance path>, ...]\", got {obj_paths}"))?;
            serialize_into_vec(&usp_builder::DeleteBuilder::new().with_allow_partial(allow_partial).with_obj_paths(obj_paths).build()?)
        }
        MsgType::USPError { code, message } => {
            let error = usp_builder::ErrorBuilder::new().set_err (code, message);
            serialize_into_vec(&error.build()?)
        }
        MsgType::USPGet { paths, max_depth } => {
            let paths = paths.join(" ");
            let v = serde_json::from_str::<Vec<String>>(&paths)
                .with_context(|| format!("Expected JSON data in the form \"[<Path name>, ...]\", got {paths}"))?;
            serialize_into_vec(&usp_builder::GetBuilder::new().with_max_depth(max_depth.unwrap_or(0)).with_params(v).build()?)
        }
        MsgType::USPGetInstances {
            first_level_only,
            obj_paths,
        } => {
            let obj_paths = obj_paths.join(" ");
            let v = serde_json::from_str::<Vec<String>>(&obj_paths)
                .with_context(|| format!("Expected JSON data in the form \"[<Object path>, ...]\", got {obj_paths}"))?;
            serialize_into_vec(&usp_builder::GetInstancesBuilder::new().with_first_level_only(first_level_only).with_obj_paths(v).build()?
        )}
        MsgType::USPGetSupportedDM {
            first_level_only,
            return_commands,
            return_events,
            return_params,
            return_unique_key_sets,
            paths,
        } => {
            let v = serde_json::from_str::<Vec<String>>(&paths.join(" "))
                .with_context(|| format!("Expected JSON data in the form \"[<Object path>, ...]\", got {paths:?}"))?;
            let msg = usp_builder::GetSupportedDMBuilder::new()
                .with_first_level_only(first_level_only)
                .with_return_commands(return_commands)
                .with_return_events(return_events)
                .with_return_params(return_params)
                .with_return_unique_key_sets(return_unique_key_sets)
                .with_obj_paths(v)
                .build()?;
            serialize_into_vec(&msg)
        }
        MsgType::USPGetSupportedProtocol { cspv } => {
            let msg = usp_builder::GetSupportedProtocolBuilder::new(cspv)
                .build()?;
            serialize_into_vec(&msg)
        }
        MsgType::USPGetResp { result } => {
            let result = result.join(" ");
            let getresp_json: Vec<(String, u32, String, Vec<(String, HashMap<String, String>)>)> = serde_json::from_str(&result)?;

            let mut getrespb = usp_builder::GetRespBuilder::new();
            for req_path_result in getresp_json {
                let mut reqpathbuilder = usp_builder::GetReqPathResultBuilder::new(req_path_result.0);
                if req_path_result.1 != 0
                {
                    reqpathbuilder.err_code = req_path_result.1;
                }
                for res_path_result in req_path_result.3 {
                    let respathbuilder = usp_builder::ResolvedPathResultBuilder::new(res_path_result.0).with_result_params(res_path_result.1.into_iter().collect());
                    reqpathbuilder = reqpathbuilder.with_res_path_results(vec![respathbuilder]);
                }
                getrespb = getrespb.with_req_path_results(vec![reqpathbuilder]);
            }

            serialize_into_vec(&getrespb.build()?)
        }
        MsgType::USPNotify {
            sub_id,
            send_resp,
            typ,
        } => {
            let mut notify = usp_builder::NotifyBuilder::new(sub_id).with_send_resp(send_resp);
            notify = match typ {
                NotifyType::OnBoardRequest { oui, product_class, serial_number, agent_supported_protocol_versions } => notify.with_onboard_request(oui, product_class, serial_number, agent_supported_protocol_versions),
                NotifyType::ValueChange { param_path, param_value } => notify.with_value_change(param_path, param_value),
                NotifyType::Event { obj_path, event_name, params } => notify.with_event(obj_path, event_name, params),
                NotifyType::ObjectCreation { obj_path, unique_keys } => notify.with_object_creation(obj_path, unique_keys),
                NotifyType::ObjectDeletion { obj_path } => notify.with_object_deletion(obj_path),
                NotifyType::OperationComplete { obj_path, command_name, command_key, operation_resp } => match operation_resp {
                    OperateResponse::OutputArgs(output_args) => notify.with_operation_complete_output_args(obj_path, command_name, command_key, output_args),
                    OperateResponse::CommandFailure(err_code, err_msg) => notify.with_operation_complete_cmd_failure(obj_path, command_name, command_key, err_code, err_msg),
                }
            };

            serialize_into_vec(&notify.build()?)
        }
        ,
        MsgType::USPNotifyResp { sub_id } => {
            let msg = usp_builder::NotifyRespBuilder::new(sub_id).build()?;
            serialize_into_vec(&msg)
        }
        MsgType::USPOperate {
            command,
            command_key,
            send_resp,
            args,
        } => {
            let v = if args.is_empty() {
                Vec::new()
            } else {
                let args = args.join(" ");
                serde_json::from_str::<Vec<(String, String)>>(&args)
                .with_context(|| format!("Expected JSON data in the form \"[[<Argument name>, <Argument value>], ...]\", got {args}"))?
            };
            serialize_into_vec(&usp_builder::OperateBuilder::new(command).with_command_key(command_key).with_send_resp(send_resp).with_input_args(v).build()?)
        }
        MsgType::USPSet {
            allow_partial,
            args,
        } => {
            let args = args.join(" ");
            let v = serde_json::from_str::<Vec<(&str, Vec<(String, String, bool)>)>>(&args)
                .with_context(|| format!("Expected JSON data in the form \"[[<Object path>, [[<Parameter name>, <Parameter value>, <Required>], ...]], ...]\", got {args}"))?;
            let msg = usp_builder::SetBuilder::new()
                .with_allow_partial(allow_partial)
                .with_update_objs(v.into_iter().map(|(path, par)| usp_builder::UpdateObjectBuilder::new(path.into()).with_param_settings(par)).collect())
                .build()?;
            serialize_into_vec(&msg)
        }
        MsgType::USPRegister { allow_partial, reg_paths } => {
            let msg = usp_builder::RegisterBuilder::new()
                .with_allow_partial(allow_partial)
                .with_reg_paths(reg_paths)
                .build()?;
            serialize_into_vec(&msg)
        }
        MsgType::USPDeregister { paths } => {
            let msg = usp_builder::DeregisterBuilder::new()
                .with_paths(paths)
                .build()?;
            serialize_into_vec(&msg)
        }
    }.context("While trying to encode message to ProtoBuf")
}

fn get_out_stream(filename: Option<PathBuf>) -> Result<Box<dyn Write>> {
    if let Some(filename) = filename {
        return Ok(Box::new(File::create(filename)?));
    }

    Ok(Box::new(stdout()))
}

/// Write the given USP Msg to the output stream in the specified format
fn write_msg<W: Write>(msg: &rusp::usp::Msg, mut out: W, format: &OutputFormat) -> Result<()> {
    match format {
        OutputFormat::Json => {
            out.write_all(&msg.to_json_pretty()?.into_bytes())?;
            writeln!(out)
        }
        OutputFormat::CStr => out.write_all(&msg.to_c_str()?.into_bytes()),
        OutputFormat::CArray => out.write_all(&msg.to_c_array()?.into_bytes()),
        OutputFormat::Protobuf => out.write_all(&msg.to_vec()?),
    }?;

    Ok(())
}

/// Write the given USP Record to the output stream in the specified format
fn write_record<W: Write>(
    record: &rusp::usp_record::Record,
    mut out: W,
    format: &OutputFormat,
) -> Result<()> {
    match format {
        OutputFormat::Json => {
            out.write_all(&record.to_json_pretty()?.into_bytes())?;
            writeln!(out)
        }
        OutputFormat::CStr => out.write_all(&record.to_c_str()?.into_bytes()),
        OutputFormat::CArray => out.write_all(&record.to_c_array()?.into_bytes()),
        OutputFormat::Protobuf => out.write_all(&record.to_vec()?),
    }?;

    Ok(())
}

fn encode_msg(
    msgid: &str,
    filename: Option<PathBuf>,
    typ: MsgType,
    format: &OutputFormat,
) -> Result<()> {
    use quick_protobuf::deserialize_from_slice;

    let encoded_body = encode_msg_body_buf(typ)?;
    let body: rusp::usp::Body =
        deserialize_from_slice(&encoded_body).context("Failed trying to deserialise Msg body")?;
    let msg = usp_builder::MsgBuilder::new()
        .with_msg_id(msgid.into())
        .with_body(body)
        .build()?;

    // Open the specified file (or stdout) as output stream and write the USP Msg to it
    write_msg(&msg, get_out_stream(filename)?, format)
}

fn extract_msg(in_file: &Path, out_file: &Path, format: &OutputFormat) -> Result<()> {
    use rusp::usp_record::mod_Record::OneOfrecord_type;

    let fp = File::open(in_file)?;
    let mut buf_reader = BufReader::new(fp);
    let mut contents = Vec::new();
    buf_reader.read_to_end(&mut contents)?;

    let record = try_decode_record(&contents)?;

    match record.record_type {
        OneOfrecord_type::no_session_context(context) => {
            let msg = try_decode_msg(&context.payload)?;
            // Open output stream
            let out = get_out_stream(Some(out_file.to_path_buf()))?;
            write_msg(&msg, out, format)?;
        }
        OneOfrecord_type::session_context(_)
        | OneOfrecord_type::websocket_connect(_)
        | OneOfrecord_type::mqtt_connect(_)
        | OneOfrecord_type::stomp_connect(_)
        | OneOfrecord_type::uds_connect(_)
        | OneOfrecord_type::disconnect(_)
        | OneOfrecord_type::None => unreachable!(),
    }

    Ok(())
}

fn encode_no_session_record(
    version: String,
    from: String,
    to: String,
    filename: Option<PathBuf>,
    format: &OutputFormat,
) -> Result<()> {
    let mut msg = Vec::new();
    stdin().read_to_end(&mut msg)?;

    let record = usp_builder::RecordBuilder::new()
        .with_version(version)
        .with_to_id(to)
        .with_from_id(from)
        .with_no_session_context_payload_bytes(msg)
        .build()?;

    // Open output stream
    let out = get_out_stream(filename)?;

    write_record(&record, out, format)
}

#[allow(clippy::too_many_arguments)]
fn encode_session_record(
    version: String,
    from: String,
    to: String,
    session_id: u64,
    sequence_id: u64,
    expected_id: u64,
    retransmit_id: u64,
    filename: Option<PathBuf>,
    format: &OutputFormat,
) -> Result<()> {
    let mut msg = Vec::new();
    stdin().read_to_end(&mut msg)?;

    let sc = usp_builder::SessionContextBuilder::new()
        .with_session_id(session_id)
        .with_sequence_id(sequence_id)
        .with_expected_id(expected_id)
        .with_retransmit_id(retransmit_id)
        .with_payload(msg);

    let record = usp_builder::RecordBuilder::new()
        .with_version(version)
        .with_to_id(to)
        .with_from_id(from)
        .with_session_context_builder(sc)
        .build()?;

    // Open output stream
    let out = get_out_stream(filename)?;

    write_record(&record, out, format)
}

fn create_mqtt_connect_record(
    version: String,
    from: String,
    to: String,
    filename: Option<PathBuf>,
    mqtt311: bool,
    subscribed_topic: String,
    format: &OutputFormat,
) -> Result<()> {
    let record = usp_builder::RecordBuilder::new()
        .with_version(version)
        .with_to_id(to)
        .with_from_id(from)
        .as_mqtt_connect_record(
            if mqtt311 {
                MQTTVersion::V3_1_1
            } else {
                MQTTVersion::V5
            },
            subscribed_topic,
        )
        .build()?;

    // Open output stream
    let out = get_out_stream(filename)?;

    write_record(&record, out, format)
}

fn main() -> Result<()> {
    let args = Rusp::parse();

    // Pass on the user chosen format to use for the output
    let format = {
        if args.carray {
            OutputFormat::CArray
        } else if args.cstr {
            OutputFormat::CStr
        } else if args.protobuf {
            OutputFormat::Protobuf
        } else {
            OutputFormat::Json
        }
    };

    match args.action {
        RuspAction::DecodeRecordFiles { files } => decode_record_files(files, &format),
        RuspAction::DecodeRecord {} => decode_record_stdin(&format),
        RuspAction::DecodeMsgFiles { files } => decode_msg_files(files, &format),
        RuspAction::DecodeMsg {} => decode_msg_stdin(&format),
        RuspAction::EncodeMsg {
            msgid,
            filename,
            typ,
        } => encode_msg(&msgid, filename, typ, &format),
        RuspAction::ExtractMsg { in_file, out_file } => extract_msg(&in_file, &out_file, &format),
        RuspAction::EncodeNoSessionRecord {
            version,
            from,
            to,
            filename,
        } => encode_no_session_record(version, from, to, filename, &format),
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
            &format,
        ),
        RuspAction::CreateMQTTConnectRecord {
            version,
            from,
            to,
            mqtt311,
            subscribed_topic,
            filename,
        } => create_mqtt_connect_record(
            version,
            from,
            to,
            filename,
            mqtt311,
            subscribed_topic,
            &format,
        ),
    }?;

    Ok(())
}
