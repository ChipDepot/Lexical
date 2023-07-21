mod locations;
mod parser;
mod utils;

fn main() {
    println!("Hello, world!");

    parser::parser::parse_yaml();
}
