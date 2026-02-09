use std::collections::HashMap;

pub struct MOLDUDP64 {
    pub socket: tokio::net::UdpSocket,
}

pub struct Header {
    pub session_id: [u8; 10],
    pub sequence_number: [u8; 8],
    pub message_count: [u8; 2],
}

pub struct MessageBlock {
    pub message_length: [u8; 2],
    pub message_data: Vec<u8>,
}

pub struct Packet {
    pub header: Header,
    pub messages: Vec<MessageBlock>,
}

pub struct RequestPacket {
    pub session_id: [u8; 10],
    pub sequence_number: [u8; 8],
    pub message_count: [u8; 2],
}

pub struct SessionTable {
    pub sessions: HashMap<[u8; 10], [u8; 8]>,
}
