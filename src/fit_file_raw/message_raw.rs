use serde::{Serialize, Serializer};
use serde::ser::{SerializeMap};
use crate::data_types::{BaseType, Value};
use crate::ParseConfig;

#[derive(Serialize)]
pub struct MessageRaw {
    pub message_number: u16,
    pub fields: Vec<FieldValueRaw>,
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

pub struct FieldValueRaw {
    pub field_number: u8,
    pub value: Value,
}

impl Serialize for FieldValueRaw {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        let mut serialized = serializer.serialize_map(Some(1)).unwrap();
        serialized.serialize_entry(self.field_number.to_string().as_str(), &self.value).unwrap();
        serialized.end()
    }
}

impl MessageDefinitionRaw {
    pub fn read_message(
        &self,
        current_position: &usize,
        buffer: &Vec<u8>,
        parse_config: &ParseConfig,
    ) -> (MessageRaw, usize) {
        let mut position = current_position.clone();
        let mut message_data = Vec::new();
        for field_definition in &self.fields {
            let read_size = field_definition.size;
            let base_type = BaseType::parse(&field_definition.base_type_value_or_dev_index);
            let end = position + (read_size as usize);
            let data = &buffer[position..end];
            let value = ((base_type).read)(&base_type, data, parse_config.endianness);
            position += read_size as usize;
            if !value.is_invalid() {
                message_data.push(FieldValueRaw { field_number: field_definition.number, value: value.clone() });
            }
        }
        (
            MessageRaw { message_number: self.message_type_number, fields: message_data },
            position,
        )
    }
}
