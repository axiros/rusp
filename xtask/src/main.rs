use std::path::PathBuf;

use argh::FromArgs;

/// xtask style utilities
#[derive(FromArgs)]
struct XTask {
    #[argh(subcommand)]
    commands: SubCommands,
}

#[derive(FromArgs)]
#[argh(subcommand)]
enum SubCommands {
    ProstBuild(ProstBuild),
}

/// Build the provided protobuf file to rust code using the prost-build crate
#[derive(FromArgs)]
#[argh(subcommand, name = "prost-build")]
struct ProstBuild {
    /// input protobuf file
    #[argh(option, short = 'i')]
    input: PathBuf,

    /// output directory to save the rust file created from `--input`. Its name will be the package
    /// name field of the protobuf file but with the .rs extension
    #[argh(option, short = 'o')]
    out_dir: PathBuf,
}

fn main() -> std::io::Result<()> {
    let args = argh::from_env::<XTask>();
    let (input, out_dir) = match args.commands {
        SubCommands::ProstBuild(ProstBuild { input, out_dir }) => (input, out_dir),
    };

    println!("Compiling: {}", input.display());
    prost_build::Config::new()
        .out_dir(out_dir)
        .type_attribute(".", "#[non_exhaustive]")
        .compile_protos(&[input], &[] as &[PathBuf])
}
