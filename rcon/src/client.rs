use std::io::{Read, Write};
use std::net::TcpStream;
use std::net::ToSocketAddrs;
use std::sync::atomic::{AtomicI32, Ordering};

use crate::Error;
use crate::codec::Decode;
use crate::message::{Packet, PacketType};

struct Sequence(AtomicI32);
impl Sequence {
    fn new() -> Self {
        Sequence(AtomicI32::new(1))
    }

    fn advance(&mut self) -> i32 {
        self.0.fetch_add(1, Ordering::SeqCst)
    }
}

pub struct Client {
    conn: TcpStream,
    sequence: Sequence,
    tx_buffer: Vec<u8>,
    rx_buffer: Vec<u8>,
} 

impl Client {

    // Max size is documented at "https://wiki.vg/RCON#Fragmentation"
    const TX_MAX_SIZE: usize = 1460;
    const RX_MAX_SIZE: usize = 4110;

    fn new<S: ToSocketAddrs>(addr: S) -> Result<Self, Error> {
        let conn = TcpStream::connect(addr)?;
        let sequence = Sequence::new();
        let tx_buffer = Vec::with_capacity(Self::TX_MAX_SIZE);
        let rx_buffer = Vec::with_capacity(Self::RX_MAX_SIZE);

        Ok(Client { conn, sequence, tx_buffer, rx_buffer, })
    }

    fn send(&mut self, packet: Packet) -> Result<Packet, Error> {
        // Send packet.
        self.tx_buffer.clear();
        let packet_size = packet.encode(&mut self.tx_buffer);
        self.conn.write(&self.tx_buffer[..packet_size])?;

        // Receive packet.
        self.rx_buffer.clear();
        unsafe {
            self.rx_buffer.set_len(4);
        }

        self.conn.read(&mut self.rx_buffer[..4])?;
        let packet_size = i32::decode(&self.rx_buffer[..4])? as usize;
        unsafe { self.rx_buffer.set_len(4 + packet_size); }
        self.conn.read(&mut self.rx_buffer[4..])?;

        let response = Packet::decode(&self.rx_buffer)?;
        Ok(response)
    }

    fn authenticate<S: Into<String>>(&mut self, password: S) -> Result<(), Error> {
        // Send packet.
        let request_id = self.sequence.advance();
        let auth_packet = Packet::new_auth(request_id, password);

        let response = self.send(auth_packet)?;

        match response.packet_type() {
            PacketType::Unauthorized => Err(Error::InvalidPassword),
            PacketType::Command => Ok(()),
            v => unreachable!("Received invalid packet type: {}", v as i32),
        }
    }

    pub fn connect<A: ToSocketAddrs, S: Into<String>>(addr: A, password: S) -> Result<Self, Error> {
        let mut client = Self::new(addr)?;
        client.authenticate(password)?;
        Ok(client)
    }

    pub fn execute<C: Into<String>>(&mut self, command: C) -> Result<String, Error> {
        let request_id = self.sequence.advance();
        let command_packet = Packet::new_command(request_id, command);

        let response = self.send(command_packet)?;

        match response.packet_type() {
            PacketType::Unauthorized => Err(Error::Unauthorised),
            PacketType::Response => Ok(response.payload()),
            v => unreachable!("Received invalid packet type: {}", v as i32),
        }
    }
}
