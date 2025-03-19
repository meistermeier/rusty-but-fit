mod fields;
mod key_value_enum;
mod message;
mod message_types;
mod types;

use crate::data_types::Value;
use fields::{DeveloperField, Field};
use itertools::Itertools;
use message::{Header, Message};
use message_types::{FieldDefinition, MessageDefinition, MessageType};
use serde::Serialize;
use std::collections::HashMap;

/// Configuration for FIT file parsing
pub struct FitFileConfig {
    /// debug output
    pub debug: bool,
    /// include fields that are unknown to the parser
    pub include_unknown_fields: bool,
    /// include methods that are unknown to the parser
    pub include_unknown_message_types: bool,
    /// include values that are parsed invalid
    pub include_invalid_values: bool,
    /// just parse the header and return the result
    pub header_only: bool,
}

#[derive(Serialize)]
pub struct FitFile {
    pub header: Header,
    pub messages: Vec<Message>,
}

impl FitFile {
    /// returns [messages](Message) filtered by [message type](Vec<String>)
    pub fn get_messages(&self, message_types: Vec<String>) -> Vec<&Message> {
        let vec = &self.messages;
        vec.into_iter()
            .filter(|message| message_types.contains(&message.display_name()))
            .collect_vec()
    }

    pub fn get_message_types(&self) -> HashMap<String, usize> {
        let vec = &self.messages;
        vec.into_iter()
            .counts_by(|message| message.display_name().to_string())
    }

    pub fn from(buffer: &Vec<u8>, config: &FitFileConfig) -> FitFile {
        let debug = config.debug;
        let header_info = &buffer[0..14];
        let header = Header::read_header(header_info);
        if config.header_only {
            return FitFile {
                header,
                messages: vec![],
            };
        }
        let mut messages: Vec<Message> = Vec::new();
        if debug {
            println!("{:?}", header);
        }
        let mut current_position: usize = header.length;
        let mut local_message_types: HashMap<u8, MessageDefinition> = HashMap::new();
        let mut parse_configs = HashMap::new();
        let mut developer_fields = vec![];
        // do the looooooping
        loop {
            // exit on message crc
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
            // start: record header
            let header_type: u8 = buffer[current_position] >> 7 & 1;
            let developer_flag: u8 = buffer[current_position] >> 5 & 1;
            let local_message_number: u8 = buffer[current_position] & 0x0F;
            if header_type == 1 {
                panic!("!! Not implemented !! Compressed timestamp header needs special handling");
            }
            let definition_message: bool = buffer[current_position] >> 6 & 1 == 1;
            current_position += 1;
            // end: record header

            if definition_message {
                let mut fields: Vec<FieldDefinition> = vec![];
                let _reserved = buffer[current_position]; // reserved
                let endianness = buffer[current_position + 1]; // architecture
                current_position += 2; // skip the header part besides the last byte for the field number
                let type_fields: [u8; 2] = buffer[current_position..current_position + 2]
                    .try_into()
                    .unwrap();
                let local_message_type_value: u16 = if endianness == 0 {
                    u16::from_le_bytes(type_fields)
                } else {
                    u16::from_be_bytes(type_fields)
                };
                let local_message_type = MessageType::resolve(local_message_type_value);

                current_position += 2; // skip the header part besides the last byte for the field number
                let number_of_fields: u8 = buffer[current_position];
                current_position += 1;

                for i in 0..number_of_fields {
                    let i2 = (i as i32 * 3) as usize;
                    let field_definition_number = buffer[current_position + i2 + 0];
                    let field_length = buffer[current_position + i2 + 1];
                    let base_type_value = buffer[current_position + i2 + 2];
                    let field = Field::resolve_field(&local_message_type, field_definition_number);
                    let field_definition = FieldDefinition {
                        field,
                        number: field_definition_number,
                        size: field_length,
                        base_type_value_or_dev_index: base_type_value,
                    };
                    fields.push(field_definition);
                }
                current_position += number_of_fields as usize * 3;

                if developer_flag == 1 {
                    let number_of_developer_fields: u8 = buffer[current_position];
                    // println!("There are {} dev fields", number_of_developer_fields);
                    if number_of_developer_fields > 0 {
                        current_position += 1;
                        for i in 0..number_of_developer_fields {
                            let i2 = (i as i32 * 3) as usize;
                            let field_definition_number = buffer[current_position + i2 + 0];
                            let field_length = buffer[current_position + i2 + 1];
                            let dev_index = buffer[current_position + i2 + 2];
                            let field = Field::DeveloperField; //Field::resolve_field(&local_message_type, field_definition_number);

                            let dev_field_definition = FieldDefinition {
                                field,
                                number: field_definition_number,
                                size: field_length,
                                base_type_value_or_dev_index: dev_index,
                            };
                            fields.push(dev_field_definition);
                        }
                        current_position += number_of_developer_fields as usize * 3;
                    }
                }
                let definition_message = MessageDefinition {
                    message_type: local_message_type.clone(),
                    fields,
                };
                local_message_types.insert(local_message_number, definition_message);
                parse_configs.insert(local_message_number, endianness);
            } else {
                let option = local_message_types.get(&local_message_number);
                if option.is_none() {
                    panic!("What the heck is {}", local_message_number);
                }
                let definition_message = option.unwrap();
                let endianness = parse_configs.get(&local_message_number).unwrap();
                let message = definition_message.read_message(
                    &current_position,
                    buffer,
                    config,
                    endianness,
                    &developer_fields,
                );
                // hack my way into dev types
                if message.0.message_type.number == 206 {
                    // we can be sure that the dev fields are provided before they are referenced
                    let Value::NumberValueU8(developer_data_index) =
                        message.0.data.value("developer_data_index")
                    else {
                        panic!(
                            "Expected u8 got {:?}",
                            message.0.data.value("developer_data_index")
                        )
                    };
                    let Value::NumberValueU8(field_definition_number) =
                        message.0.data.value("field_definition_number")
                    else {
                        panic!(
                            "Expected u8 got {:?}",
                            message.0.data.value("developer_data_index")
                        )
                    };
                    let Value::StringValue(field_name) = message.0.data.value("field_name") else {
                        panic!(
                            "Expected string got {:?}",
                            message.0.data.value("developer_data_index")
                        )
                    };
                    let Value::NumberValueU8(fit_base_type_id) =
                        message.0.data.value("fit_base_type_id")
                    else {
                        panic!(
                            "Expected u8 got {:?}",
                            message.0.data.value("developer_data_index")
                        )
                    };
                    developer_fields.push(DeveloperField {
                        field_name: field_name.clone(),
                        field_definition_number: field_definition_number.clone(),
                        developer_data_index: developer_data_index.clone(),
                        fit_base_type_id: fit_base_type_id.clone(),
                    })
                }
                current_position = message.1;
                if !message.0.is_unknown() || config.include_unknown_message_types {
                    messages.push(message.0);
                }
            }
        }

        FitFile { header, messages }
    }
}
