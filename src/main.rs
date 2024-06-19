use std::fs::File;
use std::io::BufReader;
use std::io::Read;

use clap::{Parser, Subcommand};

use crate::fit_file::{FitFile, FitFileConfig};
use crate::message::Message;

mod data_types;
mod fields;
mod key_value_enum;
mod message_types;
mod types;
mod message;
mod fit_file;

// CLI / clap definitions
#[derive(Parser)]
#[command(name = "Garmin FIT parser")]
#[command(version = "0.1")]
#[command(about = "Parsing for Garmin's FIT file format", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[arg(short, long, value_name = "FILE", help = "FIT file to parse")]
    file: String,
    #[arg(short, help = "Debug output (cannot be piped to jq)")]
    debug: bool,
    #[arg(short, long, help = "Message type as enumerated from 'summary' command")]
    message_type: Option<String>,
    #[arg(short, long, help = "Output unknown fields")]
    unknown_fields: bool,
    #[arg(short, long, help = "Output invalid values")]
    invalid_values: bool,
}

#[derive(Subcommand)]
enum Commands {
    Summary,
    Messages,
    Dump
}

fn main() {
    let args = Cli::parse();

    let file_name = &args.file;
    let file = File::open(file_name);
    if file.is_err() {
        panic!("File {} cannot be read.", file_name);
    }

    let f = file.unwrap();
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();

    reader.read_to_end(&mut buffer).unwrap();
    let fit_file_config = FitFileConfig {
        debug: args.debug,
        include_unknown_fields: args.unknown_fields,
        include_invalid_fields: args.invalid_values,
    };
    let fit_file = FitFile::from(&buffer, &fit_file_config);
    match args.command {
        Commands::Summary => {
            println!("{:?}", fit_file.get_message_types());
        },
        Commands::Messages => {
            let result = fit_file.get_messages(args.message_type.unwrap().as_str());
            println!("{}", serde_json::to_string(&result).unwrap());
        },
        Commands::Dump => {
            println!("{}", serde_json::to_string(&fit_file).unwrap());
        }
    }
}
