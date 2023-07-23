use tokio::time::Instant;

#[derive(Debug, Clone)]

pub enum IoTData {
    Integer(i32),
    Float(f32),
    Boolean(bool),
    Text(String),
    Timestamp(Instant),
}
