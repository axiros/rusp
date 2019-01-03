use quick_protobuf;
use serde_json;
use std::fs::File;
use std::io::{stdin, stdout, BufReader, Read, Write};
use std::path::PathBuf;
use structopt::*;

use rusp::{
    usp_decoder::{decode_msg, decode_record},
    usp_generator,
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
        /// Filename (will output to standard output if omitted)
        #[structopt(parse(from_os_str), short = "f", long = "file")]
        /// Output filename of file to encode USP protobuf message to
        filename: Option<PathBuf>,
        /// Type of message
        #[structopt(subcommand)]
        typ: MsgType,
        /// The message ID to use in the USP Msg header
        msgid: String,
    },
    /// Extract the USP message from a USP record
    #[structopt(name = "extract_msg")]
    ExtractMsg {
        #[structopt(parse(from_os_str))]
        /// Input filename of USP protobuf record to decode
        in_file: PathBuf,
        /// Output filename of USP protobuf message to write into
        #[structopt(parse(from_os_str))]
        out_file: PathBuf,
    },
    /// Extract the USP message body from a USP record
    #[structopt(name = "extract_msg_body")]
    ExtractMsgBody {
        #[structopt(parse(from_os_str))]
        /// Input filename of USP protobuf record to decode
        in_file: PathBuf,
        /// Output filename of USP protobuf message body to write into
        #[structopt(parse(from_os_str))]
        out_file: PathBuf,
    },
}

#[derive(StructOpt, Debug)]
enum MsgType {
    /// Generate a USP Get request message
    Get {
        /// A JSON array of Strings resembling the paths for the Get operation
        #[structopt(multiple = true)]
        paths: Vec<String>,
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

fn encode_msg(msgid: &str, filename: Option<PathBuf>, typ: &MsgType) {
    use quick_protobuf::{message::MessageWrite, Writer};

    let mut buf = Vec::new();
    let mut writer = Writer::new(&mut buf);
    let v: Vec<String>;

    let msg = usp_generator::usp_msg(
        &msgid,
        match typ {
            MsgType::Get { ref paths } => {
                v = serde_json::from_str(&paths.join(" ")).unwrap();
                usp_generator::usp_get_request(v.iter().map(std::ops::Deref::deref).collect())
            }
        },
    );
    msg.write_message(&mut writer)
        .expect("Cannot encode message");

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

fn main() {
    let opt = Rusp::from_args();

    match opt {
        Rusp::DecodeRecordFiles { files } => decode_record_files(files),
        Rusp::DecodeRecord {} => decode_record_stdin(),
        Rusp::DecodeMsgFiles { files } => decode_msg_files(files),
        Rusp::DecodeMsg {} => decode_msg_stdin(),
        Rusp::EncodeMsg {
            msgid,
            filename,
            typ,
        } => encode_msg(&msgid, filename, &typ),
        Rusp::ExtractMsg { in_file, out_file } => extract_msg(&in_file, &out_file),
        Rusp::ExtractMsgBody { in_file, out_file } => extract_msg_body(&in_file, &out_file),
    }
}
