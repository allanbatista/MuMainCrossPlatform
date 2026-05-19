use crate::error::PacketCodecError as EncodeError;
use crate::wire::{encode_short_packet_with_subcode, fixed_bytes};

pub fn player_shop_set_item_price(item_slot: u8, price: u32) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(5);
    payload.push(item_slot);
    payload.extend_from_slice(&price.to_le_bytes());
    encode_short_packet_with_subcode(0xC3, 0x3F, 0x01, &payload)
}

pub fn player_shop_open(store_name: impl AsRef<[u8]>) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(26);
    payload.extend_from_slice(&fixed_bytes::<26>(store_name));
    encode_short_packet_with_subcode(0xC3, 0x3F, 0x02, &payload)
}

pub fn player_shop_close() -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC3, 0x3F, 0x03, &[])
}

pub fn player_shop_item_list_request(
    player_id: u16,
    player_name: impl AsRef<[u8]>,
) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(12);
    payload.extend_from_slice(&player_id.to_be_bytes());
    payload.extend_from_slice(&fixed_bytes::<10>(player_name));
    encode_short_packet_with_subcode(0xC3, 0x3F, 0x05, &payload)
}

pub fn player_shop_item_buy_request(
    player_id: u16,
    player_name: impl AsRef<[u8]>,
    item_slot: u8,
) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(13);
    payload.extend_from_slice(&player_id.to_be_bytes());
    payload.extend_from_slice(&fixed_bytes::<10>(player_name));
    payload.push(item_slot);
    encode_short_packet_with_subcode(0xC3, 0x3F, 0x06, &payload)
}

pub fn player_shop_close_other(
    player_id: u16,
    player_name: impl AsRef<[u8]>,
) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(12);
    payload.extend_from_slice(&player_id.to_be_bytes());
    payload.extend_from_slice(&fixed_bytes::<10>(player_name));
    encode_short_packet_with_subcode(0xC3, 0x3F, 0x07, &payload)
}

#[cfg(test)]
mod tests {
    use super::{
        player_shop_close, player_shop_close_other, player_shop_item_buy_request,
        player_shop_item_list_request, player_shop_open, player_shop_set_item_price,
    };
    use crate::wire::fixed_bytes;
    use proptest::prelude::*;

    #[test]
    fn encodes_player_shop_packets() {
        assert_eq!(
            player_shop_set_item_price(4, 0x01020304).unwrap(),
            vec![0xC3, 0x09, 0x3F, 0x01, 0x04, 0x04, 0x03, 0x02, 0x01]
        );

        let packet = player_shop_open(b"Shop").unwrap();
        assert_eq!(packet.len(), 30);
        assert_eq!(&packet[..4], &[0xC3, 0x1E, 0x3F, 0x02]);
        assert_eq!(&packet[4..30], &fixed_bytes::<26>(b"Shop"));

        assert_eq!(player_shop_close().unwrap(), vec![0xC3, 0x04, 0x3F, 0x03]);
        assert_eq!(
            player_shop_item_list_request(0x1234, b"Alice").unwrap(),
            vec![0xC3, 0x10, 0x3F, 0x05, 0x12, 0x34, b'A', b'l', b'i', b'c', b'e', 0, 0, 0, 0, 0]
        );
        assert_eq!(
            player_shop_item_buy_request(0x1234, b"Alice", 7).unwrap(),
            vec![
                0xC3, 0x11, 0x3F, 0x06, 0x12, 0x34, b'A', b'l', b'i', b'c', b'e', 0, 0, 0, 0, 0,
                0x07
            ]
        );
        assert_eq!(
            player_shop_close_other(0x1234, b"Alice").unwrap(),
            vec![0xC3, 0x10, 0x3F, 0x07, 0x12, 0x34, b'A', b'l', b'i', b'c', b'e', 0, 0, 0, 0, 0]
        );
    }

    proptest! {
        #[test]
        fn player_shop_open_pads_and_truncates(store_name in prop::collection::vec(any::<u8>(), 0..64)) {
            let packet = player_shop_open(&store_name).unwrap();
            assert_eq!(packet.len(), 30);
            assert_eq!(&packet[4..30], &fixed_bytes::<26>(&store_name));
        }
    }
}
