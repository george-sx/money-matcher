pub mod packet;
pub mod sessions;
pub mod tests;
pub mod types;
use crate::types::*;

#[test]
fn test_message_block() {
    let message = "Hello, World!";
    let message_length: MessageLength = (message.len() as u16).to_be_bytes();
    let message_data: MessageData = Bytes::copy_from_slice(message.as_bytes());

    println!("Original Message: {:?}", message);
    println!("Message Length: {:?}", message_length);
    println!("Message Data: {:?}", message_data);

    let block = MessageBlock {
        message_length,
        message_data: message_data,
    };

    let reconstructed_length = u16::from_be_bytes(block.message_length);
    let reconstructed_data = &block.message_data;

    println!("Reconstructed Length: {:?}", reconstructed_length);
    println!(
        "Reconstructed Data: {:?}",
        std::str::from_utf8(reconstructed_data).unwrap()
    );
}
