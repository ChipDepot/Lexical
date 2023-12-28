use std::collections::HashMap;
use std::net::IpAddr;

use anyhow::{bail, Result};
use serde_yaml::mapping::Mapping;

use crate::parser::traits::{AsMapping, AsString, GetKeys};
use crate::parser::{FromMapping, IP, LOCATIONS, NAME};

use starduck::Location;

impl FromMapping for Location {
    fn from_mapping(mapping: &Mapping) -> Result<Location> {
        if let Some(name) = mapping.get_as_string(NAME) {
            // let ip: Option<IpAddr> = mapping.get_as_string(IP);

            let empty_map = Mapping::new();

            let child_mapping = mapping.get_as_mapping(LOCATIONS).unwrap_or(&empty_map);

            let mut locations: HashMap<String, Location> = HashMap::new();
            let location_keys = child_mapping.as_vector();

            // if ip.is_none() && location_keys.is_empty() {
            //     bail!("Child location `{name}` has no ip");
            // }

            for key in location_keys {
                let child_map = child_mapping.get_as_mapping(&key).unwrap();
                let location = Location::from_mapping(child_map).unwrap();

                locations.insert(key, location);
            }

            let mut location = Location::new(&name, None);
            location.locations = locations;

            return Ok(location);
        }

        bail!("Missing name in location");
    }
}
