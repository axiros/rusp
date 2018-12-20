use quick_protobuf;
use std::fs::File;
use std::io::{stdin, BufReader, Read};
use std::path::PathBuf;
use structopt::*;

mod usp;
mod usp_decoder;
mod usp_formatter;
mod usp_generator;
mod usp_record;

use self::usp_decoder::*;

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
    #[structopt(name = "extract_msg")]
    ExtractMsg {
        #[structopt(parse(from_os_str))]
        /// Input filenames of USP protobuf record to decode
        in_file: PathBuf,
        /// Output filenames of USP protobuf message to write into
        #[structopt(parse(from_os_str))]
        out_file: PathBuf,
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

fn extract_msg(in_file: &PathBuf, out_file: &PathBuf) {
    use self::usp_record::mod_Record::OneOfrecord_type::*;

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

fn main() {
    let opt = Rusp::from_args();

    match opt {
        Rusp::DecodeRecordFiles { files } => decode_record_files(files),
        Rusp::DecodeRecord {} => decode_record_stdin(),
        Rusp::DecodeMsgFiles { files } => decode_msg_files(files),
        Rusp::DecodeMsg {} => decode_msg_stdin(),
        Rusp::ExtractMsg { in_file, out_file } => extract_msg(&in_file, &out_file),
    }
}
