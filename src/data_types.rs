use std::fmt::Debug;

use serde::{Serialize, Serializer};

#[derive(PartialEq, Clone)]
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
    NumberValueVecS8(Vec<i8>),
    NumberValueVecU8(Vec<u8>),
    NumberValueVecS16(Vec<i16>),
    NumberValueVecU16(Vec<u16>),
    NumberValueVecS32(Vec<i32>),
    NumberValueVecU32(Vec<u32>),
    NumberValueVecS64(Vec<i64>),
    NumberValueVecU64(Vec<u64>),
    Invalid,
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        match self {
            Value::EnumValue(value) => serializer.serialize_str("<becoming an enum>"),
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
            Value::Invalid => serializer.serialize_str("<invalid>"),
        }
    }
}

pub struct BaseType {
    pub read_size: usize,
    pub type_number: u8,
    pub name: &'static str,
    pub invalid_value: u64,
    pub read: fn(&BaseType, data: &[u8]) -> Value,
}

impl BaseType {
    pub const ENUM: BaseType = BaseType {
        read_size: 1,
        type_number: 0,
        name: "enum",
        invalid_value: 0xFF,
        read: |me, data| {
            let mut value = vec![];
            let size = data.len();
            let mut invalid_count = 0;
            for i in 0..size {
                if data[i] != me.invalid_value as u8 {
                    let bytes = data[i..i + 1].try_into().unwrap();
                    let read_value = u8::from_le_bytes(bytes);
                    value.push(read_value);
                } else {
                   invalid_count += 1;
                }
            }
            if invalid_count == size {
                Value::Invalid
            } else {
                Value::EnumValue(value)
            }
        },
    };
    // Array of bytes. Field is invalid if all bytes are invalid.
    pub const BYTE: BaseType = BaseType {
        read_size: 1,
        type_number: 13,
        name: "byte",
        invalid_value: 0xFF,
        read: |me, data| {
            let size = data.len();
            if size > me.read_size {
                let mut value: Vec<u8> = vec![];
                for i in 0..size {
                    let bytes = data[i..i + 1].try_into().unwrap();
                    let read_value = u8::from_le_bytes(bytes);
                    value.push(read_value);
                }
                Value::NumberValueVecU8(value)
            } else {
                let value = u8::from_le_bytes(data.try_into().unwrap());
                if value == me.invalid_value as u8 {
                    Value::Invalid
                } else {
                    Value::NumberValueU8(value)
                }
            }
        },
    };
    // Null terminated string encoded in UTF-8 format
    pub const STRING: BaseType = BaseType {
        read_size: 1,
        type_number: 7,
        name: "string",
        invalid_value: 0x00,
        read: |me, data| {
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
    // 2’s complement format
    pub const SINT8: BaseType = BaseType {
        read_size: 1,
        type_number: 1,
        name: "sint8",
        invalid_value: 0x7F,
        read: |me, data| {
            let size = data.len();
            if size > me.read_size {
                let mut value: Vec<i8> = vec![];
                for i in (0..size).step_by(me.read_size) {
                    let bytes = data[i..i + me.read_size].try_into().unwrap();
                    let read_value = i8::from_le_bytes(bytes);
                    value.push(read_value);
                }
                Value::NumberValueVecS8(value)
            } else {
                let value = i8::from_le_bytes(data.try_into().unwrap());
                if value == me.invalid_value as i8 {
                    Value::Invalid
                } else {
                    Value::NumberValueS8(value)
                }
            }
        },
    };
    // 2’s complement format
    pub const SINT16: BaseType = BaseType {
        read_size: 2,
        type_number: 131,
        name: "sint16",
        invalid_value: 0x7FFF,
        read: |me, data| {
            let size = data.len();
            if size > me.read_size {
                let mut value: Vec<i16> = vec![];
                for i in (0..size).step_by(me.read_size) {
                    let bytes = data[i..i + me.read_size].try_into().unwrap();
                    let read_value = i16::from_le_bytes(bytes);
                    value.push(read_value);
                }
                Value::NumberValueVecS16(value)
            } else {
                let value = i16::from_le_bytes(data.try_into().unwrap());
                if value == me.invalid_value as i16 {
                    Value::Invalid
                } else {
                    Value::NumberValueS16(value)
                }
            }
        },
    };

    // 2’s complement format
    pub const SINT32: BaseType = BaseType {
        read_size: 4,
        type_number: 133,
        name: "sint32",
        invalid_value: 0x7FFFFFFF,
        read: |me, data| {
            let size = data.len();
            if size > me.read_size {
                let mut value: Vec<i32> = vec![];
                for i in (0..size).step_by(me.read_size) {
                    let bytes = data[i..i + me.read_size].try_into().unwrap();
                    let read_value = i32::from_le_bytes(bytes);
                    value.push(read_value);
                }
                Value::NumberValueVecS32(value)
            } else {
                let value = i32::from_le_bytes(data.try_into().unwrap());
                if value == me.invalid_value as i32 {
                    Value::Invalid
                } else {
                    Value::NumberValueS32(value)
                }
            }
        },
    };
    // 2’s complement format
    pub const SINT64: BaseType = BaseType {
        read_size: 8,
        type_number: 142,
        name: "sint64",
        invalid_value: 0x7FFFFFFFFFFFFFFF,
        read: |me, data| {
            let size = data.len();
            if size > me.read_size {
                let mut value: Vec<i64> = vec![];
                for i in (0..size).step_by(me.read_size) {
                    let bytes = data[i..i + me.read_size].try_into().unwrap();
                    let read_value = i64::from_le_bytes(bytes);
                    value.push(read_value);
                }
                Value::NumberValueVecS64(value)
            } else {
                let value = i64::from_le_bytes(data.try_into().unwrap());
                if value == me.invalid_value as i64 {
                    Value::Invalid
                } else {
                    Value::NumberValueS64(value)
                }
            }
        },
    };
    pub const UINT8: BaseType = BaseType {
        read_size: 1,
        type_number: 2,
        name: "uint8",
        invalid_value: 0xFF,
        read: |me, data| {
            let size = data.len();
            if size > me.read_size {
                let mut value: Vec<u8> = vec![];
                for i in (0..size).step_by(me.read_size) {
                    let bytes = data[i..i + me.read_size].try_into().unwrap();
                    let read_value = u8::from_le_bytes(bytes);
                    value.push(read_value);
                }
                Value::NumberValueVecU8(value)
            } else {
                let value = u8::from_le_bytes(data.try_into().unwrap());
                if value == me.invalid_value as u8 {
                    Value::Invalid
                } else {
                    Value::NumberValueU8(value)
                }
            }
        },
    };
    pub const UINT16: BaseType = BaseType {
        read_size: 2,
        type_number: 132,
        name: "uint16",
        invalid_value: 0xFFFF,
        read: |me, data| {
            let size = data.len();
            if size > me.read_size {
                let mut value: Vec<u16> = vec![];
                for i in (0..size).step_by(me.read_size) {
                    let bytes = data[i..i + me.read_size].try_into().unwrap();
                    let read_value = u16::from_le_bytes(bytes);
                    value.push(read_value);
                }
                Value::NumberValueVecU16(value)
            } else {
                let value = u16::from_le_bytes(data.try_into().unwrap());
                if value == me.invalid_value as u16 {
                    Value::Invalid
                } else {
                    Value::NumberValueU16(value)
                }
            }
        },
    };
    pub const UINT32: BaseType = BaseType {
        read_size: 4,
        type_number: 134,
        name: "uint32",
        invalid_value: 0xFFFFFFFF,
        read: |me, data| {
            let size = data.len();
            if size > me.read_size as usize {
                let mut value: Vec<u32> = vec![];
                for i in (0..size).step_by(me.read_size as usize) {
                    let bytes = data[i..i + me.read_size as usize].try_into().unwrap();
                    let read_value = u32::from_le_bytes(bytes);
                    value.push(read_value);
                }
                Value::NumberValueVecU32(value)
            } else {
                let value = u32::from_le_bytes(data.try_into().unwrap());
                if value == me.invalid_value as u32 {
                    Value::Invalid
                } else {
                    Value::NumberValueU32(value)
                }
            }
        },
    };
    pub const UINT64: BaseType = BaseType {
        read_size: 8,
        type_number: 143,
        name: "uint64",
        invalid_value: 0xFFFFFFFFFFFFFFFF,
        read: |me, data| {
            let size = data.len();
            if size > me.read_size {
                let mut value: Vec<u64> = vec![];
                for i in (0..size).step_by(me.read_size) {
                    let bytes = data[i..i + me.read_size].try_into().unwrap();
                    let read_value = u64::from_le_bytes(bytes);
                    value.push(read_value);
                }
                Value::NumberValueVecU64(value)
            } else {
                let value = u64::from_le_bytes(data.try_into().unwrap());
                if value == me.invalid_value as u64 {
                    Value::Invalid
                } else {
                    Value::NumberValueU64(value)
                }
            }
        },
    };
    pub const UINT8Z: BaseType = BaseType {
        read_size: 1,
        type_number: 10,
        name: "uint8z",
        invalid_value: 0x00,
        read: |me, data| {
            let size = data.len();
            if size > me.read_size {
                let mut value: Vec<u8> = vec![];
                for i in (0..size).step_by(me.read_size) {
                    let bytes = data[i..i + me.read_size].try_into().unwrap();
                    let read_value = u8::from_le_bytes(bytes);
                    value.push(read_value);
                }
                Value::NumberValueVecU8(value)
            } else {
                let value = u8::from_le_bytes(data.try_into().unwrap());
                if value == me.invalid_value as u8 {
                    Value::Invalid
                } else {
                    Value::NumberValueU8(value)
                }
            }
        },
    };
    pub const UINT16Z: BaseType = BaseType {
        read_size: 2,
        type_number: 139,
        name: "uint16z",
        invalid_value: 0x0000,
        read: |me, data| {
            let size = data.len();
            if size > me.read_size {
                let mut value: Vec<u16> = vec![];
                for i in (0..size).step_by(me.read_size) {
                    let bytes = data[i..i + me.read_size].try_into().unwrap();
                    let read_value = u16::from_le_bytes(bytes);
                    value.push(read_value);
                }
                Value::NumberValueVecU16(value)
            } else {
                let value = u16::from_le_bytes(data.try_into().unwrap());
                if value == me.invalid_value as u16 {
                    Value::Invalid
                } else {
                    Value::NumberValueU16(value)
                }
            }
        },
    };
    pub const UINT32Z: BaseType = BaseType {
        read_size: 4,
        type_number: 140,
        name: "uint32z",
        invalid_value: 0x00000000,
        read: |me, data| {
            let size = data.len();
            if size > me.read_size {
                let mut value: Vec<u32> = vec![];
                for i in (0..size).step_by(me.read_size) {
                    let bytes = data[i..i + me.read_size].try_into().unwrap();
                    let read_value = u32::from_le_bytes(bytes);
                    value.push(read_value);
                }
                Value::NumberValueVecU32(value)
            } else {
                let value = u32::from_le_bytes(data.try_into().unwrap());
                if value == me.invalid_value as u32 {
                    Value::Invalid
                } else {
                    Value::NumberValueU32(value)
                }
            }
        },
    };
    pub const UINT64Z: BaseType = BaseType {
        read_size: 8,
        type_number: 144,
        name: "uint64z",
        invalid_value: 0x0000000000000000,
        read: |me, data| {
            let size = data.len();
            if size > me.read_size {
                let mut value: Vec<u64> = vec![];
                for i in (0..size).step_by(me.read_size) {
                    let bytes = data[i..i + me.read_size].try_into().unwrap();
                    let read_value = u64::from_le_bytes(bytes);
                    value.push(read_value);
                }
                Value::NumberValueVecU64(value)
            } else {
                let value = u64::from_le_bytes(data.try_into().unwrap());
                if value == me.invalid_value as u64 {
                    Value::Invalid
                } else {
                    Value::NumberValueU64(value)
                }
            }
        },
    };
    pub const FLOAT32: BaseType = BaseType {
        read_size: 4,
        type_number: 136,
        name: "float32",
        invalid_value: 0xFFFFFFFF,
        read: |me, data| {
            let size = data.len();
            if size > me.read_size {
                let mut value: Vec<u32> = vec![];
                for i in (0..size).step_by(me.read_size) {
                    let bytes = data[i..i + me.read_size].try_into().unwrap();
                    let read_value = u32::from_le_bytes(bytes);
                    value.push(read_value);
                }
                Value::NumberValueVecU32(value)
            } else {
                let value = u32::from_le_bytes(data.try_into().unwrap());
                if value == me.invalid_value as u32 {
                    Value::Invalid
                } else {
                    Value::NumberValueU32(value)
                }
            }
        },
    };
    pub const FLOAT64: BaseType = BaseType {
        read_size: 8,
        type_number: 137,
        name: "float64",
        invalid_value: 0xFFFFFFFFFFFFFFFF,
        read: |me, data| {
            let size = data.len();
            if size > me.read_size {
                let mut value: Vec<u64> = vec![];
                for i in (0..size).step_by(me.read_size) {
                    let bytes = data[i..i + me.read_size].try_into().unwrap();
                    let read_value = u64::from_le_bytes(bytes);
                    value.push(read_value);
                }
                Value::NumberValueVecU64(value)
            } else {
                let value = u64::from_le_bytes(data.try_into().unwrap());
                if value == me.invalid_value as u64 {
                    Value::Invalid
                } else {
                    Value::NumberValueU64(value)
                }
            }
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

    pub fn parse(value: u8) -> BaseType {
        for base_type in BaseType::ALL_TYPES {
            if base_type.type_number == value {
                return base_type;
            }
        }
        panic!("Unknown type {}", value)
    }

}
