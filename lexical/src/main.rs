mod component;
mod location;
mod parser;
mod utils;

use starduck::component::Component;

fn main() {
    parser::parser::parse_yaml();
}
