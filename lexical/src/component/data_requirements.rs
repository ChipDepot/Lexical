use std::str::FromStr;
use std::time::Duration;

use anyhow::{anyhow, Result};
use serde_yaml::Mapping;

use starduck::DataRequirement;
use starduck::IoTOutput;

use crate::parsing::{AsString, FromMapping};
use crate::parsing::{COUNT, OUTPUT, TIMEOUT};

impl FromMapping for DataRequirement {
    fn from_mapping(mapp: &Mapping) -> Result<Self> {
        let output = mapp
            .get_as_string(OUTPUT)
            .ok_or(anyhow!("Missing keyword output for data-requirement"))
            .map(|out_str| IoTOutput::from_str(&out_str))??;

        let timeout = mapp
            .get(TIMEOUT)
            .and_then(|val| val.as_u64())
            .map(Duration::from_secs);

        let required_count = mapp
            .get(COUNT)
            .map(|k| k.as_u64())
            .ok_or(anyhow!("Invalid value for `count`"))?
            .map(|count| count as usize)
            .ok_or(anyhow!("Invalid value for `count`"))?;

        Ok(Self::new(required_count, timeout, output))
    }
}
