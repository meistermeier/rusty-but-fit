mod message_raw;

use crate::fit_file_raw::message_raw::{FieldDefinitionRaw, MessageDefinitionRaw, MessageRaw};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct FitFileRaw {
    pub messages: Vec<MessageRaw>,
}

impl FitFileRaw {
    pub fn from(buffer: &Vec<u8>) -> FitFileRaw {
        let header_info = &buffer[0..14];
        let header_length = header_length(header_info);
        let mut messages: Vec<MessageRaw> = Vec::new();
        let mut current_position: usize = header_length;
        let mut local_message_types: HashMap<u8, MessageDefinitionRaw> = HashMap::new();
        let mut parse_configs = HashMap::new();
        loop {
            // exit on message crc
            if current_position == buffer.len() - 2 {
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
                let mut fields: Vec<FieldDefinitionRaw> = vec![];
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

                current_position += 2; // skip the header part besides the last byte for the field number
                let number_of_fields: u8 = buffer[current_position];
                current_position += 1;

                for i in 0..number_of_fields {
                    let field_index = (i as i32 * 3) as usize;
                    let field_definition_number = buffer[current_position + field_index + 0];
                    let field_length = buffer[current_position + field_index + 1];
                    let base_type_value = buffer[current_position + field_index + 2];
                    let field_definition = FieldDefinitionRaw {
                        number: field_definition_number,
                        size: field_length,
                        base_type_value_or_dev_index: base_type_value,
                        is_dev_field: false,
                    };
                    fields.push(field_definition);
                }
                current_position += number_of_fields as usize * 3;

                if developer_flag == 1 {
                    let number_of_developer_fields: u8 = buffer[current_position];
                    if number_of_developer_fields > 0 {
                        current_position += 1;
                        for i in 0..number_of_developer_fields {
                            let field_index = (i as i32 * 3) as usize;
                            let field_definition_number = buffer[current_position + field_index + 0];
                            let field_length = buffer[current_position + field_index + 1];
                            let dev_index = buffer[current_position + field_index + 2];

                            let dev_field_definition = FieldDefinitionRaw {
                                number: field_definition_number,
                                size: field_length,
                                base_type_value_or_dev_index: dev_index,
                                is_dev_field: true,
                            };
                            fields.push(dev_field_definition);
                        }
                        current_position += number_of_developer_fields as usize * 3;
                    }
                }
                let definition_message = MessageDefinitionRaw {
                    message_type_number: local_message_type_value,
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
                    endianness,
                );
                current_position = message.1;
                messages.push(message.0);
            }
        }

        FitFileRaw { messages }
    }
}

fn header_length(header_info: &[u8]) -> usize {
    *header_info.get(0).unwrap() as usize
}
