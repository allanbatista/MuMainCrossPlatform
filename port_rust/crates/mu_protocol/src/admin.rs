use crate::error::PacketCodecError as EncodeError;
use crate::wire::{encode_short_packet, encode_short_packet_with_subcode, fixed_bytes};

pub type LahapItemType = u8;
pub type LahapMixType = u8;
pub type LahapStackSize = u8;

pub fn enter_market_place_request() -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, 0xBF, 0x17, &[])
}

pub fn lahap_jewel_mix_request(
    operation: LahapMixType,
    item: LahapItemType,
    mixing_stack_size: LahapStackSize,
    unmixing_source_slot: u8,
) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet(
        0xC1,
        0x32,
        &[operation, item, mixing_stack_size, unmixing_source_slot],
    )
}

pub fn lucky_number_request(
    serial1: impl AsRef<[u8]>,
    serial2: impl AsRef<[u8]>,
    serial3: impl AsRef<[u8]>,
) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(15);
    payload.extend_from_slice(&fixed_bytes::<4>(serial1));
    payload.push(0x00);
    payload.extend_from_slice(&fixed_bytes::<4>(serial2));
    payload.push(0x00);
    payload.extend_from_slice(&fixed_bytes::<4>(serial3));
    payload.push(0x00);
    encode_short_packet(0xC1, 0x9D, &payload)
}

#[cfg(test)]
mod tests {
    use super::{enter_market_place_request, lahap_jewel_mix_request, lucky_number_request};

    #[test]
    fn encodes_admin_packets() {
        assert_eq!(
            enter_market_place_request().unwrap(),
            vec![0xC1, 0x04, 0xBF, 0x17]
        );
        assert_eq!(
            lahap_jewel_mix_request(0, 1, 2, 3).unwrap(),
            vec![0xC1, 0x07, 0x32, 0x00, 0x01, 0x02, 0x03]
        );
        assert_eq!(
            lucky_number_request(b"1234", b"5678", b"9012").unwrap(),
            vec![
                0xC1, 0x12, 0x9D, b'1', b'2', b'3', b'4', 0x00, b'5', b'6', b'7', b'8', 0x00, b'9',
                b'0', b'1', b'2', 0x00
            ]
        );
    }
}
