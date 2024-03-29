use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Value {
    NumberValueU64(u64),
    NumberValueU16(u16),
    NumberValueVecU16(Vec<u16>),
    NumberValueU32(u32),
    NumberValueVecU32(Vec<u32>),
    NumberValueU8(u8),
    NumberValueVecU8(Vec<u8>),
    StringValue(String),
    Undefined,
}

pub struct BaseType {
    pub read_size: u8,
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
            let mut value: Vec<u8> = vec![];
            let size = data.len();
            for i in 0..size {
                if data[i] != me.invalid_value as u8 {
                    let bytes = data[i..i + 1].try_into().unwrap();
                    let read_value = u8::from_le_bytes(bytes);
                    value.push(read_value);
                }
            }
            Value::NumberValueVecU8(value)
        },
    };
    // Array of bytes. Field is invalid if all bytes are invalid.
    pub const BYTE: BaseType = BaseType {
        read_size: 1,
        type_number: 13,
        name: "byte",
        invalid_value: 0xFF,
        read: |me, data| {
            let mut value = 0;
            let size = data.len();
            for i in 0..size {
                let shifty = 8 * i as i32;
                value += u64::from(data[i]) << shifty;
            }
            if value == me.invalid_value {
                Value::Undefined
            } else {
                Value::NumberValueU64(value)
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
            let mut value = 0;
            let size = data.len();
            for i in 0..size {
                let shifty = 8 * i as i32;
                value += u64::from(data[i]) << shifty;
            }
            if value == me.invalid_value {
                Value::Undefined
            } else {
                Value::NumberValueU64(value)
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
            let mut value = 0;
            let size = data.len();
            for i in 0..size {
                let shifty = 8 * i as i32;
                value += u64::from(data[i]) << shifty;
            }
            if value == me.invalid_value {
                Value::Undefined
            } else {
                Value::NumberValueU64(value)
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
            let mut value = 0;
            let size = data.len();
            for i in 0..size {
                let shifty = 8 * i as i32;
                value += u64::from(data[i]) << shifty;
            }
            if value == me.invalid_value {
                Value::Undefined
            } else {
                Value::NumberValueU64(value)
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
            let mut value = 0;
            let size = data.len();
            for i in 0..size {
                let shifty = 8 * i as i32;
                value += u64::from(data[i]) << shifty;
            }
            if value == me.invalid_value {
                Value::Undefined
            } else {
                Value::NumberValueU64(value)
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
            // vec or plain
            if size > me.read_size as usize {
                let mut value: Vec<u8> = vec![];
                for i in 0..size {
                    let bytes = data[i..i + 1].try_into().unwrap();
                    let read_value = u8::from_le_bytes(bytes);
                    value.push(read_value);
                }
                Value::NumberValueVecU8(value)
            } else {
                // for i in 0..size {
                //     let shifty = 8 * i as i32;
                //     value += u64::from(data[i]) << shifty;
                // }
                let value = u8::from_le_bytes(data.try_into().unwrap());
                if value == me.invalid_value as u8 {
                    Value::Undefined
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
            // vec or plain
            if size > me.read_size as usize {
                let mut value: Vec<u16> = vec![];
                for i in (0..size).step_by(me.read_size as usize) {
                    let bytes = data[i..i + 2].try_into().unwrap();
                    let read_value = u16::from_le_bytes(bytes);
                    value.push(read_value);
                }
                Value::NumberValueVecU16(value)
            } else {
                // for i in 0..size {
                //     let shifty = 8 * i as i32;
                //     value += u64::from(data[i]) << shifty;
                // }
                let value = u16::from_le_bytes(data.try_into().unwrap());
                if value == me.invalid_value as u16 {
                    Value::Undefined
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
            // vec or plain
            if size > me.read_size as usize {
                let mut value: Vec<u32> = vec![];
                for i in (0..size).step_by(me.read_size as usize) {
                    let bytes = data[i..i + me.read_size as usize].try_into().unwrap();
                    let read_value = u32::from_le_bytes(bytes);
                    value.push(read_value);
                }
                Value::NumberValueVecU32(value)
            } else {
                // for i in 0..size {
                //     let shifty = 8 * i as i32;
                //     value += u64::from(data[i]) << shifty;
                // }
                let value = u32::from_le_bytes(data.try_into().unwrap());
                if value == me.invalid_value as u32 {
                    Value::Undefined
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
            let mut value = 0;
            let size = data.len();
            for i in 0..size {
                let shifty = 8 * i as i32;
                value += u64::from(data[i]) << shifty;
            }
            if value == me.invalid_value {
                Value::Undefined
            } else {
                Value::NumberValueU64(value)
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
            // vec or plain
            if size > me.read_size as usize {
                let mut value: Vec<u8> = vec![];
                for i in 0..size {
                    let bytes = data[i..i + 1].try_into().unwrap();
                    let read_value = u8::from_le_bytes(bytes);
                    value.push(read_value);
                }
                Value::NumberValueVecU8(value)
            } else {
                // for i in 0..size {
                //     let shifty = 8 * i as i32;
                //     value += u64::from(data[i]) << shifty;
                // }
                let value = u8::from_le_bytes(data.try_into().unwrap());
                if value == me.invalid_value as u8 {
                    Value::Undefined
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
            let mut value = 0;
            let size = data.len();
            for i in 0..size {
                let shifty = 8 * i as i32;
                value += u64::from(data[i]) << shifty;
            }
            if value == me.invalid_value {
                Value::Undefined
            } else {
                Value::NumberValueU64(value)
            }
        },
    };
    pub const UINT32Z: BaseType = BaseType {
        read_size: 4,
        type_number: 140,
        name: "uint32z",
        invalid_value: 0x00000000,
        read: |me, data| {
            let mut value = 0;
            let size = data.len();
            for i in 0..size {
                let shifty = 8 * i as i32;
                value += u64::from(data[i]) << shifty;
            }
            if value == me.invalid_value {
                Value::Undefined
            } else {
                Value::NumberValueU64(value)
            }
        },
    };
    pub const UINT64Z: BaseType = BaseType {
        read_size: 8,
        type_number: 144,
        name: "uint64z",
        invalid_value: 0x0000000000000000,
        read: |me, data| {
            let mut value = 0;
            let size = data.len();
            for i in 0..size {
                let shifty = 8 * i as i32;
                value += u64::from(data[i]) << shifty;
            }
            if value == me.invalid_value {
                Value::Undefined
            } else {
                Value::NumberValueU64(value)
            }
        },
    };
    pub const FLOAT32: BaseType = BaseType {
        read_size: 4,
        type_number: 136,
        name: "float32",
        invalid_value: 0xFFFFFFFF,
        read: |me, data| {
            let mut value = 0;
            let size = data.len();
            for i in 0..size {
                let shifty = 8 * i as i32;
                value += u64::from(data[i]) << shifty;
            }
            if value == me.invalid_value {
                Value::Undefined
            } else {
                Value::NumberValueU64(value)
            }
        },
    };
    pub const FLOAT64: BaseType = BaseType {
        read_size: 8,
        type_number: 137,
        name: "float64",
        invalid_value: 0xFFFFFFFFFFFFFFFF,
        read: |me, data| {
            let mut value = 0;
            let size = data.len();
            for i in 0..size {
                let shifty = 8 * i as i32;
                value += u64::from(data[i]) << shifty;
            }
            if value == me.invalid_value {
                Value::Undefined
            } else {
                Value::NumberValueU64(value)
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
