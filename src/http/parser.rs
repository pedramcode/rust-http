use std::collections::HashMap;

pub enum Method {
    POST,
    GET,
    PUT,
    DELETE,
    HEADER,
    OPTION,
}

pub enum Kind {
    REQUEST,
    RESPONSE,
}

pub struct HTTPMessage {
    pub kind: Kind,
    pub version: u32,
    pub url: String,
    pub status: u32,
    pub reason: String,
    pub method: Method,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl HTTPMessage {
    pub fn from_string(data: &str) -> Result<Self, String> {
        let mut message: HTTPMessage;
        let line = data.lines().next().unwrap();
        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.len() != 3 {
            return Err(String::from("HTTP message is invalid"));
        }

        let kind: Kind;

        if tokens.0 == "HTTP/1.1" {
            kind = Kind::RESPONSE;
        }else{
            kind = Kind::REQUEST;
        }

        Ok(HTTPMessage {
            kind,
            version: 0,
            url: "".to_string(),
            status: 0,
            reason: "".to_string(),
            method: Method::POST,
            headers: Default::default(),
            body: "".to_string(),
        })
    }
}
