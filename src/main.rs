pub mod token;

use clap::{Parser, ValueEnum};
use token::decode::{decode_token, Token};

#[derive(ValueEnum, Clone, Debug)]
pub enum TokenType {
    Jwt,
}

#[derive(Parser)]
struct Args {
    /// Token to use
    #[arg(short, long)]
    token: String,

    /// Token type (format)
    #[arg(long, value_enum, default_value_t = TokenType::Jwt)]
    token_type: TokenType,
}

fn get_token_parts(args: &Args) -> Vec<String> {
    let token_parts: Vec<String> = match args.token_type {
        TokenType::Jwt => args
            .token
            .split('.')
            .into_iter()
            .map(String::from)
            .collect(),
    };
    token_parts
}

fn main() {
    let args = Args::parse();

    let token_parts = get_token_parts(&args);

    let token = decode_token(&token_parts, &args.token_type);

    match token {
        Token::JwtToken(jwt_token) => {
            println!("{}", jwt_token)
        }
    }
}
