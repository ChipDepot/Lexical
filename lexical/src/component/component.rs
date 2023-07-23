use std::collections::HashMap;

use serde_yaml::{Mapping, Value};

use super::output::IoTData;
use super::properties::Property;

#[derive(Debug, Clone)]
pub struct Component {
    pub name: String,
    pub properties: HashMap<String, Property>,
    pub outputs: HashMap<String, IoTData>,
}

impl Component {
    pub fn new(name: String) -> Component {
        Component {
            name,
            properties: HashMap::new(),
            outputs: HashMap::new(),
        }
    }

    pub fn from_mapping() {
        todo!()
    }
}
