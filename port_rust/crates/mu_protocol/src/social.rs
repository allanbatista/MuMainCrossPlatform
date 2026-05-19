use crate::error::PacketCodecError as EncodeError;
use crate::wire::{encode_long_packet, encode_short_packet, fixed_bytes};

pub use crate::admin::{lahap_jewel_mix_request, LahapItemType, LahapMixType, LahapStackSize};

pub fn party_list_request() -> Result<Vec<u8>, EncodeError> {
    encode_short_packet(0xC1, 0x42, &[])
}

pub fn party_player_kick_request(player_index: u8) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet(0xC1, 0x43, &[player_index])
}

pub fn party_invite_request(target_player_id: u16) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet(0xC1, 0x40, &target_player_id.to_be_bytes())
}

pub fn party_invite_response(accepted: bool, requester_id: u16) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(3);
    payload.push(u8::from(accepted));
    payload.extend_from_slice(&requester_id.to_be_bytes());
    encode_short_packet(0xC1, 0x41, &payload)
}

pub fn letter_delete_request(letter_index: u16) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(3);
    payload.push(0x00);
    payload.extend_from_slice(&letter_index.to_le_bytes());
    encode_short_packet(0xC1, 0xC8, &payload)
}

pub fn letter_read_request(letter_index: u16) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(3);
    payload.push(0x00);
    payload.extend_from_slice(&letter_index.to_le_bytes());
    encode_short_packet(0xC1, 0xC7, &payload)
}

pub fn letter_list_request() -> Result<Vec<u8>, EncodeError> {
    encode_short_packet(0xC1, 0xC9, &[])
}

pub fn letter_send_request(
    letter_id: u32,
    receiver: impl AsRef<[u8]>,
    title: impl AsRef<[u8]>,
    rotation: u8,
    animation: u8,
    message_length: u16,
    message: impl AsRef<[u8]>,
) -> Result<Vec<u8>, EncodeError> {
    let message = message.as_ref();
    let mut payload = Vec::with_capacity(78 + message.len());
    payload.extend_from_slice(&letter_id.to_le_bytes());
    payload.extend_from_slice(&fixed_bytes::<10>(receiver));
    payload.extend_from_slice(&fixed_bytes::<60>(title));
    payload.push(rotation);
    payload.push(animation);
    payload.extend_from_slice(&message_length.to_le_bytes());
    payload.extend_from_slice(message);
    encode_long_packet(0xC4, 0xC5, &payload)
}

pub fn friend_list_request() -> Result<Vec<u8>, EncodeError> {
    encode_short_packet(0xC1, 0xC0, &[])
}

pub fn friend_add_request(friend_name: impl AsRef<[u8]>) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(10);
    payload.extend_from_slice(&fixed_bytes::<10>(friend_name));
    encode_short_packet(0xC1, 0xC1, &payload)
}

pub fn friend_delete(friend_name: impl AsRef<[u8]>) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(10);
    payload.extend_from_slice(&fixed_bytes::<10>(friend_name));
    encode_short_packet(0xC1, 0xC3, &payload)
}

pub fn chat_room_create_request(friend_name: impl AsRef<[u8]>) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(10);
    payload.extend_from_slice(&fixed_bytes::<10>(friend_name));
    encode_short_packet(0xC1, 0xCA, &payload)
}

pub fn friend_add_response(
    accepted: bool,
    friend_requester_name: impl AsRef<[u8]>,
) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(11);
    payload.push(u8::from(accepted));
    payload.extend_from_slice(&fixed_bytes::<10>(friend_requester_name));
    encode_short_packet(0xC1, 0xC2, &payload)
}

pub fn set_friend_online_state(online_state: bool) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet(0xC1, 0xC4, &[u8::from(online_state)])
}

pub fn chat_room_invitation_request(
    friend_name: impl AsRef<[u8]>,
    room_id: u16,
    request_id: u32,
) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(17);
    payload.extend_from_slice(&fixed_bytes::<10>(friend_name));
    payload.extend_from_slice(&room_id.to_be_bytes());
    payload.extend_from_slice(&request_id.to_be_bytes());
    encode_short_packet(0xC1, 0xCB, &payload)
}

