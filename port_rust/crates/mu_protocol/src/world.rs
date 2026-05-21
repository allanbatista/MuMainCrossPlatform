use crate::error::PacketCodecError as EncodeError;
use crate::wire::{encode_short_packet, encode_short_packet_with_subcode, fixed_bytes};

pub fn enter_gate_request(
    gate_number: u16,
    teleport_target_x: u8,
    teleport_target_y: u8,
) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(4);
    payload.extend_from_slice(&gate_number.to_le_bytes());
    payload.push(teleport_target_x);
    payload.push(teleport_target_y);
    encode_short_packet_with_subcode(0xC3, 0x1C, 0x00, &payload)
}

pub fn enter_gate_request_075(
    gate_number: u8,
    teleport_target_x: u8,
    teleport_target_y: u8,
) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet(
        0xC3,
        0x1C,
        &[gate_number, teleport_target_x, teleport_target_y],
    )
}

pub fn teleport_target(
    target_id: u16,
    teleport_target_x: u8,
    teleport_target_y: u8,
) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(4);
    payload.extend_from_slice(&target_id.to_le_bytes());
    payload.push(teleport_target_x);
    payload.push(teleport_target_y);
    encode_short_packet(0xC3, 0xB0, &payload)
}

pub fn warp_command_request(
    command_key: u32,
    warp_info_index: u16,
) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(6);
    payload.extend_from_slice(&command_key.to_le_bytes());
    payload.extend_from_slice(&warp_info_index.to_le_bytes());
    encode_short_packet_with_subcode(0xC1, 0x8E, 0x02, &payload)
}

#[allow(clippy::too_many_arguments)]
pub fn server_change_authentication(
    account_xor3: impl AsRef<[u8]>,
    character_name_xor3: impl AsRef<[u8]>,
    auth_code1: u32,
    auth_code2: u32,
    auth_code3: u32,
    auth_code4: u32,
    tick_count: u32,
    client_version: impl AsRef<[u8]>,
    client_serial: impl AsRef<[u8]>,
) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(65);
    payload.extend_from_slice(&fixed_bytes::<12>(account_xor3));
    payload.extend_from_slice(&fixed_bytes::<12>(character_name_xor3));
    payload.extend_from_slice(&auth_code1.to_le_bytes());
    payload.extend_from_slice(&auth_code2.to_le_bytes());
    payload.extend_from_slice(&auth_code3.to_le_bytes());
    payload.extend_from_slice(&auth_code4.to_le_bytes());
    payload.extend_from_slice(&tick_count.to_le_bytes());
    payload.extend_from_slice(&fixed_bytes::<5>(client_version));
    payload.extend_from_slice(&fixed_bytes::<16>(client_serial));
    encode_short_packet_with_subcode(0xC3, 0xB1, 0x01, &payload)
}

#[cfg(test)]
mod tests {
    use super::{
        enter_gate_request, enter_gate_request_075, server_change_authentication, teleport_target,
        warp_command_request,
    };

    #[test]
    fn encodes_world_packets() {
        assert_eq!(
            enter_gate_request(0x1234, 5, 6).unwrap(),
            vec![0xC3, 0x08, 0x1C, 0x00, 0x34, 0x12, 0x05, 0x06]
        );
        assert_eq!(
            enter_gate_request_075(7, 5, 6).unwrap(),
            vec![0xC3, 0x06, 0x1C, 0x07, 0x05, 0x06]
        );
        assert_eq!(
            teleport_target(0x1234, 5, 6).unwrap(),
            vec![0xC3, 0x07, 0xB0, 0x34, 0x12, 0x05, 0x06]
        );
        assert_eq!(
            warp_command_request(0x01020304, 0x1234).unwrap(),
            vec![0xC1, 0x0A, 0x8E, 0x02, 0x04, 0x03, 0x02, 0x01, 0x34, 0x12]
        );

        let packet =
            server_change_authentication(b"account", b"hero", 1, 2, 3, 4, 5, b"1.0.0", b"serial")
                .unwrap();

        assert_eq!(packet.len(), 69);
        assert_eq!(&packet[..4], &[0xC3, 0x45, 0xB1, 0x01]);
        assert_eq!(&packet[4..16], b"account\0\0\0\0\0");
        assert_eq!(&packet[16..28], b"hero\0\0\0\0\0\0\0\0");
        assert_eq!(&packet[48..53], b"1.0.0");
        assert_eq!(&packet[53..69], b"serial\0\0\0\0\0\0\0\0\0\0");
    }
}
