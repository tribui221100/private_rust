
pub use crate::modules::sensor_types::SensorData;
pub use crate::modules::sensor_types::SensorError;

impl SensorData {

    pub fn parse(slice: &[u8]) -> Result<SensorData,SensorError>{
        
        // Data structure [id,4 bytes of timestamp, 4 bytes of data]
        
        //1. Sensor Data length check
        if slice.len() < 9{
            return Err(SensorError::InvalidLenght);
        }

        //2. Extract data
        let mut id;
        let mut byte_id : [u8;1];
        if slice[0] > 0xFE {
            return Err(SensorError::BadSensorId);
        }
        else
        {
            byte_id = slice[0..1].try_into().unwrap();
        }
        id = u8::from_be_bytes(byte_id);

        let byte_chunks_value: [u8; 4] = slice[1..5].try_into().unwrap();
        let value = u32::from_le_bytes(byte_chunks_value);

        let byte_chunks_ts: [u8; 4] = slice[5..9].try_into().unwrap();
        let timestamp = u32::from_le_bytes(byte_chunks_ts);

        if timestamp < 0{
            return Err(SensorError::InvalidTimestamp);
        }
        

        Ok(SensorData { timestamp, id, value })
    }
}


