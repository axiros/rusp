use argh::FromArgs;
use rhai::packages::Package;
use rhai::{Engine, EvalAltResult, Position};
use rhai_rand::RandomPackage;
use rhai_rusp::RuspPackage;

use std::convert::Into;
use std::path::PathBuf;
use std::{fs::File, io::Read, path::Path, process::exit};

fn eprint_error(input: &str, mut err: EvalAltResult) {
    fn eprint_line(lines: &[&str], pos: Position, err_msg: &str) {
        let line = pos.line().unwrap();
        let line_no = format!("{line}: ");

        eprintln!("{line_no}{}", lines[line - 1]);

        for (i, err_line) in err_msg.to_string().lines().enumerate() {
            // Display position marker
            println!(
                "{0:>1$}{err_line}",
                if i > 0 { "| " } else { "^ " },
                line_no.len() + pos.position().unwrap() + 1,
            );
        }
        eprintln!();
    }

    let lines: Vec<_> = input.lines().collect();

    // Print error
    let pos = err.take_position();

    if pos.is_none() {
        // No position
        eprintln!("{err}");
    } else {
        // Specific position
        eprint_line(&lines, pos, &err.to_string());
    }
}

#[derive(FromArgs)]
/// the Rust USP toolkit, rhai runner
struct Rusp {
    #[argh(option, long = "script", short = 's')]
    /// inline rhai script
    script: Option<String>,

    #[argh(positional)]
    /// a filename for a Rhai script to parse
    filename: Option<PathBuf>,
}

fn main() {
    let args: Rusp = argh::from_env();

    // Initialize scripting engine
    let mut engine = Engine::new();

    // Create rusp package and add the package into the engine
    engine.register_static_module("rusp", RuspPackage::new().as_shared_module());
    engine.register_global_module(RandomPackage::new().as_shared_module());
    engine.set_optimization_level(rhai::OptimizationLevel::Simple);

    if let Some(filename) = args.filename {
        if args.script.is_some() {
            eprintln!("Inline scripting and the use of a file are mutual exclusive");
            exit(1);
        }
        let mut contents = String::new();
        let filename = match Path::new(&filename).canonicalize() {
            Err(err) => {
                eprintln!("Error script file path: {filename:?}\n{err}");
                exit(1);
            }
            Ok(f) => match f.strip_prefix(std::env::current_dir().unwrap().canonicalize().unwrap())
            {
                Ok(f) => f.into(),
                _ => f,
            },
        };

        let mut f = match File::open(&filename) {
            Err(err) => {
                eprintln!(
                    "Error reading script file: {}\n{}",
                    filename.to_string_lossy(),
                    err
                );
                exit(1);
            }
            Ok(f) => f,
        };

        if let Err(err) = f.read_to_string(&mut contents) {
            eprintln!(
                "Error reading script file: {}\n{}",
                filename.to_string_lossy(),
                err
            );
            exit(1);
        }

        let contents = if contents.starts_with("#!") {
            // Skip shebang
            &contents[contents.find('\n').unwrap_or(0)..]
        } else {
            &contents[..]
        };

        if let Err(err) = engine
            .compile(contents)
            .map_err(Into::into)
            .and_then(|mut ast| {
                ast.set_source(filename.to_string_lossy().to_string());
                engine.run_ast(&ast)
            })
        {
            let filename = filename.to_string_lossy();

            eprintln!("{:=<1$}", "", filename.len());
            eprintln!("{filename}");
            eprintln!("{:=<1$}", "", filename.len());
            eprintln!();

            eprint_error(contents, *err);
        }
    } else if let Some(contents) = args.script {
        let filename = "<script>";

        if let Err(err) = engine
            .compile(&contents)
            .map_err(Into::into)
            .and_then(|mut ast| {
                ast.set_source(filename);
                engine.run_ast(&ast)
            })
        {
            eprintln!("{:=<1$}", "", filename.len());
            eprintln!("{filename}");
            eprintln!("{:=<1$}", "", filename.len());
            eprintln!();

            eprint_error(&contents, *err);
        }
    } else {
        eprintln!("You will either have to supply a filename, or you can use the --script option");
    }
}
