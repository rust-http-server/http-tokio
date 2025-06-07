use std::fmt::Display;
use mime_guess::{mime};

macro_rules! content_types {
    ($(($entry:ident, $mime:expr),)+) => {
        pub enum ContentType {
            $($entry,)+
        }

        impl AsRef<str> for ContentType {
            fn as_ref(&self) -> &str {
                match self {
                    $(ContentType::$entry => $mime.as_ref(),)+
                }
            }
        }

        impl Display for ContentType {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.as_ref())
            }
        }
    };
}



content_types! {
    (Html, mime::TEXT_HTML_UTF_8),
    (Json, mime::APPLICATION_JSON),
    (Xml, mime::TEXT_XML),
    (Plain, mime::TEXT_PLAIN_UTF_8),
    (OctetStream, mime::APPLICATION_OCTET_STREAM),
    (FormUrlEncoded, mime::APPLICATION_WWW_FORM_URLENCODED),
}