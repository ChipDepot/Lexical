use super::{traits::{AsString, GetKeys}, ParseError};
use serde_yaml::{Mapping, Value};

impl AsString for Mapping {
    fn get_as_string(&self, key: &str) -> Result<String, ParseError> {
        match self.get(key) {
            Some(s) => Ok(s.as_str().unwrap().to_string()),
            None => Err(ParseError::MissingKey(key.to_string())),
        }
    }
}

impl GetKeys for Mapping {
    type T = String;

    fn as_vector(&self) -> Vec<Self::T> {
        fn process_key(key: &Value) -> String {
            match key.as_str() {
                Some(key) => key.to_string(),
                None => panic!("Invalid name used as keyname."),
            }
        }

        self.keys().map(process_key).collect()
    }
}