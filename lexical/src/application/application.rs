use anyhow::{bail, Result};

use starduck::Application;

use crate::parsing::{AsString, FromMapping};

impl FromMapping for Application {
    fn from_mapping(mapp: &serde_yaml::Mapping) -> Result<Self> {
        const NAME: &str = "name";
        const DESCRIPTION: &str = "description";

        if let Some(app_name) = mapp.get_as_string(NAME) {
            let description = mapp.get_as_string(DESCRIPTION);

            return Ok(Application::new(&app_name, description.as_deref()));
        }

        bail!("Missing application.name on YAML file");
    }
}
