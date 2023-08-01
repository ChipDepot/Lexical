use serde_yaml::Mapping;

use crate::parser::parser;
use crate::parser::traits::{AsString, GetKeys};
use crate::parser::{FromMapping, ParseError};

use starduck::component::{Component, ComponentError, ComponentType, IoTOutput};

impl FromMapping for Component {
    type T = Component;

    fn from_mapping(mapp: &Mapping) -> Result<Self::T, ParseError> {
        // Get name and component-type from Mapping
        let name = mapp.get_as_string(Component::NAME)?;
        let component_type = ComponentType::from_string(
            mapp.get_as_string(Component::COMPONENT_TYPE)
                .unwrap()
                .as_str(),
        )
        .unwrap();

        // Create component
        let mut component = Component::new(name, component_type);

        // Create empty map for None cases
        let empty_map = Mapping::new();

        // Get outputs mapping
        let out_mapp = match mapp.get(Component::OUTPUTS) {
            Some(val) => val.as_mapping().unwrap(),
            None => &empty_map,
        };

        // Extract the keys
        let out_keys = out_mapp.as_vector();

        // Insert the outputs of the component
        for key in out_keys {
            let iot_out = match out_mapp.get_as_string(&key) {
                Ok(s) => match IoTOutput::from_string(s) {
                    Ok(i) => i,
                    Err(e) => panic!("{}", e),
                },
                Err(_) => panic!("{}", ParseError::NotString(key)),
            };

            component.outputs.insert(key, iot_out);
        }

        // Repeat the same thing for components
        let com_mapp = match mapp.get(Component::COMPONENTS) {
            Some(val) => val.as_mapping().unwrap(),
            None => &empty_map,
        };

        let com_keys = com_mapp.as_vector();

        for key in com_keys {
            let child_component = match com_mapp.get(&key) {
                Some(k) => match k.as_mapping() {
                    Some(mapp) => Component::from_mapping(mapp),
                    None => Err(ParseError::MissingKey(key.clone())),
                },
                None => break, // No child components
            };

            component.components.insert(key, Box::new(child_component?));
        }

        Ok(component)
    }
}
