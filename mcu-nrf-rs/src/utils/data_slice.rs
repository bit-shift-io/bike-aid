use crate::utils::globals;


#[derive(Clone, defmt::Format)]
pub struct DataSlice {
    pub data: [u8; globals::BLE_BUFFER_LENGTH],
    pub data_len: usize,
}


impl DataSlice {
    pub fn new(data: &[u8]) -> DataSlice {
        let data_len = data.len();
        if data_len > globals::BLE_BUFFER_LENGTH {
            panic!("Data length exceeds buffer size");
        }

        let mut buffer = [0u8; globals::BLE_BUFFER_LENGTH];
        buffer[..data_len].copy_from_slice(data);

        let data_len = data.len();

        DataSlice {
            data: buffer,
            data_len,
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.data[..self.data_len]
    }

    pub fn as_string(&self) -> &str {
        let bytes = self.as_bytes();
        core::str::from_utf8(bytes).unwrap()
    }
}