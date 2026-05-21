use std::convert::TryFrom;

use crate::error::PacketCodecError as EncodeError;
use crate::wire::{
    encode_short_packet, encode_short_packet_with_subcode, fixed_bytes, xor3_encrypt,
};

pub fn authenticate(room_id: u16, token: impl AsRef<[u8]>) -> Result<Vec<u8>, EncodeError> {
    let mut token = fixed_bytes::<10>(token);
    xor3_encrypt(&mut token, 6);

    let mut payload = Vec::with_capacity(12);
    payload.extend_from_slice(&room_id.to_le_bytes());
    payload.extend_from_slice(&token);
    encode_short_packet_with_subcode(0xC1, 0x00, 0x00, &payload)
}

pub fn chat_room_client_joined(
    client_index: u8,
    name: impl AsRef<[u8]>,
) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(11);
    payload.push(client_index);
    payload.extend_from_slice(&fixed_bytes::<10>(name));
    encode_short_packet_with_subcode(0xC1, 0x01, 0x00, &payload)
}

pub fn chat_room_client_left(
    client_index: u8,
    name: impl AsRef<[u8]>,
) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(11);
    payload.push(client_index);
    payload.extend_from_slice(&fixed_bytes::<10>(name));
    encode_short_packet_with_subcode(0xC1, 0x01, 0x01, &payload)
}

pub fn leave_chat_room() -> Result<Vec<u8>, EncodeError> {
    encode_short_packet(0xC1, 0x01, &[])
}

pub fn keep_alive() -> Result<Vec<u8>, EncodeError> {
    encode_short_packet(0xC1, 0x05, &[])
}

pub fn chat_message(sender_index: u8, message: impl AsRef<[u8]>) -> Result<Vec<u8>, EncodeError> {
    let message = message.as_ref();
    let message_length = u8::try_from(message.len()).map_err(|_| EncodeError::PacketTooLarge {
        size: message.len(),
    })?;

    let mut encrypted_message = message.to_vec();
    xor3_encrypt(&mut encrypted_message, 5);

    let mut payload = Vec::with_capacity(2 + encrypted_message.len());
    payload.push(sender_index);
    payload.push(message_length);
    payload.extend_from_slice(&encrypted_message);
    encode_short_packet(0xC1, 0x04, &payload)
}

pub fn public_chat_message(
    character: impl AsRef<[u8]>,
    message: impl AsRef<[u8]>,
) -> Result<Vec<u8>, EncodeError> {
    let message = message.as_ref();
    let mut payload = Vec::with_capacity(10 + message.len());
    payload.extend_from_slice(&fixed_bytes::<10>(character));
    payload.extend_from_slice(message);
    encode_short_packet(0xC1, 0x00, &payload)
}

pub fn whisper_message(
    receiver_name: impl AsRef<[u8]>,
    message: impl AsRef<[u8]>,
) -> Result<Vec<u8>, EncodeError> {
    let message = message.as_ref();
    let mut payload = Vec::with_capacity(10 + message.len());
    payload.extend_from_slice(&fixed_bytes::<10>(receiver_name));
    payload.extend_from_slice(message);
    encode_short_packet(0xC1, 0x02, &payload)
}

#[cfg(test)]
mod tests {
    use super::{
        authenticate, chat_message, chat_room_client_joined, chat_room_client_left, keep_alive,
        leave_chat_room, public_chat_message, whisper_message,
    };

    #[test]
    fn encodes_chat_room_packets() {
        assert_eq!(authenticate(7, b"12345").unwrap().len(), 16);
        assert_eq!(
            chat_room_client_joined(3, b"Alice").unwrap(),
            vec![0xC1, 0x0F, 0x01, 0x00, 0x03, b'A', b'l', b'i', b'c', b'e', 0, 0, 0, 0, 0]
        );
        assert_eq!(
            chat_room_client_left(3, b"Alice").unwrap(),
            vec![0xC1, 0x0F, 0x01, 0x01, 0x03, b'A', b'l', b'i', b'c', b'e', 0, 0, 0, 0, 0]
        );
        assert_eq!(leave_chat_room().unwrap(), vec![0xC1, 0x03, 0x01]);
        assert_eq!(keep_alive().unwrap(), vec![0xC1, 0x03, 0x05]);
    }

    #[test]
    fn encodes_chat_messages() {
        let packet = chat_message(9, b"abc").unwrap();
        assert_eq!(&packet[..5], &[0xC1, 0x08, 0x04, 0x09, 0x03]);
        assert_eq!(
            &public_chat_message(b"Hero", b"hello").unwrap()[..13],
            &[0xC1, 0x12, 0x00, b'H', b'e', b'r', b'o', 0, 0, 0, 0, 0, 0]
        );
        assert_eq!(
            &whisper_message(b"Hero", b"hi").unwrap()[..13],
            &[0xC1, 0x0F, 0x02, b'H', b'e', b'r', b'o', 0, 0, 0, 0, 0, 0]
        );
    }
}
