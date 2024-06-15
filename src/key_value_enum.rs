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
        pub fn resolve(message_type: &MessageType, i: u8) -> Field {
            return match (message_type.number, i) {
                $(
                    ($MESG_NUM, $FIELD_NUM) => Field::from($field_name, |value| $enum_type::resolve(value).to_string()),
                )+
                _ => Field::from("unknown", |_| "unknown".to_string()),
            };
        }
    };
}
