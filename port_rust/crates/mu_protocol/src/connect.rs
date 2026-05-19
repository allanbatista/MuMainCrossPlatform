use std::convert::TryFrom;

pub use crate::codec::{decode_packet, encode_packet};
pub use crate::error::PacketCodecError as EncodeError;
pub use crate::frame::PacketFrame as PacketView;

use crate::wire::{
    encode_long_packet_with_subcode, encode_short_packet, encode_short_packet_with_subcode,
    fixed_bytes, xor3_encrypt,
};

pub const CONNECT_SERVER_HEADCODE: u8 = 0xF4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ServerEntry {
    pub connect_index: u16,
    pub percent: u8,
}

impl ServerEntry {
    pub const fn new(connect_index: u16, percent: u8) -> Self {
        Self {
            connect_index,
            percent,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LegacyServerEntry {
    pub server_id: u8,
    pub percent: u8,
}

impl LegacyServerEntry {
    pub const fn new(server_id: u8, percent: u8) -> Self {
        Self { server_id, percent }
    }
}

pub fn is_server_list_request(bytes: &[u8]) -> bool {
    matches!(decode_packet(bytes), Ok(packet) if {
        packet.headcode == CONNECT_SERVER_HEADCODE && packet.subcode == 0x06
    })
}

pub fn is_server_list_request_old(bytes: &[u8]) -> bool {
    matches!(decode_packet(bytes), Ok(packet) if {
        packet.headcode == CONNECT_SERVER_HEADCODE && packet.subcode == 0x02
    })
}

pub fn hello() -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, 0x00, 0x01, &[])
}

pub fn server_list_request() -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, CONNECT_SERVER_HEADCODE, 0x06, &[])
}

pub fn server_list_request_old() -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, CONNECT_SERVER_HEADCODE, 0x02, &[])
}

pub fn connection_info_request_075(server_id: u8) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, CONNECT_SERVER_HEADCODE, 0x03, &[server_id])
}

pub fn connection_info_request(server_id: u16) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(
        0xC1,
        CONNECT_SERVER_HEADCODE,
        0x03,
        &server_id.to_le_bytes(),
    )
}

pub fn connection_info(ip_address: impl AsRef<[u8]>, port: u16) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(18);
    payload.extend_from_slice(&fixed_bytes::<16>(ip_address));
    payload.extend_from_slice(&port.to_le_bytes());
    encode_short_packet_with_subcode(0xC1, CONNECT_SERVER_HEADCODE, 0x03, &payload)
}

pub fn encode_server_list_response(entries: &[ServerEntry]) -> Result<Vec<u8>, EncodeError> {
    let payload = build_server_list_payload(entries)?;
    encode_long_packet_with_subcode(0xC2, CONNECT_SERVER_HEADCODE, 0x06, &payload)
}

pub fn encode_server_list_response_old(
    entries: &[LegacyServerEntry],
) -> Result<Vec<u8>, EncodeError> {
    let payload = build_server_list_payload_old(entries)?;
    encode_long_packet_with_subcode(0xC2, CONNECT_SERVER_HEADCODE, 0x02, &payload)
}

pub fn patch_check_request(
    major_version: u8,
    minor_version: u8,
    patch_version: u8,
) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet(0xC1, 0x02, &[major_version, minor_version, patch_version])
}

pub fn patch_version_okay() -> Result<Vec<u8>, EncodeError> {
    encode_short_packet(0xC1, 0x02, &[0x00])
}

pub fn client_needs_patch(
    patch_version: u8,
    patch_address: impl AsRef<[u8]>,
) -> Result<Vec<u8>, EncodeError> {
    let mut patch_address = fixed_bytes::<132>(patch_address);
    xor3_encrypt(&mut patch_address, 6);

    let mut payload = Vec::with_capacity(134);
    payload.push(patch_version);
    payload.push(0x00);
    payload.extend_from_slice(&patch_address);

    encode_short_packet_with_subcode(0xC1, 0x05, 0x01, &payload)
}

