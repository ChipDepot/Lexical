use std::net::IpAddr;

use anyhow::{anyhow, bail, Result};
use serde_yaml::mapping::Mapping;

use crate::parsing::{AsMapping, AsString, GetKeys};
use crate::parsing::{FromMapping, DATA_REQUIREMENTS, IP, LOCATIONS, NAME};

use starduck::{DataRequirement, Location};

impl FromMapping for Location {
    fn from_mapping(mapping: &Mapping) -> Result<Self> {
        let name = mapping
            .get_as_string(NAME)
            .ok_or(anyhow!("Missing `name` in location"))?;

        let ip = mapping
            .get_as_string(IP)
            .and_then(|ip| ip.parse::<IpAddr>().ok());

        let mut location = Location::new(&name, ip);

        if let (Some(_), Some(_)) = (mapping.get(LOCATIONS), mapping.get(DATA_REQUIREMENTS)) {
            bail!("Location {name} has both child locations and data-requirements.");
        }

        if let Some(child_mapping) = mapping.get_as_mapping(LOCATIONS) {
            let location_keys = child_mapping.as_vector();

            let locations = location_keys
                .iter()
                .map(|key| {
                    let child_map = child_mapping.get_as_mapping(key).unwrap();
                    let location = Location::from_mapping(child_map).unwrap();

                    (key.clone(), location)
                })
                .collect();

            location.locations = locations;

            return Ok(location);
        }

        if let Some(requirement_map) = mapping.get_as_mapping(DATA_REQUIREMENTS) {
            let data_req_keys = requirement_map.as_vector();
            let data_requirements = data_req_keys
                .iter()
                .map(|key| {
                    debug!("{key}");
                    let child_map = requirement_map.get_as_mapping(key).unwrap();
                    let data_req = DataRequirement::from_mapping(&child_map).unwrap();

                    (key.clone(), data_req)
                })
                .collect();

            location.data_requirements = data_requirements;

            return Ok(location);
        }

        bail!(
            "Missing information for {name} location. Missing child locations or data requirements"
        );
    }
}
