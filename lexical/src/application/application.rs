use starduck::application::Application;
use crate::parser::{traits::FromMapping, ParseError};


impl FromMapping for Application {
    type T = Application;

    fn from_mapping(_mapp: &serde_yaml::Mapping) -> Result<Self::T, ParseError> {

        // let app_name = mapp.get_as_string(Application::NAME)?;
        
        todo!()
    }
}