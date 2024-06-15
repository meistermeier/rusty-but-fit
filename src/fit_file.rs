use std::collections::HashMap;
use itertools::Itertools;
use serde_with::serde_derive::Serialize;
use crate::Cli;
use crate::data_types::BaseType;
use crate::fields::Field;
use crate::message::{Header, Message};
use crate::message_types::{FieldDefinition, MessageDefinition, MessageType};

#[derive(Serialize)]
pub struct FitFile {
    header: Header,
    messages: Vec<Message>,
}

impl FitFile {
    pub fn get_messages(&self, message_type: &str) -> Vec<&Message> {
        let vec = &self.messages;
        vec.into_iter()
            .filter(|message| message.message_type.name.eq(message_type))
            .collect_vec()
    }

    pub fn get_message_types(&self) -> HashMap<String, usize> {
        let vec = &self.messages;
        vec.into_iter()
            .counts_by(|message| message.message_type.name.to_string())
    }

    pub fn read_content(buffer: &Vec<u8>, args: &Cli) -> FitFile {
        let debug = args.debug;
        let header_info = &buffer[0..14];
        let header = Header::read_header(header_info);
        let mut messages: Vec<Message> = Vec::new();
        if debug {
            println!("{:?}", header);
        }
        let mut current_position = 14; // position after header
        let mut local_message_types = HashMap::new();
        loop {
            // do the looooooping
            if current_position == buffer.len() - 2 {
                if debug {
                    println!(
                        "CRC content: {:#04x} {:#04x}",
                        buffer[current_position],
                        buffer[current_position + 1]
                    );
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
                    let field = Field::resolve(&local_message_type, field_definition_number);
                    let field_definition = FieldDefinition {
                        field,
                        number: field_definition_number,
                        size: field_length,
                        base_type: BaseType::parse(base_type_value),
                    };
                    fields.push(field_definition);
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

                let message = definition_message.read(&current_position, buffer, args);
                current_position = message.1;
                messages.push(message.0);
            }
        }

        FitFile { header, messages }
    }
}