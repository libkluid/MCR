use crate::codec::{Encode, Decode};
use crate::Error;

#[allow(dead_code)]
#[repr(i32)]
#[derive(Clone, Copy, Debug)]
pub(crate) enum PacketType {
    Response     = 0x00,
    Command      = 0x02,
    Login        = 0x03,
    Unauthorized = i32::MIN,
}

#[derive(Clone, Debug)]
pub struct Packet {
    length: i32,
    request_id: i32,
    packet_type: PacketType,
    payload: String,
    pad: &'static [u8; 2],
}

impl Packet {
    const HEADER_SIZE: usize = 10;
    const PAD: [u8; 2] = [0; 2];

    // Creates login packet.
    #[inline]
    pub(crate) fn new_auth<P: Into<String>>(request_id: i32, password: P) -> Packet {
        Self::new(request_id, PacketType::Login, password.into())
    }

    // Creates command packet.
    #[inline]
    pub(crate) fn new_command<C: Into<String>>(request_id: i32, command: C) -> Packet {
        Self::new(request_id, PacketType::Command, command.into())
    }

    // Creates rew packet.
    #[inline]
    fn new(request_id: i32, packet_type: PacketType, payload: String) -> Self {
        let length = (Self::HEADER_SIZE + payload.len()) as i32;
        let pad = &Self::PAD;

        Packet { 
            length,
            request_id,
            packet_type,
            payload,
            pad,
        }
    }

    pub(crate) fn packet_type(&self) -> PacketType {
        self.packet_type
    }

    pub(crate) fn payload(self) -> String {
        self.payload
    }

    pub(crate) fn encode(&self, buffer: &mut Vec<u8>) -> usize {
        let mut nwrite = 0;
        nwrite += self.length.to_le().encode(buffer);
        nwrite += self.request_id.to_le().encode(buffer);
        nwrite += (self.packet_type as i32).to_le().encode(buffer);
        nwrite += self.payload.encode(buffer);
        nwrite += self.pad.encode(buffer);
        nwrite
    }

    pub(crate) fn decode(buffer: &[u8]) -> Result<Self, Error> {
        if buffer.len() < Self::HEADER_SIZE {
            return Err(Error::InvalidPacketLength);
        }

        let null_position = (&buffer[12..])
            .iter()
            .position(|&byte| byte == 0)
            .ok_or(Error::InvalidPacketLength)?;

        let length = i32::decode(&buffer[0..4])?;
        let request_id = i32::decode(&buffer[4..8])?;
        let packet_type = unsafe {
            let raw_value = i32::decode(&buffer[8..12])?;
            std::mem::transmute(raw_value)
        };
        let payload = String::decode(&buffer[12..(12 + null_position)])?;
        let _pad = <[u8; 2]>::decode(&buffer[(12 + null_position)..(12 + null_position + 2)])?;

        Ok(Packet {
            length,
            request_id,
            packet_type,
            payload,
            pad: &Self::PAD,
        }
        )

    }
}
