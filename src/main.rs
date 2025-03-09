use std::fs::File;
use std::io::BufReader;
use std::io::Read;

use clap::{Args, Parser, Subcommand};
use garmin_fit::fit_file::{FitFile, FitFileConfig};

// CLI / clap definitions
#[derive(Parser)]
#[command(name = "Garmin FIT parser")]
#[command(version = "0.5.0")]
#[command(about = "Parsing for Garmin's FIT file format", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[arg(short, long, value_name = "FILE", help = "FIT file to parse")]
    file: String,
    #[arg(short, help = "Debug output (cannot be piped to jq)")]
    debug: bool,
    #[arg(short, long, help = "Output unknown fields")]
    unknown_fields: bool,
    #[arg(long, help = "Output unknown message types")]
    unknown_message_types: bool,
    #[arg(short, long, help = "Output invalid values")]
    invalid_values: bool,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Create summary of all messages and their count")]
    Summary,
    #[command(about = "Return messages defined by the -m parameter")]
    Messages(MessagesArgs),
    #[command(about = "Outputs all messages, incl. unknown messages and invalid fields")]
    Raw,
    #[command(about = "Show parsed header")]
    Header,
}

#[derive(Args)]
struct MessagesArgs {
    #[arg(
        short,
        long = "message_type",
        value_name = "MESSAGE_TYPE",
        help = "Message types as enumerated from 'summary' command. Can be repeated for multiple messages."
    )]
    message_types: Vec<String>,
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
    let fit_file_config = match args.command {
        Commands::Raw =>
        // enforce raw jq parsable output
        // * no debug message
        // * include unknown fields and invalid values
        // * include unknown message types
        {
            FitFileConfig {
                debug: false,
                include_unknown_fields: true,
                include_unknown_message_types: true,
                include_invalid_values: true,
                header_only: false,
            }
        }
        Commands::Header => FitFileConfig {
            debug: args.debug,
            include_unknown_fields: args.unknown_fields,
            include_unknown_message_types: args.unknown_message_types,
            include_invalid_values: args.invalid_values,
            header_only: true,
        },
        _ => FitFileConfig {
            debug: args.debug,
            include_unknown_fields: args.unknown_fields,
            include_unknown_message_types: args.unknown_message_types,
            include_invalid_values: args.invalid_values,
            header_only: false,
        },
    };
    let fit_file = FitFile::from(&buffer, &fit_file_config);
    match args.command {
        Commands::Summary => println!("{:?}", fit_file.get_message_types()),
        Commands::Messages(messages_args) => {
            let result = fit_file.get_messages(messages_args.message_types);
            println!("{}", serde_json::to_string(&result).unwrap());
        }
        Commands::Raw => println!("{}", serde_json::to_string(&fit_file).unwrap()),
        Commands::Header => println!("{}", serde_json::to_string(&fit_file.header).unwrap()),
    }
}
