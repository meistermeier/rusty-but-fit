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
macro_rules! ding {
    ($name:ident: $($field_name:ident, $type:ident)+ ) => {
        pub struct $name {
            $(
               $field_name: $type,
            )+
        }
    };
}
#[macro_export]
macro_rules! blubb {
    ($NAME:ident, $TYPE:ty, $READ_SIZE:literal, $TYPE_NUMBER:literal, $INVALID_VALUE:literal) => {
        pub const $NAME: BaseType = BaseType {
            read_size: $READ_SIZE,
            type_number: $TYPE_NUMBER,
            name: "asdf",
            invalid_value: $INVALID_VALUE,
            read: |me, data| {
                let size = data.len();
                if size > me.read_size as usize {
                    let mut value: Vec<$TYPE> = vec![];
                    for i in 0..size {
                        let position = (i * me.read_size);
                        let bytes = data[position..position + me.read_size].try_into().unwrap();
                        let read_value = <$TYPE>::from_le_bytes(bytes);
                        value.push(read_value);
                    }
                    Value::NumberValueVecS32(value)
                } else {
                    let value = <$TYPE>::from_le_bytes(data.try_into().unwrap());
                    if value == me.invalid_value as $TYPE {
                        Value::Invalid
                    } else {
                        Value::NumberValueS32(value)
                    }
                }
            }
        };
    };
    }