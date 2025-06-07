use std::{collections::HashMap, ops::{Deref, DerefMut}};

#[derive(Debug, Default, Clone)]
pub struct Headers(HashMap<String, Vec<String>>);

impl Headers {
    pub fn new() -> Self {
        Headers(HashMap::new())
    }
    
    /// Custom insert: forces lowercase keys
    pub fn insert(&mut self, key_value: (impl AsRef<str>, impl AsRef<str>)) {
        self.0.insert(capitalize_key(key_value.0.as_ref()), vec![key_value.1.as_ref().to_string()]);
    }

    pub fn append(&mut self, key_value: (impl AsRef<str>, impl AsRef<str>)) {
        self.0
            .entry(capitalize_key(key_value.0.as_ref()))
            .or_insert_with(Vec::new)
            .push(key_value.1.as_ref().to_string());
    }

    pub fn insert_header_line(&mut self, line: &str) -> Result<(), ()> {
        if let Some(key_value) = line.split_once(": ") {
            self.insert(key_value);
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.0.get(&capitalize_key(key)).map(|v| v.first()).flatten()
    }

    pub fn add_set_cookie(&mut self, name: &str, value: &str) {
        let cookie_str = format!("{}={}", name, value);
        self.append(("Set-Cookie", cookie_str));
    }

    pub fn get_cookie(&self, name: &str) -> Option<String> {
        self.get("Cookie").and_then(|cookie_header| {
            cookie_header
                .split(';')
                .map(|pair| pair.trim())
                .find_map(|pair| {
                    let mut parts = pair.splitn(2, '=');
                    match (parts.next(), parts.next()) {
                        (Some(k), Some(v)) if k == name => Some(v.to_string()),
                        _ => None,
                    }
                })
        })
    }

    /// Converts headers into a formatted string with capitalized keys.
    pub fn to_string(&self) -> String {
        self.0
            .iter()
            .flat_map(|(key, values)| {
                values.iter().map(move |value| {
                    format!("{}: {}", key, value)
                })
            })
            .collect::<Vec<_>>()
            .join("\r\n")
    }

    pub fn is_chunked(&self) -> bool {
        self.get("Transfer-Encoding") == Some(&String::from("chunked"))
    }
}

/// Allow Headers to behave like HashMap<String, String>
impl Deref for Headers {
    type Target = HashMap<String, Vec<String>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Headers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0  
    }
}

/// Capitalizes HTTP header keys (e.g., "content-type" -> "Content-Type").
fn capitalize_key(key: &str) -> String {
    key.split('-')
        .map(|s| {
            let mut chars = s.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().chain(chars).collect(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join("-")
}
