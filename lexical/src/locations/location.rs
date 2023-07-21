
use std::{boxed::Box, collections::HashMap};

pub struct Location {
    pub locations: Option<Box<Location>>,
    pub name: String,
    pub ip: Option<String>,
    pub properties: HashMap<String, String>
}

impl Location {
    pub fn new() -> Location {
        todo!()
    }

    fn parse_locations() -> Result<Location, String> {
        todo!()
    }
}