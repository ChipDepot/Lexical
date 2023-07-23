use tokio::time::Duration;

#[derive(Debug, Clone)]
pub enum Property {
    Time(Duration),
    Numeric(i32),
    Bool(bool),
    String(String),
}

impl ToString for Property {
    fn to_string(&self) -> String {
        match self {
            Property::Time(t) => t.as_secs_f64().to_string(),
            Property::Numeric(n) => n.to_string(),
            Property::Bool(b) => b.to_string(),
            Property::String(s) => s.to_string(),
        }
    }
}
