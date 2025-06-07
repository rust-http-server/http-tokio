use mime_guess::{mime};

macro_rules! content_types {
    ($(($fn:ident, $mime:expr),)+) => {
        $(
            pub fn $fn<'a>() -> (&'static str, String) {
                ("Content-Type", $mime.to_string())
            }
        )+
    };
}

content_types! {
    (html, mime::TEXT_HTML_UTF_8),
    (json, mime::APPLICATION_JSON),
    (xml, mime::TEXT_XML),
    (plain, mime::TEXT_PLAIN_UTF_8),
    (octet_stream, mime::APPLICATION_OCTET_STREAM),
    (form_url_encoded, mime::APPLICATION_WWW_FORM_URLENCODED),
}