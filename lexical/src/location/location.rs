use serde_yaml::mapping::Mapping;

use crate::parser::error_handler::ParseError;
use crate::parser::parser;
use crate::parser::FromMapping;

use starduck::location::Location;

impl FromMapping for Location {
    type T = Location;

    fn from_mapping(mapping: &Mapping) -> Result<Location, ParseError> {
        let name = parser::get_as_string(mapping, Location::NAME)?;
        let ip_string = match parser::get_as_string(&mapping, Location::IP) {
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
