use super::traits::{AsMapping, AsString, GetKeys};
use serde_yaml::{Mapping, Value};

impl AsString for Mapping {
    fn get_as_string(&self, key: &str) -> Option<String> {
        self.get(key).and_then(|res| match res.as_str() {
            Some(s) => Some(s.to_string()),
            None => None,
        })
    }
}

impl GetKeys<String> for Mapping {
    fn as_vector(&self) -> Vec<String> {
        fn process_key(key: &Value) -> String {
            match key.as_str() {
                Some(key) => key.to_string(),
                None => panic!("Invalid name used as keyname."),
            }
        }

        self.keys().map(process_key).collect()
    }
}

impl AsMapping for Mapping {
    fn get_as_mapping(&self, key: &str) -> Option<&Mapping> {
        self.get(key).and_then(|val| val.as_mapping())
    }
}
