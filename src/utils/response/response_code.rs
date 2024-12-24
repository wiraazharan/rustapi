use serde::Serialize;

macro_rules! define_response_code {
    (
        $(
            $name:ident = $value:expr
        ),*
    ) => {
        #[derive(Debug, Serialize, Copy, Clone)]
        pub enum ResponseCode {
            $(
                $name = $value
            ),*
        }

        impl ResponseCode {
            pub fn to_code(&self) -> u16 {
                *self as u16
            }

            pub fn to_description(&self) -> &'static str {
                match self {
                    $(
                        ResponseCode::$name => stringify!($name),
                    )*
                }
            }
        }
    };
}

define_response_code! {
    ERROR = 2111,
    SUCCESS = 2100,
    GENERALERROR = 2222,
    DBQUERYERROR = 3333,
    NOTFOUND = 4444
}
