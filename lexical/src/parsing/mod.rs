mod mapping;
mod parser;
mod traits;

pub use traits::{AsMapping, AsString, FromMapping, GetKeys};

pub use parser::parse_yaml;

pub const NAME: &str = "name";
pub const APPLICATION: &str = "application";
pub const LOCATIONS: &str = "locations";
pub const DATA_REQUIREMENTS: &str = "data-requirements";
pub const IP: &str = "ip";
pub const OUTPUT: &str = "output";
pub const TIMEOUT: &str = "timeout";
pub const COUNT: &str = "count";
