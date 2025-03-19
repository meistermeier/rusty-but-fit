use std::fmt::Debug;

use serde::{Serialize, Serializer};

#[derive(PartialEq, Clone, Debug)]
pub enum Value {
    EnumValue(Vec<u8>),
    NumberValueS8(i8),
    NumberValueU8(u8),
    NumberValueS16(i16),
    NumberValueU16(u16),
    NumberValueS32(i32),
    NumberValueU32(u32),
    StringValue(String),
    NumberValueS64(i64),
    NumberValueU64(u64),
    NumberValueF32(f32),
    NumberValueF64(f64),
    NumberValueVecS8(Vec<i8>),
    NumberValueVecU8(Vec<u8>),
    NumberValueVecS16(Vec<i16>),
    NumberValueVecU16(Vec<u16>),
    NumberValueVecS32(Vec<i32>),
    NumberValueVecU32(Vec<u32>),
    NumberValueVecS64(Vec<i64>),
    NumberValueVecU64(Vec<u64>),
    NumberValueVecF32(Vec<f32>),
    NumberValueVecF64(Vec<f64>),
    Invalid,
}

impl Value {
    pub fn is_invalid(&self) -> bool {
        match *self {
            Value::Invalid => true,
            _ => false,
        }
    }

    fn serialize_intern<S>(value_type: &Value, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value_type {
            Value::EnumValue(value) => serializer.serialize_some(value),
            Value::NumberValueU64(value) => serializer.serialize_u64(value.clone()),
            Value::NumberValueU16(value) => serializer.serialize_u16(value.clone()),
            Value::NumberValueVecU16(value) => serializer.serialize_some(value),
            Value::NumberValueU32(value) => serializer.serialize_u32(value.clone()),
            Value::NumberValueVecU32(value) => serializer.serialize_some(value),
            Value::NumberValueVecU64(value) => serializer.serialize_some(value),
            Value::NumberValueVecS64(value) => serializer.serialize_some(value),
            Value::NumberValueU8(value) => serializer.serialize_u8(value.clone()),
            Value::NumberValueVecU8(value) => serializer.serialize_some(value),
            Value::NumberValueS64(value) => serializer.serialize_i64(value.clone()),
            Value::NumberValueS16(value) => serializer.serialize_i16(value.clone()),
            Value::NumberValueVecS16(value) => serializer.serialize_some(value),
            Value::NumberValueS32(value) => serializer.serialize_i32(value.clone()),
            Value::NumberValueVecS32(value) => serializer.serialize_some(value),
            Value::NumberValueS8(value) => serializer.serialize_i8(value.clone()),
            Value::NumberValueVecS8(value) => serializer.serialize_some(value),
            Value::StringValue(value) => serializer.serialize_str(value.as_str()),
            Value::NumberValueF32(value) => serializer.serialize_f32(value.clone()),
            Value::NumberValueF64(value) => serializer.serialize_f64(value.clone()),
            Value::NumberValueVecF32(value) => serializer.serialize_some(value),
            Value::NumberValueVecF64(value) => serializer.serialize_some(value),
            Value::Invalid => serializer.serialize_str("invalid value"),
        }
    }
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Value::serialize_intern(&self, serializer)
    }
}

#[derive(Debug)]
pub struct BaseType {
    pub read_size: usize,
    pub type_number: u8,
    pub invalid_value: u64,
    pub read: fn(&BaseType, data: &[u8], endianness: &u8) -> Value,
}

impl Clone for BaseType {
    fn clone(&self) -> Self {
        BaseType {
            read_size: self.read_size,
            type_number: self.type_number,
            invalid_value: self.invalid_value,
            read: self.read,
        }
    }
}

impl BaseType {
    crate::base_type! {
        ENUM, 1, 0, 0xFF, u8, NumberValueU8, EnumValue
        BYTE, 1, 13, 0xFF, u8, NumberValueU8, NumberValueVecU8
        SINT8, 1, 1, 0x7F, i8, NumberValueS8, NumberValueVecS8
        SINT16, 2, 131, 0x7FFF, i16, NumberValueS16, NumberValueVecS16
        SINT32, 4, 133, 0x7FFFFFFF, i32, NumberValueS32, NumberValueVecS32
        SINT64, 8, 142, 0x7FFFFFFFFFFFFFFF, i64, NumberValueS64, NumberValueVecS64
        UINT8, 1, 2, 0xFF, u8, NumberValueU8, NumberValueVecU8
        UINT16, 2, 132, 0xFFFF, u16, NumberValueU16, NumberValueVecU16
        UINT32, 4, 134, 0xFFFFFFF, u32, NumberValueU32, NumberValueVecU32
        UINT64, 8, 143, 0xFFFFFFFFFFFFFFFF, u64, NumberValueU64, NumberValueVecU64
        UINT8Z, 1, 10, 0x00, u8, NumberValueU8, NumberValueVecU8
        UINT16Z, 2, 139, 0x0000, u16, NumberValueU16, NumberValueVecU16
        UINT32Z, 4, 140, 0x00000000, u32, NumberValueU32, NumberValueVecU32
        UINT64Z, 8, 144, 0x0000000000000000, u64, NumberValueU64, NumberValueVecU64
        FLOAT32, 4, 136, 0xFFFFFFF, f32, NumberValueF32, NumberValueVecF32
        FLOAT64, 8, 137, 0xFFFFFFFFFFFFFFFF, f64, NumberValueF64, NumberValueVecF64
    }
    // Null terminated string encoded in UTF-8 format
    pub const STRING: BaseType = BaseType {
        read_size: 1,
        type_number: 7,
        invalid_value: 0x00,
        read: |me, data, _| {
            let size = data.len();
            let mut value = String::new();
            for i in 0..size {
                let raw_value = data[i];
                if raw_value != 0 && raw_value != me.invalid_value as u8 {
                    value.push(raw_value as char);
                }
            }
            Value::StringValue(value)
        },
    };

    const ALL_TYPES: [BaseType; 17] = [
        BaseType::ENUM,
        BaseType::BYTE,
        BaseType::STRING,
        BaseType::SINT8,
        BaseType::SINT16,
        BaseType::SINT32,
        BaseType::SINT64,
        BaseType::UINT8,
        BaseType::UINT16,
        BaseType::UINT32,
        BaseType::UINT64,
        BaseType::UINT8Z,
        BaseType::UINT16Z,
        BaseType::UINT32Z,
        BaseType::UINT64Z,
        BaseType::FLOAT32,
        BaseType::FLOAT64,
    ];

    pub fn parse(value: &u8) -> BaseType {
        for base_type in BaseType::ALL_TYPES {
            if base_type.type_number.eq(value) {
                return base_type;
            }
        }
        panic!("Unknown type {}", value)
    }
}
