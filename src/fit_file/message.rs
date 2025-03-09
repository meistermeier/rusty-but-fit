use serde::ser::{SerializeMap, SerializeStruct};
use serde::Serialize;
use serde::Serializer;

use crate::data_types::Value;
use crate::fit_file::fields::Field;
use crate::fit_file::message_types::MessageType;

#[derive(Debug, Serialize)]
#[allow(dead_code)] // derived debug does not touch the fields
pub struct Header {
    pub length: usize,
    protocol_version: u8,
    profile_version: String,
    data_size: u32,
    data_type: String,
    crc: [u8; 2],
}

impl Header {
    fn from(
        length: usize,
        protocol_version: u8,
        profile_version: String,
        data_size: u32,
        data_type: String,
        crc: [u8; 2],
    ) -> Header {
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
        let length = *header_info.get(0).unwrap() as usize;
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
            crc,
        )
    }
}

pub struct Message {
    pub message_type: MessageType,
    pub data: Messages,
}

impl Message {
    pub fn from(message_type: MessageType, data: Messages) -> Message {
        Message { message_type, data }
    }

    pub fn is_unknown(&self) -> bool {
        self.message_type.name.eq("Unknown")
    }

    pub fn display_name(&self) -> String {
        if self.is_unknown() {
            let message_number = self.message_type.number;
            let mut field_value = "Unknown".to_string();
            field_value.push_str(message_number.to_string().as_str());
            field_value
        } else {
            self.message_type.name.to_string()
        }
    }
}

impl Clone for Message {
    fn clone(&self) -> Self {
        Message {
            message_type: self.message_type.clone(),
            data: self.data.clone(),
        }
    }
}

impl Serialize for Message {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut serialized = serializer
            .serialize_struct(self.message_type.name, 2)
            .unwrap();
        serialized
            .serialize_field("message_type", &self.display_name())
            .unwrap();
        serialized.serialize_field("message", &self.data).unwrap();
        serialized.end()
    }
}

#[derive(Clone)]
pub struct FieldValue {
    pub field: Field,
    pub value: Value,
}

pub struct Messages {
    pub data: Vec<FieldValue>,
}

impl Messages {
    pub fn value(&self, field_name: &str) -> &Value {
        &self
            .data
            .iter()
            .find(|&entry| match &entry.field {
                Field::Unknown(_inner_field) => false,
                Field::EnumField(inner_field) => inner_field.name.eq(field_name),
                Field::ValueField(inner_field) => inner_field.name.eq(field_name),
                Field::DeveloperField => false,
            })
            .unwrap()
            .value
    }
}

impl Clone for Messages {
    fn clone(&self) -> Self {
        Messages {
            data: self.data.clone(),
        }
    }
}

impl Serialize for Messages {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer
            .serialize_map(Option::from(self.data.len()))
            .unwrap();
        for entry in &self.data {
            map.serialize_key(&entry.field).unwrap();
            match &entry.value {
                // todo revisit this part
                Value::EnumValue(value) => {
                    if value.is_empty() {
                        map.serialize_value("").unwrap()
                    } else {
                        let enum_field_value = &u32::from(value[0]);
                        match &entry.field {
                            Field::Unknown(_) => map.serialize_value(value).unwrap(),
                            Field::EnumField(enum_field) => {
                                let string = (enum_field.translate_enum)(enum_field_value);
                                map.serialize_value(&string).unwrap()
                            }
                            _ => map.serialize_value(&entry.value).unwrap(),
                        }
                    }
                }
                _ => map.serialize_value(&entry.value).unwrap(),
            }
        }
        map.end()
    }
}
