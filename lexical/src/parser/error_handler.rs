use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("missing key `{0}` from YAML file")]
    MissingKey(String),
    #[error("The argument for key '`{0}`' is not a valid String")]
    NotString(String),
    #[error("The string `{0}` is not a valid IpAddr")]
    NotIpAddr(String),
    #[error("Invalid component-type `{0}` in component `{1}`")]
    InvalidComponentType(String, String),
    #[error("Location `{0}` has no child locations or an ip")]
    NoLocationIp(String),
}
