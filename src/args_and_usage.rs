use clap::{Arg, App};
use std::path::PathBuf;

// Programmer defined constants
static PROGRAM_NAME: &'static str = "blockpass";

// Derived constants
static VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Debug)]
pub struct Args {
    pub input: PathBuf,
    pub output: PathBuf,
    pub keep_orig: bool,
    pub omit_start_delim: bool,
    pub start_deliminator: String,
    pub end_deliminator: String,
}

pub fn parse_args() -> Args {
    let args = App::new(PROGRAM_NAME)
        .version(VERSION)
        .author("Russell W. Bentley <russell.w.bentley@icloud.com>")
        .about("A tool for passing blocks from one file to another")
        .arg(Arg::with_name("INPUT")
            .help("The file to retreive the block from")
            .long("input")
            .value_name("input/file.ex")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("OUTPUT")
            .help("The file to prepend the block to")
            .long("output")
            .value_name("output/fie.ex")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("KEEP_ORIG")
            .help("Defualt behaivor is to delete original outout file")
            .long("keep-orig"))
        .arg(Arg::with_name("OMIT_START_DELIM")
            .help("Do not copy start deliminator to output")
            .long("omit-start-delim"))
        .arg(Arg::with_name("START_DELIMINATOR")
            .help("The deliminator that starts the block")
            .long("start-delim")
            .default_value("---"))
        .arg(Arg::with_name("END_DELIMINATOR")
            .help("The deliminator that ends the block")
            .long("end-delim")
            .default_value("---"))
        .get_matches();

    let input_path_raw = args.value_of("INPUT").unwrap();
    let input_path = PathBuf::from(input_path_raw);

    let output_path_raw = args.value_of("OUTPUT").unwrap();
    let output_path = PathBuf::from(output_path_raw);

    Args {
        input: input_path,
        output: output_path,
        keep_orig: args.is_present("KEEP_ORIG"),
        omit_start_delim: args.is_present("OMIT_START_DELIM"),
        start_deliminator: String::from(args.value_of("START_DELIMINATOR").unwrap()),
        end_deliminator: String::from(args.value_of("END_DELIMINATOR").unwrap()),
    }
}


