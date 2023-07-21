use crate::utils::file_handler as file;
use crate::locations::location;
use crate::parser::keywords::Keyword;

use serde_yaml::{Value, Mapping};



fn parse_locations(locations: &Value) -> Result<location::Location, String> {

    let keys: Vec<String> = match locations.as_mapping() {
        Some(mapping) => extract_keys(mapping),
        None => todo!(),
    };

    // let location::Location::new(locations["name"].as_str().unwrap());

    println!("{:?}", locations);


    todo!();
}

fn extract_keys(mapping: &Mapping) -> Vec<String> {
    fn process_key(key: &Value) -> String {
        match key.as_str() {
            Some(key) => key.to_string(),
            None => panic!("Invalid name used as keyname."),
        }
    }

    mapping.keys().map(process_key).collect()
 }

pub fn parse_yaml() -> Result<Value, String>{
    // First, we need to get the file. We can 
    let file = file::get_file()?;
    let data: Value = serde_yaml::from_reader(file).expect("Could not read file as YAML");

    // Second, we give the locations section of the file to our location parser.
    parse_locations(&data[Keyword::Locations]);
    
    return Err("String".to_string())

}

