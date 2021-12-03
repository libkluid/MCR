use crate::Error;

pub trait Encode {
    fn encode(&self, buffer: &mut Vec<u8>) -> usize;
}

impl Encode for i32 {
    fn encode(&self, buffer: &mut Vec<u8>) -> usize {
        buffer.extend(self.to_le_bytes());
        std::mem::size_of::<Self>()
    }
}

impl Encode for String {
    fn encode(&self, buffer: &mut Vec<u8>) -> usize {
        buffer.extend(self.as_bytes());
        self.len()
    }
}

impl Encode for [u8; 2] {
    fn encode(&self, buffer: &mut Vec<u8>) -> usize {
        buffer.extend(self);
        2
    }
}

pub trait Decode: Sized {
    fn decode(buffer: &[u8]) -> Result<Self, Error>;
}

impl Decode for i32 {
    fn decode(buffer: &[u8]) -> Result<Self, Error> {
        let size = std::mem::size_of::<Self>();
        let array: [u8; 4] = TryFrom::try_from(&buffer[..size])
            .map_err(|_| Error::Decode("i32", 4, buffer.len()))?;
        
        Ok(i32::from_le_bytes(array))
    }
}

impl Decode for String {
    fn decode(buffer: &[u8]) -> Result<Self, Error> {
        let s = String::from_utf8_lossy(buffer).to_string();
        Ok(s)
    }
}

impl Decode for [u8; 2] {
    fn decode(buffer: &[u8]) -> Result<Self, Error> {
        let array: [u8; 2] = TryFrom::try_from(&buffer[..2])
            .map_err(|_| Error::Decode("[u8; 2]", 2, buffer.len()))?;
        Ok(array)
    }
}