#[cfg(test)]
mod tests {
    use super::{
        chat_room_create_request, chat_room_invitation_request, friend_add_request,
        friend_add_response, friend_delete, friend_list_request, letter_delete_request,
        letter_list_request, letter_read_request, letter_send_request, party_invite_request,
        party_invite_response, party_list_request, party_player_kick_request,
        set_friend_online_state,
    };
    use crate::wire::fixed_bytes;

    #[test]
    fn encodes_party_and_friend_packets() {
        assert_eq!(party_list_request().unwrap(), vec![0xC1, 0x03, 0x42]);
        assert_eq!(
            party_player_kick_request(7).unwrap(),
            vec![0xC1, 0x04, 0x43, 0x07]
        );
        assert_eq!(
            party_invite_request(0x1234).unwrap(),
            vec![0xC1, 0x05, 0x40, 0x12, 0x34]
        );
        assert_eq!(
            party_invite_response(true, 0x1234).unwrap(),
            vec![0xC1, 0x06, 0x41, 0x01, 0x12, 0x34]
        );
        assert_eq!(
            letter_delete_request(0x1234).unwrap(),
            vec![0xC1, 0x06, 0xC8, 0x00, 0x34, 0x12]
        );
        assert_eq!(
            letter_read_request(0x1234).unwrap(),
            vec![0xC1, 0x06, 0xC7, 0x00, 0x34, 0x12]
        );
        assert_eq!(letter_list_request().unwrap(), vec![0xC1, 0x03, 0xC9]);
        assert_eq!(friend_list_request().unwrap(), vec![0xC1, 0x03, 0xC0]);
        assert_eq!(
            friend_add_request(b"Alice").unwrap(),
            vec![0xC1, 0x0D, 0xC1, b'A', b'l', b'i', b'c', b'e', 0, 0, 0, 0, 0]
        );
        assert_eq!(
            friend_delete(b"Alice").unwrap(),
            vec![0xC1, 0x0D, 0xC3, b'A', b'l', b'i', b'c', b'e', 0, 0, 0, 0, 0]
        );
        assert_eq!(
            chat_room_create_request(b"Alice").unwrap(),
            vec![0xC1, 0x0D, 0xCA, b'A', b'l', b'i', b'c', b'e', 0, 0, 0, 0, 0]
        );
        assert_eq!(
            friend_add_response(true, b"Alice").unwrap(),
            vec![0xC1, 0x0E, 0xC2, 0x01, b'A', b'l', b'i', b'c', b'e', 0, 0, 0, 0, 0]
        );
        assert_eq!(
            set_friend_online_state(false).unwrap(),
            vec![0xC1, 0x04, 0xC4, 0x00]
        );
        assert_eq!(
            chat_room_invitation_request(b"Alice", 0x1234, 0x01020304).unwrap(),
            vec![
                0xC1, 0x13, 0xCB, b'A', b'l', b'i', b'c', b'e', 0, 0, 0, 0, 0, 0x12, 0x34, 0x01,
                0x02, 0x03, 0x04
            ]
        );
    }

    #[test]
    fn encodes_letter_send_request() {
        let packet = letter_send_request(0x01020304, b"Bob", b"Title", 1, 2, 2, b"hi").unwrap();
        assert_eq!(packet.len(), 84);
        assert_eq!(&packet[..4], &[0xC4, 0x00, 0x54, 0xC5]);
        assert_eq!(&packet[4..8], &0x01020304u32.to_le_bytes());
        assert_eq!(&packet[8..18], &fixed_bytes::<10>(b"Bob"));
        assert_eq!(&packet[18..78], &fixed_bytes::<60>(b"Title"));
        assert_eq!(packet[78], 1);
        assert_eq!(packet[79], 2);
        assert_eq!(&packet[80..82], &2u16.to_le_bytes());
        assert_eq!(&packet[82..], b"hi");
    }
}
