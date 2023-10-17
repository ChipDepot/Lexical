use serde_yaml::{Mapping, Value};
use starduck::properties::Property;
use std::{collections::HashMap, net::IpAddr};

use crate::parser::ParseError;
use crate::utils::file_handler as file;

use starduck::{application::Application, component::Component, location::Location};

use super::{
    traits::{AsMapping, GetKeys},
    FromMapping,
};

#[non_exhaustive]
pub struct Parser {}

impl Parser {
    pub fn parse_yaml() -> Result<Application, ParseError> {
        info!("Loading file");
        let file = file::get_file(None).expect("Could not open file");
        let data: Value = serde_yaml::from_reader(file).expect("Could not read file as YAML");
        let mapping: &Mapping = data.as_mapping().expect("Could not map file as YAML");
        info!("YAML File loaded");

        let components = Self::parse_components(mapping)?;
        let mut locations = Self::parse_locations(mapping)?;

        Self::locate_components(mapping, &mut locations, components.clone());

        Ok(Application::new(locations, components.clone()))
    }

    fn parse_components(mapping: &Mapping) -> Result<HashMap<String, Component>, ParseError> {
        let mapping: &Mapping = mapping.get_as_mapping(Component::COMPONENTS).unwrap();

        let mut components: HashMap<String, Component> = HashMap::new();
        let component_keys = mapping.as_vector();

        for key in component_keys {
            let child_map = mapping.get_as_mapping(&key).unwrap();
            let component = Component::from_mapping(child_map)?;

            components.insert(key, component);
        }

        info!("Components validated");
        return Ok(components);
    }

    fn parse_locations(mapping: &Mapping) -> Result<HashMap<String, Box<Location>>, ParseError> {
        let mapping: &Mapping = mapping.get_as_mapping(Location::LOCATIONS).unwrap();

        let mut locations: HashMap<String, Box<Location>> = HashMap::new();
        let location_keys = mapping.as_vector();

        for key in location_keys {
            let child_map = mapping.get_as_mapping(&key).unwrap();
            let location = Location::from_mapping(child_map)?;

            locations.insert(key, location);
        }

        info!("Locations validated");
        return Ok(locations);
    }

    fn locate_components(
        mapping: &Mapping,
        locations: &mut HashMap<String, Box<Location>>,
        components: HashMap<String, Component>,
    ) {
        let component_mapping = mapping.get_as_mapping(Component::COMPONENTS).unwrap();
        let component_keys = mapping
            .get_as_mapping(Component::COMPONENTS)
            .unwrap()
            .as_vector();

        for key in component_keys {
            let component_locations = component_mapping
                .get_as_mapping(&key)
                .unwrap()
                .get_as_mapping(Component::LOCATIONS)
                .unwrap();

            let component_location_keys = component_locations.as_vector();

            for loc_key in component_location_keys {
                let loc_mapp = component_locations.get_as_mapping(&loc_key).unwrap();
                let loc_properties = Property::from_mapping(loc_mapp).unwrap();

                let mut component = components.get(&key).unwrap().clone();
                component.properties.extend(loc_properties);

                if locations.contains_key(&loc_key) {
                    locations
                        .get_mut(&loc_key)
                        .unwrap()
                        .components
                        .insert(key.to_owned(), component);
                    continue;
                }

                for (_, location) in &mut *locations {
                    if let Some(location) = location.get_mut(&loc_key) {
                        location
                            .components
                            .insert(key.to_owned(), component.clone());
                        continue;
                    }
                }
            }
        }

        info!("Objective Architecture validated");
    }

    pub fn parse_ip(ip_string: &str) -> Result<Option<IpAddr>, ParseError> {
        match ip_string.parse::<IpAddr>() {
            Ok(ip) => Ok(Some(ip)),
            Err(_) => Err(ParseError::NotIpAddr(String::from(ip_string))),
        }
    }
}
