use serde_yaml::{Mapping, Value};
use std::{collections::HashMap, net::IpAddr};

use crate::parser::ParseError;
use crate::utils::file_handler as file;

use starduck::{
    component::Component,
    location::Location,
    application::Application
};

use super::{traits::{GetKeys, AsMapping}, FromMapping};

#[non_exhaustive]
pub struct Parser {}

impl Parser {
    pub fn parse_yaml() -> Result<Application, ParseError> {
        // First, we need to get the file. We can
        let file = file::get_file().expect("Could not open file");
        let data: Value = serde_yaml::from_reader(file).expect("Could not read file as YAML");
        let mapping: &Mapping = data.as_mapping().expect("Could not map file as YAML");

        let components = Self::parse_components(mapping);
        let location = Self::parse_locations(mapping);

        todo!()
    }

    fn parse_components(mapping: &Mapping) -> HashMap<String, Component> {
        let mapping: &Mapping = mapping.get_as_mapping(Component::COMPONENTS).unwrap();

        let mut components: HashMap<String, Component> = HashMap::new();
        let component_keys = mapping.as_vector();
    
        for key in component_keys {
            let child_map = mapping.get_as_mapping(&key).unwrap();
            let component = Component::from_mapping(child_map).unwrap();
    
            components.insert(key, component);
        }
    
        return components;
    }

    fn parse_locations(mapping: &Mapping) -> HashMap<String, Location> {
        let mapping: &Mapping = mapping.get_as_mapping(Location::LOCATIONS).unwrap();

        let mut locations: HashMap<String, Location> = HashMap::new();
        let location_keys = mapping.as_vector();

        for key in location_keys {
            let child_map = mapping.get_as_mapping(&key).unwrap();
            let location = Location::from_mapping(child_map).unwrap();

            locations.insert(key, location);
        }

        return locations;
    }

    pub fn parse_ip(ip_string: &str) -> Result<Option<IpAddr>, ParseError> {
        match ip_string.parse::<IpAddr>() {
            Ok(ip) => Ok(Some(ip)),
            Err(_) => Err(ParseError::NotIpAddr(String::from(ip_string))),
        }
    }

}