use super::error_handler::ParseError;
use serde_yaml::Mapping;

pub trait FromMapping {
    type T;

    fn from_mapping(mapp: &Mapping) -> Result<Self::T, ParseError>;
}

pub trait AsString {
    fn get_as_string(&self, key: &str) -> Result<String, ParseError>;
}

pub trait GetKeys {
    type T;

    fn as_vector(&self) -> Vec<Self::T>;
}

pub trait AsMapping {
    fn get_as_mapping(&self, key: &str) -> Option<&Mapping>;
}
