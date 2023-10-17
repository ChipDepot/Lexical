use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("missing key `{0}`")]
    MissingKey(String),
    #[error("missing argument for flag `{0}`")]
    MissingArgument(String),
    #[error("The argument for key '`{0}`' is not a valid String")]
    NotString(String),
    #[error("The string `{0}` is not a valid IpAddr")]
    NotIpAddr(String),
    // #[error("Invalid component-type `{0}` in component `{1}`")]
    // InvalidComponentType(String, String),
    #[error("Invalid property `{0}` in component")]
    InvalidProperty(String),
    #[error("Location `{0}` has no child locations or an ip")]
    NoLocationIp(String),
    #[error("Couldn't parse `{0}`")]
    CouldntParse(String),
}
