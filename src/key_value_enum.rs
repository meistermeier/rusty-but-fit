#[macro_export]
macro_rules! key_value_enum {
    (pub enum $name:ident { $( $key:ident = $val:literal ),+ $(,)? } ) => {
        #[derive(PartialEq, Debug)]
        #[repr(u8)]
        pub enum $name {
            $(
                $key = $val,
            )+
            Invalid = 255 ,
        }

        impl $name {
            pub fn resolve(enum_value: &u8) -> Self {
                match enum_value {
                    $(
                        $val => Self::$key,
                    )+
                    _ => Self::Invalid
                }
            }
        }
    }
}

#[macro_export]
macro_rules! something {
    ({$x:ident}*) => {
        $($x)*
    };
}
#[macro_export]
macro_rules! count_len {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + crate::count_len!($($xs)*));
}
#[macro_export]
macro_rules! arr_from_consts {
    ($nv:vis impl $ni:ident {$( $v:vis const $f:ident: $t:ident = $e:expr; )*}) => {
        $nv impl $ni {
            $($v const $f: $t = $e;)*
            $nv const ALL: [$ni; crate::count_len!($($f)*)] = [
                $($t :: $f),*
            ];
        }
    }
}