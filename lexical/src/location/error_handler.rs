use thiserror::Error;

#[derive(Error, Debug)]
pub enum LocationError {
    #[error("Location `{0}` has no child locations or an ip")]
    NoLocationIp(String),
}
