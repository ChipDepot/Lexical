use starduck::application::Application;
use crate::parser::{traits::{FromMapping, AsString}, ParseError};


impl FromMapping for Application {
    type T = Application;

    fn from_mapping(mapp: &serde_yaml::Mapping) -> Result<Self::T, crate::parser::ParseError> {

        let app_name = mapp.get_as_string(Application::NAME)?;
        
        todo!()
    }
}