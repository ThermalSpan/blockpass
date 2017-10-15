extern crate clap;
#[macro_use]
extern crate error_chain;

mod args_and_usage;

use args_and_usage::parse_args;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::prelude::*;

quick_main!(|| -> Result<()> {
    let args = parse_args();

    // We need a reader for the input file
    let input_file = File::open(&args.input).chain_err(|| {
        format!("Can't open input file: {}", args.input.display())
    })?;
    let mut input_reader = BufReader::new(input_file);

    // First lets set up the stuff we need to read in the block
    // A buffer to store the block we read in
    let mut block_buffer = String::new();
    // A buffer to store each line as we read it
    let mut line_buffer = String::new();

    // For our use case, the first line MUST be the start deliminator
    input_reader.read_line(&mut line_buffer)?;
    if chomp(&line_buffer) != args.start_deliminator {
        println!(
            "{} vs {} : wat",
            chomp(&line_buffer),
            args.start_deliminator
        );
        bail!(ErrorKind::NoStartDeliminator);
    }

    // We'll also keep the start delim unless told otherwise
    if !args.omit_start_delim {
        block_buffer.push_str(&line_buffer);
    }

    // Now we iterate through the remaining lines
    // Until we hit the end deliminator
    loop {
        line_buffer.clear();
        input_reader.read_line(&mut line_buffer)?;

        // push that line into the block_buffer
        block_buffer.push_str(&line_buffer);

        // If we see the end_delimnator we're done
        if chomp(&line_buffer) == args.end_deliminator {
            break;
        }
    }

    // For our use case, the block MUST be non empty
    if block_buffer.is_empty() {
        bail!(ErrorKind::EmptyBlock);
    }

    // The first step is to move the original file aside
    let mut original_output_path = args.output.clone();
    if !original_output_path.set_extension("orig") {
        bail!(ErrorKind::UnableToAddOrigExtension);
    }
    fs::rename(&args.output, &original_output_path).chain_err(
        || {
            format!(
                "Unable to rename\n{}\nto\n{}",
                args.output.display(),
                original_output_path.display()
            )
        },
    )?;

    // Now lets write out the original file
    let mut output_file = File::create(&args.output).chain_err(|| {
        format!("Can't create ouput file: {}", args.input.display())
    })?;

    let mut orig_file = File::open(&original_output_path).chain_err(|| {
        format!("Can't open orig file: {}", original_output_path.display())
    })?;

    // First write out the block
    output_file.write_all(block_buffer.as_bytes())?;

    // This might be our use case specific, but we need a blank line following the
    // YAML
    write!(output_file, "\n")?;

    // No we need to copy contents of orig_file to output_file
    let mut output_buffer = String::new();
    orig_file.read_to_string(&mut output_buffer).chain_err(|| {
        format!(
            "Can't read contents from {}",
            original_output_path.display()
        )
    })?;

    output_file.write_all(output_buffer.as_bytes()).chain_err(
        || {
            format!("Can't write to {}", args.output.display())
        },
    )?;

    if !args.keep_orig {
        fs::remove_file(&original_output_path).chain_err(|| {
            format!(
                "Unable to delete original {}",
                original_output_path.display()
            )
        })?;
    }

    Ok(())
});

// Use this to remove a trailing newline, if there is one
fn chomp(input: &str) -> String {
    let mut result = String::from(input);

    match result.pop() {
        None | Some('\n') => (),
        Some(c) => result.push(c),
    }

    result
}

error_chain! {
    errors {
        NoStartDeliminator {
            description("First line of input was not the start deliminator")
        }
        EmptyBlock {
            description("The block in the input file was empty")
        }
        UnableToAddOrigExtension {
            description("We were unable to add the .orig extension the ouput path")
        }
    }
    foreign_links {
        IO(std::io::Error);
    }
}
