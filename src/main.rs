#![recursion_limit = "1000"]

#[macro_use]
extern crate derive_builder;

use clap::{Parser, Subcommand};
use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use serde::{Serialize, Serializer};
use serde_with::serde_derive::Serialize;

use crate::data_types::{BaseType, Value};
use crate::fields::Field;
use crate::message_types::{FieldDefinition, MessageDefinition, MessageType};

mod data_types;
mod fields;
mod key_value_enum;
mod message_types;
mod types;

#[derive(Parser)]
#[command(name = "Garmin FIT parser")]
#[command(version = "0.1")]
#[command(about = "Parsing for Garmin's FIT file format", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[arg(short, long, value_name = "FILE", help = "FIT file to parse")]
    file: String,
    #[arg(short)]
    debug: bool,
    #[arg(short, long)]
    message_type: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    Summary,
    Messages,
    Dump
}

struct Header {
    length: u8,
    protocol_version: u8,
    profile_version: String,
    data_size: u32,
    data_type: String,
    crc: [u8; 2],
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

    // Read file into vector.
    reader.read_to_end(&mut buffer).unwrap();
    let info = read_content(&buffer, args.debug);
    match args.command {
        Commands::Summary => {
            println!("{:?}", info.get_message_types());
        },
        Commands::Messages => {
            let result = info.get_messages(args.message_type.unwrap().as_str());
            println!("{}", serde_json::to_string(&result).unwrap());
        },
        Commands::Dump => {
            println!("{}", serde_json::to_string(&info).unwrap());
        }
    }
}

#[derive(Serialize)]
struct Message {
    message_type: MessageType,
    data: HashMap<Field, Value>,
}

impl Message {
    pub fn get_value(&self, field_name: &str) -> Option<&Value> {
        self.data
            .iter()
            .find(|(field, value)| field.name.eq(field_name))
            .and_then(|(_, v)| Option::from(v))
    }
}

impl Debug for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entries(&self.data).finish()
    }
}

impl Clone for Message {
    fn clone(&self) -> Self {
        Message {message_type: self.message_type.clone(), data: self.data.clone()}
    }
}

fn read_content(buffer: &Vec<u8>, debug: bool) -> Info {
    let header_info = &buffer[0..14];
    let header = read_header(header_info);
    let mut messages: Vec<Message> = Vec::new();
    if debug {
        println!("Header -> Length: {}, Protocol: {}, Profile: {}, Data size: {}, Data type: {}, crc (raw): {:?}",
                 header.length,
                 header.protocol_version,
                 header.profile_version,
                 header.data_size,
                 header.data_type,
                 header.crc
        );
    }
    let mut current_position = 14; // after header
    let mut local_message_types = HashMap::new();
    loop {
        // do the looooooping
        if current_position == buffer.len() - 2 {
            if debug {
                println!("debug");
                let crc = &header.crc;
                println!(
                    "CRC check: {:#04x} equals {:#04x} and {:#04x} equals {:#04x}",
                    crc.get(0).unwrap(),
                    buffer[current_position],
                    crc.get(1).unwrap(),
                    buffer[current_position + 1]
                );
                println!("Something for later");
            }
            break;
        }
        let header_type: u8 = buffer[current_position] >> 7 & 1;
        let developer_flag: u8 = buffer[current_position] >> 5 & 1;
        let local_message_number: u8 = buffer[current_position] & 0x0F;
        if header_type == 1 {
            println!("!!Compressed timestamp header needs special handling");
        }
        let parsed_message_type: u8 = buffer[current_position] >> 6 & 1;
        current_position += 1;

        // definition message
        if parsed_message_type == 1 {
            let mut fields: Vec<FieldDefinition> = vec![];
            current_position += 2; // skip the header part besides the last byte for the field number
            let type_f1 = buffer[current_position];
            let type_f2 = buffer[current_position + 1];
            let local_message_type_value: u16 = type_f1 as u16 + type_f2 as u16;
            let local_message_type = MessageType::resolve(local_message_type_value);
            //println!("Local message type {:?} ({})", local_message_type.name, local_message_type_value);

            current_position += 2; // skip the header part besides the last byte for the field number
            let number_of_fields: u8 = buffer[current_position];
            current_position += 1;

            for i in 0..number_of_fields {
                let i2 = (i as i32 * 3) as usize;
                let field_definition_number = buffer[current_position + i2 + 0];
                //println!("\t field definition number {}", field_definition_number);
                let field_length = buffer[current_position + i2 + 1]; // as i32;
                let base_type_value = buffer[current_position + i2 + 2];
                let field = FieldDefinition {
                    field: Field::resolve(&local_message_type, field_definition_number),
                    number: field_definition_number,
                    size: field_length,
                    base_type: BaseType::parse(base_type_value),
                };
                fields.push(field);
            }
            current_position += number_of_fields as usize * 3;

            if developer_flag == 1 {
                let number_of_developer_fields: u8 = buffer[current_position];
                if number_of_developer_fields > 0 {
                    println!("I have dev fields");
                    current_position += 1;
                }
            }
            let definition_message = MessageDefinition {
                message_type: local_message_type.clone(),
                fields,
            };
            local_message_types.insert(local_message_number, definition_message);
        } else {
            let definition_message = local_message_types.get(&local_message_number).unwrap();

            let message = definition_message.read(&current_position, buffer);
            current_position = message.1;
            messages.push(message.0);
        }
    }

    Info { messages }
}

#[derive(Serialize)]
struct Result {
    name: String,
    messages: Vec<Message>,
}

// todo name tbd
#[derive(Serialize)]
struct Info {
    messages: Vec<Message>,
}

impl Info {
    pub fn get_messages(&self, message_type: &str) -> Result {
        let vec = &self.messages;
        let mut new_messages:Vec<Message> = Vec::new();
        vec.into_iter()
            .filter(|message| message.message_type.name.eq(message_type))
            .for_each(|message| new_messages.push(message.clone()));
        let name = message_type.to_string();
        Result { name, messages: new_messages}
    }

    pub fn get_message_types(&self) -> HashMap<String, usize> {
        let vec = &self.messages;
        vec.into_iter()
            .counts_by(|message| message.message_type.name.to_string())
    }
}

fn read_header(header_info: &[u8]) -> Header {
    let length = *header_info.get(0).unwrap();
    let protocol_version = *header_info.get(1).unwrap() >> 4;

    let profile_field1: u16 = u16::from(*header_info.get(2).unwrap());
    let profile_field2: u16 = u16::from(*header_info.get(3).unwrap()) << 8;
    let profile_version_value = profile_field1 + profile_field2;
    let first_part = profile_version_value / 100;
    let second_part = profile_version_value % 100;
    let profile_version = format!("{}.{}", first_part, second_part);

    let data_size = u32::from(*header_info.get(4).unwrap())
        + (u32::from(*header_info.get(5).unwrap()) << 8)
        + (u32::from(*header_info.get(6).unwrap()) << 16)
        + (u32::from(*header_info.get(7).unwrap()) << 24);

    let data_type = format!(
        "{}{}{}{}",
        (*header_info.get(8).unwrap() as char).to_string(),
        (*header_info.get(9).unwrap() as char).to_string(),
        (*header_info.get(10).unwrap() as char).to_string(),
        (*header_info.get(11).unwrap() as char).to_string()
    );

    let crc: [u8; 2] = [
        header_info.get(13).unwrap().clone(),
        header_info.get(12).unwrap().clone(),
    ];
    // .try_into()
    // .expect("slice with incorrect length");
    Header {
        length,
        protocol_version,
        profile_version,
        data_size,
        data_type,
        crc,
    }
}
