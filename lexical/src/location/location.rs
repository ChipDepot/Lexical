use std::collections::HashMap;
use std::net::IpAddr;

use serde_yaml::mapping::Mapping;

use crate::parser::error_handler::ParseError;
use crate::parser::Parser;
use crate::parser::traits::{AsString, GetKeys, AsMapping};
use crate::parser::FromMapping;

use starduck::location::Location;

impl FromMapping for Location {
    type T = Box<Location>;

    fn from_mapping(mapping: &Mapping) -> Result<Box<Location>, ParseError> {
        let name = mapping.get_as_string(Location::NAME)?;
        let ip: Option<IpAddr> = mapping
            .get_as_string(Location::IP)
            .and_then(|ip| Parser::parse_ip(&ip))
            .unwrap_or(None);

        let empty_map = Mapping::new();

        let child_mapping = mapping
            .get_as_mapping(Location::LOCATIONS)
            .unwrap_or(&empty_map);
    
        let mut locations: HashMap<String, Box<Location>> = HashMap::new();
        let location_keys = child_mapping.as_vector();
        
        if ip.is_none() && location_keys.is_empty() {
            return Err(ParseError::NoLocationIp(String::from(&name)));
        }
        
        for key in location_keys {
            let child_map = child_mapping.get_as_mapping(&key).unwrap();
            let location = Location::from_mapping(child_map).unwrap();
            
            locations.insert(key, location);
        }
    
        let mut location = Location::new(name, ip);
        location.locations = locations;

        Ok(Box::new(location))
    }
}
