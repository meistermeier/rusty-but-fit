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
    ($($MESG_NUM:literal, $FIELD_NUM:literal, $field_name:ident, $enum_type:ident)+)  => {
            $(
                ($MESG_NUM, $FIELD_NUM) => panic!("alarm"),//$field_name,
            )+
    };
}
