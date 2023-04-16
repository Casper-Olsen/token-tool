use std::{collections::HashMap, fmt::Display};

use serde_json::Value;

use crate::TokenType;

pub struct JwtToken {
    pub header: String,
    pub payload: String,
    pub signature: Vec<u8>,
}

impl Display for JwtToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pretty_header = pretty_json(&self.header);
        let pretty_payload = pretty_json(&self.payload);

        writeln!(f, "Header:")?;
        writeln!(f, "{}", pretty_header)?;
        writeln!(f)?;
        writeln!(f, "Payload:")?;
        write!(f, "{}", pretty_payload)?;
        Ok(())
    }
}

fn pretty_json(json: &str) -> String {
    let hashmap: HashMap<String, Value> = serde_json::from_str(json).unwrap(); // TODO: Error handling
    let pretty_json = serde_json::to_string_pretty(&hashmap).unwrap(); // TODO: Error handling
    pretty_json
}

pub enum Token {
    JwtToken(JwtToken),
}

// Only works for JWT tokens for now
pub fn decode_token(token_parts: &Vec<String>, _token_type: &TokenType) -> Token {
    if token_parts.len() != 3 {
        panic!("Invalid format for JWT token")
    }

    let header = decode_token_part(&token_parts[0]);
    let payload = decode_token_part(&token_parts[1]);
    let signature = decode_signature_part(&token_parts[2]);

    let jwt_token = JwtToken {
        header: header,
        payload: payload,
        signature: signature,
    };

    Token::JwtToken(jwt_token)
}

fn decode_token_part(part: &str) -> String {
    let decoded = base64_url::decode(part).unwrap(); // TODO: Error handling
    let decoded_str = std::str::from_utf8(&decoded).unwrap(); // TODO: Error handling
    decoded_str.to_string()
}

fn decode_signature_part(part: &str) -> Vec<u8> {
    let decoded = base64_url::decode(part).unwrap(); // TODO: Error handling
    decoded
}
