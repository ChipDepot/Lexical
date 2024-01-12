use std::str::FromStr;

use anyhow::{anyhow, Result};
use chrono::Duration;
use serde_yaml::Mapping;

use starduck::DataRequirement;
use starduck::IoTOutput;

use crate::parsing::{AsString, FromMapping};
use crate::parsing::{COUNT, OUTPUT, REQUIRED, TIMEOUT};

impl FromMapping for DataRequirement {
    fn from_mapping(mapp: &Mapping) -> Result<Self> {
        let output = mapp
            .get_as_string(OUTPUT)
            .ok_or(anyhow!("Missing keyword '{OUTPUT}' for data-requirement"))
            .map(|out_str| IoTOutput::from_str(&out_str))??;

        let required = mapp
            .get_as_string(REQUIRED)
            .ok_or(anyhow!("Missing keyword '{REQUIRED}' for data-requirement"))
            .map(|out_str| out_str.parse())??;

        let timeout = mapp
            .get(TIMEOUT)
            .and_then(|val| val.as_i64())
            .map(Duration::seconds);

        let required_count = mapp
            .get(COUNT)
            .map(|k| k.as_u64())
            .ok_or(anyhow!("Invalid value for '{COUNT}'"))?
            .map(|count| count as usize)
            .ok_or(anyhow!("Invalid value for '{COUNT}'"))?;

        Ok(Self::new(required_count, required, timeout, output))
    }
}
