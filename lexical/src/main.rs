mod component;
mod location;
mod parser;
mod utils;
mod application;

fn main() {

    let app = parser::parser::Parser::parse_yaml().unwrap(); 

    println!("{}", serde_json::to_string_pretty(&app).unwrap());

}
