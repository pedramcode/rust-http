use std::collections::HashMap;

#[derive(Debug)]
pub enum Method {
    POST,
    GET,
    PUT,
    DELETE,
    HEAD,
    OPTIONS,
    CONNECT,
    TRACE,
    PATCH,
}

impl Method {
    fn from_string(method_str: &str) -> Option<Method> {
        match method_str {
            "POST" => Some(Method::POST),
            "GET" => Some(Method::GET),
            "PUT" => Some(Method::PUT),
            "DELETE" => Some(Method::DELETE),
            "HEAD" => Some(Method::HEAD),
            "OPTIONS" => Some(Method::OPTIONS),
            "CONNECT" => Some(Method::CONNECT),
            "TRACE" => Some(Method::TRACE),
            "PATCH" => Some(Method::PATCH),
            _ => None,
        }
    }

    fn to_string(&self) -> &str {
        match self {
            Method::POST => "POST",
            Method::GET => "GET",
            Method::PUT => "PUT",
            Method::DELETE => "DELETE",
            Method::HEAD => "HEAD",
            Method::OPTIONS => "OPTIONS",
            Method::CONNECT => "CONNECT",
            Method::TRACE => "TRACE",
            Method::PATCH => "PATCH",
        }
    }
}

#[derive(Debug)]
pub enum Kind {
    REQUEST,
    RESPONSE,
}

#[derive(Debug)]
pub struct HTTPMessage {
    pub kind: Kind,
    pub version: String,
    pub url: Option<String>,
    pub status: Option<u32>,
    pub reason: Option<String>,
    pub method: Option<Method>,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<String>,
}

impl HTTPMessage {
    pub fn from_string(data: &str) -> Result<Self, String> {
        let line = data.lines().next().unwrap();
        let head_tokens: Vec<&str> = line.split_whitespace().collect();
        if head_tokens.len() != 3 {
            return Err(String::from("HTTP message is invalid"));
        }

        let kind: Kind;
        let mut version: String = String::new();
        let mut method: Option<Method> = None;
        let mut url: Option<String> = None;
        let mut reason: Option<String> = None;
        let mut status: Option<u32> = None;

        if head_tokens[0].len() > 5 && head_tokens[0][0..5] == String::from("HTTP/") {
            kind = Kind::RESPONSE;
            version = head_tokens[0].to_string();
            reason = Some(head_tokens[2].to_string());
            status = Some(head_tokens[1].parse::<u32>().unwrap());
        } else {
            kind = Kind::REQUEST;
            version = head_tokens[2].to_string();
            method = Method::from_string(head_tokens[0]);
            url = Some(head_tokens[1].to_string());
        }

        let mut headers: HashMap<String, String> = HashMap::new();
        let mut body: Option<String> = None;

        let lines: Vec<&str> = data.lines().collect();
        let mut is_body = false;
        for line in lines.iter().skip(1) {
            if is_body {
                body = Some(line.to_string());
                break;
            }

            if line.trim().is_empty() {
                is_body = true;
                continue;
            }

            let header_tokens: Vec<&str> = line.splitn(2, ':').map(|s| s.trim()).collect();
            if header_tokens.len() == 2 {
                headers.insert(header_tokens[0].to_string(), header_tokens[1].to_string());
            }
        }

        Ok(HTTPMessage {
            kind,
            version,
            url,
            status,
            reason,
            method,
            headers: Some(headers),
            body,
        })
    }
}