fn build_server_list_payload(entries: &[ServerEntry]) -> Result<Vec<u8>, EncodeError> {
    let payload_bytes = entries
        .len()
        .checked_mul(3)
        .and_then(|value| value.checked_add(2))
        .ok_or(EncodeError::PacketTooLarge { size: usize::MAX })?;
    let packet_bytes = payload_bytes
        .checked_add(5)
        .ok_or(EncodeError::PacketTooLarge { size: usize::MAX })?;
    let count = u16::try_from(entries.len())
        .map_err(|_| EncodeError::PacketTooLarge { size: packet_bytes })?;

    let mut payload = Vec::with_capacity(payload_bytes);
    payload.extend_from_slice(&count.to_be_bytes());

    for entry in entries {
        payload.extend_from_slice(&entry.connect_index.to_le_bytes());
        payload.push(entry.percent);
    }

    Ok(payload)
}

fn build_server_list_payload_old(entries: &[LegacyServerEntry]) -> Result<Vec<u8>, EncodeError> {
    let payload_bytes = entries
        .len()
        .checked_mul(2)
        .and_then(|value| value.checked_add(1))
        .ok_or(EncodeError::PacketTooLarge { size: usize::MAX })?;
    let packet_bytes = payload_bytes
        .checked_add(5)
        .ok_or(EncodeError::PacketTooLarge { size: usize::MAX })?;
    let count = u8::try_from(entries.len())
        .map_err(|_| EncodeError::PacketTooLarge { size: packet_bytes })?;

    let mut payload = Vec::with_capacity(payload_bytes);
    payload.push(count);

    for entry in entries {
        payload.push(entry.server_id);
        payload.push(entry.percent);
    }

    Ok(payload)
}

#[cfg(test)]
mod tests {
    use super::{
        client_needs_patch, connection_info, connection_info_request, connection_info_request_075,
        encode_server_list_response, encode_server_list_response_old, hello,
        is_server_list_request, is_server_list_request_old, patch_check_request,
        patch_version_okay, server_list_request, server_list_request_old, LegacyServerEntry,
        ServerEntry,
    };

    #[test]
    fn encodes_connect_packets() {
        assert_eq!(hello().unwrap(), vec![0xC1, 0x04, 0x00, 0x01]);
        assert_eq!(server_list_request().unwrap(), vec![0xC1, 0x04, 0xF4, 0x06]);
        assert_eq!(
            server_list_request_old().unwrap(),
            vec![0xC1, 0x04, 0xF4, 0x02]
        );
        assert_eq!(
            connection_info_request_075(7).unwrap(),
            vec![0xC1, 0x05, 0xF4, 0x03, 0x07]
        );
        assert_eq!(
            connection_info_request(7).unwrap(),
            vec![0xC1, 0x06, 0xF4, 0x03, 0x07, 0x00]
        );
        assert_eq!(
            connection_info(b"127.0.0.1", 55901).unwrap(),
            vec![
                0xC1, 0x16, 0xF4, 0x03, b'1', b'2', b'7', b'.', b'0', b'.', b'0', b'.', b'1', 0, 0,
                0, 0, 0, 0, 0, 0x5D, 0xDA
            ]
        );
    }

    #[test]
    fn encodes_patch_packets() {
        assert_eq!(
            patch_check_request(1, 2, 3).unwrap(),
            vec![0xC1, 0x06, 0x02, 0x01, 0x02, 0x03]
        );
        assert_eq!(patch_version_okay().unwrap(), vec![0xC1, 0x04, 0x02, 0x00]);

        let packet = client_needs_patch(7, b"/patch");
        let packet = packet.unwrap();
        assert_eq!(packet.len(), 138);
        assert_eq!(&packet[..6], &[0xC1, 0x8A, 0x05, 0x01, 0x07, 0x00]);
    }

    #[test]
    fn encodes_server_list_responses() {
        assert_eq!(
            encode_server_list_response(&[ServerEntry::new(7, 42)]).unwrap(),
            vec![0xC2, 0x00, 0x0A, 0xF4, 0x06, 0x00, 0x01, 0x07, 0x00, 0x2A]
        );
        assert_eq!(
            encode_server_list_response_old(&[LegacyServerEntry::new(7, 42)]).unwrap(),
            vec![0xC2, 0x00, 0x08, 0xF4, 0x02, 0x01, 0x07, 0x2A]
        );
    }

    #[test]
    fn detects_server_list_requests() {
        assert!(is_server_list_request(&[0xC1, 0x04, 0xF4, 0x06]));
        assert!(is_server_list_request_old(&[0xC1, 0x04, 0xF4, 0x02]));
    }
}
