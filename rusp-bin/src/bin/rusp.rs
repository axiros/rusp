#![allow(clippy::type_complexity)]

use rusp_lib as rusp;

use clap::{Parser, Subcommand};
use rusp::usp_builder;
use rusp::usp_record::mod_MQTTConnectRecord::MQTTVersion;
use std::collections::HashMap;
use std::fs::File;
use std::io::{stdin, stdout, BufReader, Read, Write};
use std::path::{Path, PathBuf};

use anyhow::Result;

use rusp::usp_decoder::{try_decode_msg, try_decode_record};

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
    /// Not available anymore since 0.96
    #[command(name = "encode_msg")]
    EncodeMsg {
        /// The message ID to use in the USP Msg header
        _msgid: String,
        /// Filename (will output to standard output if omitted)
        #[arg(short = 'f', long = "file")]
        /// Output filename of file to encode USP Protobuf message to
        _filename: Option<PathBuf>,
        #[arg(required = true)]
        _command: Vec<String>,
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

fn get_out_stream(filename: Option<PathBuf>) -> Result<Box<dyn Write>> {
    if let Some(filename) = filename {
        Ok(Box::new(File::create(filename)?))
    } else {
        Ok(Box::new(stdout()))
    }
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
            _msgid,
            _filename,
            _command,
        } => Err(anyhow::anyhow!(
            "Support for encoding messages has been removed in rusp 0.96, use the new rusp-run command instead"
        )),
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