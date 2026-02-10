use crate::types::*;
impl Packet {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.header.session_id);
        bytes.extend_from_slice(&self.header.sequence_number);
        bytes.extend_from_slice(&self.header.message_count);

        for message in &self.message_blocks {
            bytes.extend_from_slice(&message.message_length);
            bytes.extend_from_slice(&message.message_data);
        }

        bytes
    }

    pub fn from_bytes(mut bytes: &[u8]) -> Packet {
        let mut session_id: SessionID = [0u8; 10];
        session_id.copy_from_slice(&bytes[..10]);
        bytes = &bytes[10..];

        let mut sequence_number: SequenceNumber = [0u8; 8];
        sequence_number.copy_from_slice(&bytes[..8]);
        bytes = &bytes[8..];

        let mut message_count: MessageCount = [0u8; 2];
        message_count.copy_from_slice(&bytes[..2]);
        bytes = &bytes[2..];

        let mc = u16::from_be_bytes(message_count) as usize;
        let mut message_blocks = Vec::with_capacity(mc);

        for _ in 0..mc {
            let mut message_length: MessageLength = [0u8; 2];
            message_length.copy_from_slice(&bytes[..2]);
            bytes = &bytes[2..];

            let ml = u16::from_be_bytes(message_length) as usize;
            let message_data = bytes[..ml].to_vec();
            bytes = &bytes[ml..];

            message_blocks.push(MessageBlock {
                message_length,
                message_data,
            });
        }

        Packet {
            header: Header {
                session_id,
                sequence_number,
                message_count,
            },
            message_blocks,
        }
    }
}
