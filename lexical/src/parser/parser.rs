use std::collections::HashMap;

use anyhow::{anyhow, bail, Result};
use serde_yaml::{Mapping, Value};

use crate::{
    parser::{APPLICATION, LOCATIONS},
    utils::file_handler as file,
};

use starduck::{Application, Location};

use super::{
    traits::{AsMapping, GetKeys},
    FromMapping,
};

pub(crate) fn load_mapping() -> Result<Mapping> {
    info!("Loading file");
    let file = file::get_file(None)?;
    let data: Value = serde_yaml::from_reader(file)?;

    info!("YAML File loaded");

    return data
        .as_mapping()
        .ok_or(anyhow!("Could not map file as YAML"))
        .cloned();
}

pub fn parse_yaml() -> Result<Application> {
    // Load mapping
    let mapping = load_mapping()?;

    info!("Creating Application instance");
    let mut app = match mapping.get_as_mapping(APPLICATION) {
        Some(app_info) => Application::from_mapping(app_info)?,
        None => bail!("Missing Application manifest in YAML file"),
    };
    info!("Application instance created");

    info!("Creating Locations");
    let locations = parse_locations(&mapping)?;
    info!("Locations created");

    app.locations.locations = locations;

    Ok(app)
}

fn parse_locations(mapping: &Mapping) -> Result<HashMap<String, Location>> {
    if let Some(mapping) = mapping.get_as_mapping(LOCATIONS) {
        let mut locations = HashMap::<String, Location>::new();

        for key in mapping.as_vector() {
            if let Some(mapp) = mapping.get_as_mapping(&key) {
                let location = Location::from_mapping(mapp)?;
                locations.insert(key, location);
            }
        }
        return Ok(locations);
    }

    bail!("`locations` keyword missing on YAML file");
}
