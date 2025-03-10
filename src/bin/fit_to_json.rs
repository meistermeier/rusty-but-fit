use std::env::args;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::process::exit;
use garmin_fit::fit_file_raw::FitFileRaw;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        eprintln!("You must provide exactly one argument (the file to parse)");
        eprintln!("Exit 1");
        exit(1);
    }
    let file_name = &args[1];
    let file = File::open(file_name);
    if file.is_err() {
        eprintln!("File {} cannot be read.", file_name);
        eprintln!("Exit 2");
        exit(2);
    }

    let f = file.unwrap();
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();

    reader.read_to_end(&mut buffer).unwrap();

    let fit_file = FitFileRaw::from(&buffer);
    println!("{}", serde_json::to_string(&fit_file.messages).unwrap());
}
