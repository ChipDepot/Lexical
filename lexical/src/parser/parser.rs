use serde_yaml::{Mapping, Value};
use std::{collections::HashMap, net::IpAddr};

use crate::parser::ParseError;
use crate::utils::file_handler as file;

use starduck::{
    component::Component,
    location::{Location, LocationError},
};

use super::{traits::GetKeys, FromMapping};

fn parse_locations(locations_value: &Value) -> Result<Location, ParseError> {
    let mapping = match locations_value.as_mapping() {
        Some(mapping) => mapping,
        None => panic!("Could not turn "),
    };

    // This should give us the parent, or root, location
    let mut location: Location = Location::from_mapping(mapping)?;
    let location_name: String = location.name.clone();
    // println!("{:?}", locations_value);

    // Now we get the parent location child location keys
    let keys: Vec<String> = match mapping.get(Location::LOCATIONS) {
        Some(val) => match val.as_mapping() {
            Some(mapp) => mapp.as_vector(),
            None => Vec::new(), // Return empty vector
        },
        None => Vec::new(),
    };

    // Validate that there is at least a child location
    if keys.is_empty() && location.ip.is_none() {
        panic!("{}", LocationError::NoLocationIp(location_name));
    }

    for key in keys.into_iter() {
        let child_location = match locations_value.get(Location::LOCATIONS) {
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

fn parse_components(mapping: &Mapping) -> HashMap<String, Component> {
    let mut components: HashMap<String, Component> = HashMap::new();

    let component_keys = mapping.as_vector();

    for key in component_keys {
        let component = match mapping.get(&key) {
            Some(c) => match c.as_mapping() {
                Some(m) => match Component::from_mapping(m) {
                    Ok(comp) => comp,
                    Err(e) => panic!("{}", e),
                },
                None => panic!("Invalid mapping for Component"),
            },
            None => panic!("{}", ParseError::MissingKey(key.clone())), // Shouldn't happen
        };

        components.insert(key, component);
    }

    return components;
}

pub fn parse_ip(ip_string: String) -> Result<Option<IpAddr>, ParseError> {
    match ip_string.parse::<IpAddr>() {
        Ok(ip) => Ok(Some(ip)),
        Err(_) => Err(ParseError::NotIpAddr(ip_string)),
    }
}

pub fn get_as_string(mapping: &Mapping, key: &str) -> Result<String, ParseError> {
    match mapping.get(key) {
        Some(val) => match val.as_str() {
            Some(s) => Ok(s.to_string()),
            None => Err(ParseError::NotString(key.to_string())),
        },
        None => Err(ParseError::MissingKey(key.to_string())),
    }
}

pub fn parse_yaml() -> Result<Value, ParseError> {
    // First, we need to get the file. We can
    let file = file::get_file().expect("Could not open file");
    let data: Value = serde_yaml::from_reader(file).expect("Could not read file as YAML");

    // Second, we give the locations section of the file to our location parser.
    let location = parse_locations(
        &data
            .get(Location::LOCATIONS)
            .expect(format!("Missing keyword '{}' on YAML file", Location::LOCATIONS).as_str()),
    )?;

    let components = parse_components(
        data.get(Component::COMPONENTS.to_string())
            .expect(&format!(
                "{}",
                ParseError::MissingKey(Component::COMPONENTS.to_string())
            ))
            .as_mapping()
            .expect(&format!(
                "{}",
                ParseError::MissingKey(Component::COMPONENTS.to_string())
            )),
    );
    // let location = HashMap::new();

    println!(
        "{}",
        components
            .get("dol139")
            .unwrap()
            .components
            .get("co2-sensor")
            .unwrap()
            .to_string()
    );

    todo!();
}
