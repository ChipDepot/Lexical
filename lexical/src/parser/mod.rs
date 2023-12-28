pub mod parser;
pub mod traits;

pub use traits::FromMapping;

mod mapping;

pub use parser::parse_yaml;

pub const NAME: &str = "name";
pub const APPLICATION: &str = "application";
pub const LOCATIONS: &str = "locations";
pub const IP: &str = "ip";
