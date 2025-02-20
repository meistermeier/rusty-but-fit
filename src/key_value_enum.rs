#[macro_export]
macro_rules! key_value_enum {
    (pub enum $name:ident { $( $key:ident = $val:literal ),* $(,)? } ) => {
        #[derive(PartialEq, Debug)]
        #[repr(u32)]
        pub enum $name {
            $(
                $key = $val,
            )*
            Invalid =u32::MAX ,
        }

        impl $name {
            pub fn resolve(enum_value: &u32) -> Self {
                match enum_value {
                    $(
                        $val => Self::$key,
                    )*
                    _ => Self::Invalid
                }
            }
        }
        impl Display for $name {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                write!(f, "{:?}", self)
            }
        }
    }
}

#[macro_export]
macro_rules! expand_fields {
    ($($MESG_NUM:literal, $FIELD_NUM:literal, $field_name:literal, $enum_type:ident)+)  => {
        fn resolve_enum(message_type: &MessageType, field_number: u8) -> Field {
            let message_number = message_type.number;
            return match (message_type.number, field_number) {
                $(
                    ($MESG_NUM, $FIELD_NUM) => Field::EnumField(EnumField::from(field_number, $field_name.to_string(), |value| $enum_type::resolve(value).to_string())),
                )+
                _ => Field::Unknown(UnknownField { message_number, field_number }),
            };
        }
    };
    ($($MESG_NUM:literal, $FIELD_NUM:literal, $field_name:literal)+)  => {
        pub fn resolve_field(message_type: &MessageType, field_number: u8) -> Field {
            return match (message_type.number, field_number) {
                $(
                    ($MESG_NUM, $FIELD_NUM) => Field::ValueField(ValueField::from(field_number, $field_name.to_string())),
                )+
                _ => Field::resolve_enum(message_type, field_number),
            };
        }
    };
}

#[macro_export]
macro_rules! base_type {
    ($($NAME:ident, $READ_SIZE:literal, $TYPE_NUMBER:literal, $INVALID_VALUE:literal, $DATA_TYPE:ty, $VALUE_TYPE:ident, $VALUE_TYPE_VEC:ident)+) => {
        $(
    pub const $NAME: BaseType = BaseType {
        read_size: $READ_SIZE,
        type_number:$TYPE_NUMBER,
        invalid_value: $INVALID_VALUE,
        read: |me, data, endianness| {
            let size = data.len();
            // also create vec if type is enum
            if size > me.read_size || me.type_number == 0 {
                let mut value: Vec<$DATA_TYPE> = vec![];
                let mut invalid_count = 0;
                for i in (0..size).step_by(me.read_size) {
                    let bytes = data[i..i + me.read_size].try_into().unwrap();
                    let read_value = if endianness == 0 {<$DATA_TYPE>::from_le_bytes(bytes)} else {<$DATA_TYPE>::from_be_bytes(bytes)};
                    if read_value != me.invalid_value as $DATA_TYPE {
                        value.push(read_value);
                    } else {
                        invalid_count += 1;
                    }
                }
                if invalid_count == size {
                    Value::Invalid
                } else {
                    Value::$VALUE_TYPE_VEC(value)
                }
            } else {
                let value = if endianness == 0 {<$DATA_TYPE>::from_le_bytes(data.try_into().unwrap())} else {<$DATA_TYPE>::from_be_bytes(data.try_into().unwrap())};
                if value == me.invalid_value as $DATA_TYPE {
                    Value::Invalid
                } else {
                    Value::$VALUE_TYPE(value)
                }
            }
        },
    };
      )+
    }
}
