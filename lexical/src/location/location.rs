use serde_yaml::mapping::Mapping;
use std::{boxed::Box, collections::HashMap, net::IpAddr};

use crate::location::keywords::Keywords;
use crate::parser::error_handler::ParseError;
use crate::parser::parser;

#[derive(Debug, Clone)]
pub struct Location {
    pub name: String,
    pub ip: Option<IpAddr>,
    pub locations: HashMap<String, Box<Location>>,
    pub properties: HashMap<String, String>,
}

impl Location {
    pub fn new(name: String, ip: Option<IpAddr>) -> Location {
        return Location {
            locations: HashMap::new(),
            name,
            ip,
            properties: HashMap::new(),
        };
    }

    pub fn from_mapping(
        mapping: &Mapping
    ) -> Result<Location, ParseError> {

        let name = parser::extract_value_as_string(mapping, Keywords::NAME)?;
        let ip_string = match parser::extract_value_as_string(&mapping, Keywords::IP) {
            Ok(ip) => Some(ip),
            Err(ParseError::MissingKey(_)) => None,
            Err(e) => {
                panic!("{}", e.to_string())
            }
        };

        let ip = match ip_string {
            Some(ip) => parser::parse_ip(ip)?,
            None => None,
        };
        
        Ok(Location::new(name, ip))
    }

}

impl ToString for Location {
    fn to_string(&self) -> String {
        format!("name: {}\nip: {:?}\nlocation keys: {:?}", self.name, self.ip, self.locations.keys())
    }
}
