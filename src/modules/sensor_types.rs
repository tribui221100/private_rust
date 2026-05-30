

#[derive(PartialEq)]
pub struct SensorData{
    pub timestamp: u32,
    pub id: u8,
    pub value: u32
}

#[derive(PartialEq, Debug)]
pub enum SensorError{
    InvalidLenght,
    BadSensorId,
    InvalidTimestamp
}