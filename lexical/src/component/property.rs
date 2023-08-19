use serde_yaml::Value;
use std::collections::HashMap;

use starduck::properties::Property;

use crate::parser::error_handler::ParseError;
use crate::parser::traits::{FromMapping, GetKeys};

impl FromMapping for Property {
    type T = HashMap<String, Property>;

    fn from_mapping(mapp: &serde_yaml::Mapping) -> Result<Self::T, ParseError> {
        let mut property_hashmap: HashMap<String, Property> = HashMap::new();
        let property_keys = mapp.as_vector();

        for key in property_keys {
            let property = match mapp.get(&key).unwrap() {
                Value::Bool(b) => Ok(Property::Bool(*b)),
                Value::Number(n) => {
                    if n.is_f64() {
                        Ok(Property::Float(n.as_f64().unwrap()))
                    } else {
                        Ok(Property::Integer(n.as_i64().unwrap()))
                    }
                }
                Value::String(s) => Ok(Property::String(s.to_owned())),
                _ => Err(ParseError::InvalidProperty(key.to_owned())),
            }?;

            property_hashmap.insert(key, property);
        }
        
        Ok(property_hashmap)
    }
}
