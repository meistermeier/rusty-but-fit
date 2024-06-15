use std::collections::HashMap;
use std::str::FromStr;
use serde::{Serialize, Serializer};
use serde::ser::SerializeMap;
use serde_with::serde_derive::Serialize;
use crate::data_types::Value;
use crate::fields::Field;
use crate::message_types::MessageType;

#[derive(Debug, Serialize)]
#[allow(dead_code)] // derived debug does not touch the fields
pub struct Header {
    length: u8,
    protocol_version: u8,
    profile_version: String,
    data_size: u32,
    data_type: String,
    crc: [u8;2],
}

impl Header {
    fn from(length: u8, protocol_version: u8, profile_version: String, data_size: u32, data_type: String, crc: [u8; 2]) -> Header {
       Header {
           length,
           protocol_version,
           profile_version,
           data_size,
           data_type,
           crc: <[u8; 2]>::try_from(crc).unwrap(),
       }
    }

    pub fn read_header(header_info: &[u8]) -> Header {
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
        Header::from(
            length,
            protocol_version,
            profile_version,
            data_size,
            data_type,
            crc)
    }
}

pub struct Message {
    pub message_type: MessageType,
    data: HashMap<Field, Value>,
}

impl Message {
   pub fn from( message_type: MessageType, data: HashMap<Field, Value>) -> Message {
       Message {
           message_type,
           data,
       }
   }
}

impl Clone for Message {
    fn clone(&self) -> Self {
        Message {message_type: self.message_type.clone(), data: self.data.clone()}
    }
}

impl Serialize for Message {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut map = serializer.serialize_map(Option::from(self.data.len())).unwrap();
        for entry in &self.data {
            map.serialize_key(&entry.0).unwrap();
            match entry.1 {
                Value::EnumValue(value) => {
                    if value.is_empty() {
                        map.serialize_value("").unwrap()
                    } else {
                        let enum_field_value = &u32::from(value[0]);
                        let string = (entry.0.translate_enum)(enum_field_value);
                        // todo poor man's workaround for a bad enum translation architecture
                        if string.eq("true") || string.eq("false") {
                            let bool_value:bool = FromStr::from_str(string.as_str()).unwrap();
                            map.serialize_value(&bool_value).unwrap()
                        } else {
                            map.serialize_value(&string).unwrap();
                        }
                    }
                },
                _ => map.serialize_value(&entry.1).unwrap()
            }
        }
        map.end()
    }
}