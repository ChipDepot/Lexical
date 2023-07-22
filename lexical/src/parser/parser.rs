use serde_yaml::{Value, Mapping};
use std::net::IpAddr;

use crate::utils::file_handler as file;
use crate::locations::{location::{self, Location}, error_handler::LocationError};
use crate::parser::{keywords::Keyword, error_handler::ParseError};


fn parse_locations(locations_value: &Value) -> Result<location::Location, ParseError> {

    let mapping = match locations_value.as_mapping() {
        Some(mapping) => mapping,
        None => panic!("Could not turn "),
    };

    // This should give us the parent, or root, location
    let mut location: Location = Location::from_mapping(mapping)?;
    let location_name: String = location.name.clone();
    // println!("{:?}", locations_value);
    
    // Now we get the parent location child location keys
    let keys: Vec<String> = match mapping.get(Keyword::LOCATIONS) {
        Some(val) => match val.as_mapping() {
            Some(mapp) => extract_keys(mapp),
            None => Vec::new(), // Return empty vector 
        },
        None => Vec::new(), 
    };
    
    // Validate that there is at least a child location 
    if keys.is_empty() && location.ip.is_none() {
        panic!("{}", LocationError::NoLocationIp(location_name));
    }
    
    for key in keys.into_iter() {
        let child_location = match locations_value.get(Keyword::LOCATIONS) {
            Some(child_locations) => match child_locations.get(&key) {
                Some(v) => parse_locations(v),
                None => Err(ParseError::MissingKey(key.clone())), // This shouldn't happen because of the way we get the keys
            },
            None => break, // There aren't any child locations
        };

        location.locations.insert(key, Box::new(child_location?));
    }

    // And now we should be able to return this
    Ok(location)
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

pub fn parse_ip(ip_string: String) -> Result<Option<IpAddr>, ParseError> {

    match ip_string.parse::<IpAddr>() {
        Ok(ip) => Ok(Some(ip)),
        Err(_) => Err(ParseError::NotIpAddr(ip_string)),
    }

}


pub fn extract_value_as_string(mapping: &Mapping, key: &str) -> Result<String, ParseError> {
    match mapping.get(key) {
        Some(val) => match val.as_str() {
            Some(s) => Ok(s.to_string()), 
            None => Err(ParseError::NotString(key.to_string())),
        },
        None => Err(ParseError::MissingKey(key.to_string())),
    }
}


pub fn parse_yaml() -> Result<Value, ParseError>{
    // First, we need to get the file. We can 
    let file = file::get_file().expect("Could not open file");
    let data: Value = serde_yaml::from_reader(file).expect("Could not read file as YAML");

    // Second, we give the locations section of the file to our location parser.
    let location = parse_locations(
        &data.get(Keyword::LOCATIONS)
        .expect(format!("Missing keyword '{}' on YAML file", Keyword::LOCATIONS).as_str()))?;

    println!("{}", location.to_string());

    todo!();
}

