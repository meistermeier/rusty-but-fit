use serde::{Serialize, Serializer};
use serde::ser::{SerializeMap, SerializeStruct};
use crate::data_types::{BaseType, Value};

#[derive(Serialize)]
pub struct MessageRaw {
    pub message_number: u16,
    pub fields: Fields,
}

#[derive(Serialize)]
pub struct FieldDefinitionRaw {
    pub(crate) number: u8,
    pub(crate) size: u8,
    pub(crate) base_type_value_or_dev_index: u8,
    pub(crate) is_dev_field: bool,
}

#[derive(Serialize)]
pub struct MessageDefinitionRaw {
    pub message_type_number: u16,
    pub fields: Vec<FieldDefinitionRaw>,
}

pub struct Fields {
    pub data: Vec<FieldValueRaw>,
}

impl Serialize for Fields {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        let mut serialized = serializer.serialize_map(Some(self.data.len())).unwrap();
        for entry in &self.data {
            serialized.serialize_key(entry.field_number.to_string().as_str()).unwrap();
            serialized.serialize_value(&entry.value).unwrap();
        }

        serialized.end()
    }
}

pub struct FieldValueRaw {
    pub field_number: u8,
    pub value: Value,
}

impl MessageDefinitionRaw {
    pub fn read_message(
        &self,
        current_position: &usize,
        buffer: &Vec<u8>,
        endianness: &u8,
    ) -> (MessageRaw, usize) {
        let mut position = current_position.clone();
        let mut message_data = Vec::new();
        for field_definition in &self.fields {
            let read_size = field_definition.size;
            let base_type = if field_definition.is_dev_field {
                BaseType::parse(&0)
            } else {
                BaseType::parse(&field_definition.base_type_value_or_dev_index)
            };
            let end = position + (read_size as usize);
            let data = &buffer[position..end];
            let value = ((base_type).read)(&base_type, data, endianness);
            position += read_size as usize;
            if !value.is_invalid() {
                message_data.push(FieldValueRaw { field_number: field_definition.number, value: value.clone() });
            }
        }
        (
            MessageRaw { message_number: self.message_type_number, fields: Fields{data: message_data }},
            position,
        )
    }
}
