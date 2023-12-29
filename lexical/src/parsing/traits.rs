use anyhow::Result;
use serde_yaml::Mapping;

pub trait FromMapping {
    fn from_mapping(mapp: &Mapping) -> Result<Self>
    where
        Self: Sized;
}

pub trait AsString {
    fn get_as_string(&self, key: &str) -> Option<String>;
}

pub trait GetKeys<T> {
    fn as_vector(&self) -> Vec<T>;
}

pub trait AsMapping {
    fn get_as_mapping(&self, key: &str) -> Option<&Mapping>;
}
