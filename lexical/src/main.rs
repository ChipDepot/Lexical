mod component;
mod location;
mod parser;
mod utils;
mod application;

fn main() {
    parser::parser::parse_yaml();
}
