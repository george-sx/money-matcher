use crate::types::*;
use moldudp64_core::sessions::SessionTable;
use moldudp64_core::types::*;
use std::ops::Add;
use tokio::net::UdpSocket;

pub struct MOLDPRODUCER {
    pub socket: UdpSocket,
    pub session_table: SessionTable,
    pub message_queue: MessageQueue,
}

impl MOLDPRODUCER {
    pub async fn new() -> Self {
        MOLDPRODUCER {
            socket: UdpSocket::bind("0.0.0.0:9000").await.unwrap(),
            session_table: SessionTable::new(),
            message_queue: Vec::new(),
        }
    }

    pub fn make_packet(&mut self, messages: MessageQueue) -> Packet {
        let session_id = self.session_table.get_current_session();
        let sequence_number = self.session_table.next_sequence(session_id);
        let message_count = (messages.len() as u16).to_be_bytes();
        let header = Header {
            session_id,
            sequence_number,
            message_count,
        };

        let mut message_blocks: MessageBlocks = Vec::new();

        for msg in messages {
            let block: MessageBlock = MessageBlock {
                message_length: (msg.len() as u16).to_be_bytes(),
                message_data: msg,
            };
            message_blocks.push(block);
        }

        let packet = Packet {
            header,
            message_blocks,
        };

        packet
    }

    pub fn flush(&mut self) -> Packet {
        let messages = std::mem::take(&mut self.message_queue);
        self.make_packet(messages)
    }

    pub fn enqueue_message(&mut self, message: MessageData) {
        self.message_queue.push(message);
    }
}

#[tokio::test]
async fn test_packet_engine() {
    let mut mold: MOLDPRODUCER = MOLDPRODUCER::new().await;

    for i in 1..=100 {
        let message: String = i.to_string().add(" TEST");
        mold.enqueue_message(message.as_bytes().to_vec());
    }

    let packet = mold.flush();

    let header = packet.header;
    let message_blocks = packet.message_blocks;
    println!(
        "Header Session ID: {:?} {:?}",
        std::str::from_utf8(&header.session_id).unwrap(),
        header.session_id
    );
    println!(
        "Header Sequence Number: {:?} {:?}",
        u64::from_be_bytes(header.sequence_number) as usize,
        header.sequence_number
    );
    println!(
        "Header Message Count: {:?} {:?}",
        u16::from_be_bytes(header.message_count) as usize,
        header.message_count
    );

    let mut k = 1;
    for msg in message_blocks {
        println!(
            "Message {:?} : {:?} {:?}",
            k,
            std::str::from_utf8(&msg.message_data).unwrap(),
            msg.message_data
        );

        k += 1;
    }
}
