mod types;
mod message_types;
mod fields;

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use fields::Field;
use types::BaseType;
use crate::types::Value;

struct Header {
    length: u8,
    protocol_version: u8,
    profile_version: String,
    data_size: u32,
    data_type: String,
    crc: [u8; 2],
}

struct DefinitionMessage {
    fields: Vec<FieldDefinition>,
    message_type: message_types::MessageType,
}

struct DataMessage {}

struct FieldDefinition {
    number: u8,
    size: u8,
    base_type: BaseType,
}

impl FieldDefinition {
    pub(crate) fn read(&self, data: &[u8]) -> Value {
        let i = self.number;

        Value::Undefined
    }
}

fn main() {
    let f = File::open("activity2.fit").unwrap();
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();

    // Read file into vector.
    reader.read_to_end(&mut buffer).unwrap();
    read_content(&buffer);
}

fn read_content(buffer: &Vec<u8>) {
    let header_info = &buffer[0..14];
    let header = read_header(header_info);
    println!("Header -> Length: {}, Protocol: {}, Profile: {}, Data size: {}, Data type: {}, crc (raw): {:?}",
             header.length,
             header.protocol_version,
             header.profile_version,
             header.data_size,
             header.data_type,
             header.crc
    );
    let mut current_position = 14; // after header
    let mut local_message_types = HashMap::new();
    loop {
        // do the looooooping
        if current_position == buffer.len() - 2 {
            let crc = &header.crc;
            println!("CRC check: {:#04x} equals {:#04x} and {:#04x} equals {:#04x}", crc.get(0).unwrap(), buffer[current_position], crc.get(1).unwrap(), buffer[current_position + 1]);
            println!("Something for later");
            break;
        }
        let header_type: u8 = buffer[current_position] >> 7 & 1;
        let developer_flag: u8 = buffer[current_position] >> 5 & 1;
        let local_message_number: u8 = buffer[current_position] & 0x0F;
        // let irgendwas: u16 =
        //     (buffer[current_position] & 1 + buffer[current_position] >> 1 & 1 + buffer[current_position] >> 2 & 1 + buffer[current_position] >> 3 & 1) as u16;
        // println!("\tirgendwas: {}", irgendwas);
        if header_type == 1 {
            println!("!!Compressed timestamp header needs special handling");
        }
        let message_type: u8 = buffer[current_position] >> 6 & 1;
        // println!("Message type {} at {:#04x} with value {}", if message_type == 1 { "definition" } else { "data" }, current_position, buffer[current_position] >> 6 & 1);
        current_position += 1;

        if message_type == 1 {
            let mut fields_length: i32 = 0;
            let mut fields: Vec<FieldDefinition> = vec![];
            current_position += 2; // skip the header part besides the last byte for the field number
            let type_f1 = buffer[current_position];
            let type_f2 = buffer[current_position + 1];
            let local_message_type_value: u16 =
                type_f1 as u16 + type_f2 as u16;
            let definition_type = message_types::MessageType::parse(local_message_type_value);
            println!("Local message type {:?} ({})", definition_type.name, local_message_type_value);

            current_position += 2; // skip the header part besides the last byte for the field number
            let number_of_fields: u8 = buffer[current_position];
            current_position += 1;

            for i in 0..number_of_fields {
                let i2 = (i as i32 * 3) as usize;
                let field_definition_number = buffer[current_position + i2 + 0];
                //println!("\t field definition number {}", field_definition_number);
                let field_length = buffer[current_position + i2 + 1];// as i32;
                let base_type_value = buffer[current_position + i2 + 2];
                let field = FieldDefinition {
                    number: field_definition_number,
                    size: field_length,
                    base_type: BaseType::parse(base_type_value),
                };
                fields.push(field);
                fields_length += field_length as i32;
                // println!("Base type value {} / {:#04x} with length {}", base_type_value, base_type_value, field_length);
            }
            current_position += number_of_fields as usize * 3;

            if developer_flag == 1 {
                let number_of_developer_fields: u8 = buffer[current_position];
                if number_of_developer_fields > 0 {
                    println!("I have dev fields");
                    current_position += 1;
                }
            }
            let definition_message = DefinitionMessage {
                message_type: definition_type,
                fields,
            };
            local_message_types.insert(local_message_number, definition_message);
            // println!("definition done. Setting {:#04x} to length {}", local_message_number, number_of_fields);
        } else {
            let definition_message = local_message_types.get(&local_message_number).unwrap().clone();
            let definition_message_type = &definition_message.message_type;
            let mut skipped_fields: Vec<Field> = vec![];
            for field in definition_message.fields.iter().clone() {
                let end = current_position + (field.size as usize);
                let data = &buffer[current_position..end];
                let value = ((field.base_type).read)(&field.base_type, data);
                let data_field = fields::Field::parse(field.number, definition_message_type);
                current_position += field.size as usize;
                if data_field == Field::UNKNOWN && definition_message_type.number != 12 {
                    skipped_fields.push(data_field);
                    continue;
                }
                match value {
                    Value::NumberValueU64(value) => {
                        println!("{} (type: {} / number: {}) with value {:?}", data_field.name, field.base_type.name, field.number, value);
                    }
                    Value::NumberValueU8(value) => {
                        println!("{} (type: {} / number: {}) with value {:?}", data_field.name, field.base_type.name, field.number, value);
                    }
                    Value::NumberValueVecU8(value) => {
                        if field.base_type.name == "enum" && !value.is_empty() {
                            let enum_value = Value::StringValue((data_field.translate_enum)(&data_field, value.get(0).unwrap()));
                            println!("{} (type: {} / number: {}) with value {:?}", data_field.name, field.base_type.name, field.number, enum_value);
                        } else {
                            println!("{} (type: {} / number: {}) with value {:?}", data_field.name, field.base_type.name, field.number, value);
                        }
                    }
                    Value::NumberValueU16(value) => {
                        println!("{} (type: {} / number: {}) with value {:?}", data_field.name, field.base_type.name, field.number, value);
                    }
                    Value::NumberValueVecU16(value) => {
                        println!("{} (type: {} / number: {}) with value {:?}", data_field.name, field.base_type.name, field.number, value);
                    }
                    Value::NumberValueU32(value) => {
                        println!("{} (type: {} / number: {}) with value {:?}", data_field.name, field.base_type.name, field.number, value);
                    }
                    Value::NumberValueVecU32(value) => {
                        println!("{} (type: {} / number: {}) with value {:?}", data_field.name, field.base_type.name, field.number, value);
                    }
                    Value::StringValue(value) => {
                        println!("{} (type: {} / number: {}) with value {:?}", data_field.name, field.base_type.name, field.number, value);
                    }
                    Value::Undefined => {
                        //println!("Ignoring value for field {}", data_field.name);
                    }
                }
            }
            if skipped_fields.len() > 0 {
                // println!("Skipped {} unknown fields: {:?}", skipped_fields.len(), skipped_fields);
            }
            // println!("data done. read from {:#04x} and now am at {:#04x}", local_message_number, current_position);
        }
        // only parse the first definition and data message
        // if message_type != 1 {
        //     break;
        // }
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
