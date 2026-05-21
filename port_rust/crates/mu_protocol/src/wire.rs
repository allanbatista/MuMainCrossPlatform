use crate::error::PacketCodecError;
use std::convert::TryFrom;

const XOR3_KEY: [u8; 3] = [0xFC, 0xCF, 0xAB];

pub(crate) fn fixed_bytes<const N: usize>(value: impl AsRef<[u8]>) -> [u8; N] {
    let value = value.as_ref();
    let mut bytes = [0u8; N];
    let len = value.len().min(N);
    bytes[..len].copy_from_slice(&value[..len]);
    bytes
}

pub(crate) fn xor3_encrypt(bytes: &mut [u8], start_offset: usize) {
    for (index, byte) in bytes.iter_mut().enumerate() {
        *byte ^= XOR3_KEY[(start_offset + index) % XOR3_KEY.len()];
    }
}

pub(crate) fn encode_short_packet(
    code: u8,
    packet_type: u8,
    payload: &[u8],
) -> Result<Vec<u8>, PacketCodecError> {
    let size = 3usize
        .checked_add(payload.len())
        .ok_or(PacketCodecError::PacketTooLarge { size: usize::MAX })?;
    let size = u8::try_from(size).map_err(|_| PacketCodecError::PacketTooLarge { size })?;

    let mut packet = Vec::with_capacity(usize::from(size));
    packet.push(code);
    packet.push(size);
    packet.push(packet_type);
    packet.extend_from_slice(payload);
    Ok(packet)
}

pub(crate) fn encode_short_packet_with_subcode(
    code: u8,
    packet_type: u8,
    subcode: u8,
    payload: &[u8],
) -> Result<Vec<u8>, PacketCodecError> {
    let size = 4usize
        .checked_add(payload.len())
        .ok_or(PacketCodecError::PacketTooLarge { size: usize::MAX })?;
    let size = u8::try_from(size).map_err(|_| PacketCodecError::PacketTooLarge { size })?;

    let mut packet = Vec::with_capacity(usize::from(size));
    packet.push(code);
    packet.push(size);
    packet.push(packet_type);
    packet.push(subcode);
    packet.extend_from_slice(payload);
    Ok(packet)
}

pub(crate) fn encode_long_packet_with_subcode(
    code: u8,
    packet_type: u8,
    subcode: u8,
    payload: &[u8],
) -> Result<Vec<u8>, PacketCodecError> {
    let size = 5usize
        .checked_add(payload.len())
        .ok_or(PacketCodecError::PacketTooLarge { size: usize::MAX })?;
    let size = u16::try_from(size).map_err(|_| PacketCodecError::PacketTooLarge { size })?;

    let mut packet = Vec::with_capacity(usize::from(size));
    packet.push(code);
    packet.extend_from_slice(&size.to_be_bytes());
    packet.push(packet_type);
    packet.push(subcode);
    packet.extend_from_slice(payload);
    Ok(packet)
}

pub(crate) fn encode_long_packet(
    code: u8,
    packet_type: u8,
    payload: &[u8],
) -> Result<Vec<u8>, PacketCodecError> {
    let size = 4usize
        .checked_add(payload.len())
        .ok_or(PacketCodecError::PacketTooLarge { size: usize::MAX })?;
    let size = u16::try_from(size).map_err(|_| PacketCodecError::PacketTooLarge { size })?;

    let mut packet = Vec::with_capacity(usize::from(size));
    packet.push(code);
    packet.extend_from_slice(&size.to_be_bytes());
    packet.push(packet_type);
    packet.extend_from_slice(payload);
    Ok(packet)
}

#[cfg(test)]
mod tests {
    use super::{encode_long_packet, fixed_bytes, xor3_encrypt};

    #[test]
    fn pads_and_truncates_fixed_bytes() {
        assert_eq!(fixed_bytes::<4>(b"ab"), [b'a', b'b', 0, 0]);
        assert_eq!(fixed_bytes::<3>(b"abcdef"), [b'a', b'b', b'c']);
    }

    #[test]
    fn encrypts_with_repeating_three_byte_key() {
        let mut bytes = [0u8; 6];
        xor3_encrypt(&mut bytes, 0);
        assert_eq!(bytes, [0xFC, 0xCF, 0xAB, 0xFC, 0xCF, 0xAB]);

        let mut shifted = [0u8; 3];
        xor3_encrypt(&mut shifted, 5);
        assert_eq!(shifted, [0xAB, 0xFC, 0xCF]);
    }

    #[test]
    fn encodes_long_packets_without_subcode() {
        assert_eq!(
            encode_long_packet(0xC4, 0xC5, &[0x01, 0x02]).unwrap(),
            vec![0xC4, 0x00, 0x06, 0xC5, 0x01, 0x02]
        );
    }
}
