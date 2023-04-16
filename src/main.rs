use clap::{Parser, ValueEnum};

#[derive(Parser)]
struct Args {
    /// Token to use
    #[arg(short, long)]
    token: String,

    ///
    #[arg(long, value_enum, default_value_t = TokenType::Jwt)]
    token_type: TokenType,
}

#[derive(ValueEnum, Clone, Debug)]
enum TokenType {
    Jwt,
}

fn main() {
    let args = Args::parse();

    println!("Hello {}!", args.token);
}
