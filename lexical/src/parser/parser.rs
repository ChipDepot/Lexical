use serde_yaml::{Mapping, Value};
use std::{collections::HashMap, net::IpAddr};

use crate::parser::ParseError;
use crate::utils::file_handler as file;

use starduck::{
    component::Component,
    location::{Location, LocationError},
};

use super::{traits::GetKeys, FromMapping};

fn parse_locations(mapp: &Mapping) -> Result<Location, ParseError> {
    // Get location specific mapping
    let mapp = mapp
        .get(Location::LOCATIONS)
        .expect(&ParseError::MissingKey(String::from(Location::LOCATIONS)).to_string())
        .as_mapping()
        .unwrap();

    // This should give us the parent, or root, location
    let mut location: Location = Location::from_mapping(mapp)?;
    let location_name: String = location.name.clone();

    // Now we get the parent location child location keys
    let keys: Vec<String> = mapp
        .get(Location::LOCATIONS)
        .unwrap_or_default(Mapping::new)
        .unwrap_or_default()
        .as_vector();

    // Validate that there is at least a child location
    if keys.is_empty() && location.ip.is_none() {
        panic!("{}", LocationError::NoLocationIp(location_name));
    }

    for key in keys.into_iter() {
        let child_location = match mapp.get(Location::LOCATIONS) {
            Some(child_locations) => match child_locations.get(&key) {
                Some(v) => parse_locations(v.as_mapping().unwrap()),
                None => Err(ParseError::MissingKey(key.clone())), // This shouldn't happen because of the way we get the keys
            },
            None => break, // There aren't any child locations
        };

        location.locations.insert(key, Box::new(child_location?));
    }

    // And now we should be able to return this
    Ok(location)
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
    let mapp: &Mapping = data.as_mapping().expect("Could not map file as YAML");

    // Second, we build the base components
    let components: HashMap<String, Component> = parse_components(mapp);

    // Third, we give the locations section of the file to our location parser.
    let location = parse_locations(mapp)?;

    // let location = HashMap::new();

    // Now, that we got them componentes, we have to add them to the locations

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

// ---------------------------------------------------------------------------------------------------- //

fn parse_components(mapping: &Mapping) -> HashMap<String, Component> {
    let mapping: &Mapping = mapping
        .get(Component::COMPONENTS.to_string())
        .expect(
            ParseError::MissingKey(Component::COMPONENTS.to_string())
                .to_string()
                .as_str(),
        )
        .as_mapping()
        .expect("Unable to parse Component Mapping");

    let mut components: HashMap<String, Component> = HashMap::new();

    let component_keys = mapping.as_vector();

    for key in component_keys {
        let component = match mapping.get(&key).unwrap().as_mapping() {
            Some(m) => match Component::from_mapping(m) {
                Ok(comp) => comp,
                Err(e) => panic!("{}", e),
            },
            None => panic!("Invalid mapping for Component"),
        };

        components.insert(key, component);
    }

    return components;
}

// ---------------------------------------------------------------------------------------------------- //
