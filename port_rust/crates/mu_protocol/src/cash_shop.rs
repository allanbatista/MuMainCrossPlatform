use crate::error::PacketCodecError as EncodeError;
use crate::wire::{encode_short_packet_with_subcode, fixed_bytes};

pub fn cash_shop_point_info_request() -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, 0xD2, 0x01, &[])
}

pub fn cash_shop_open_state(is_closed: bool) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, 0xD2, 0x02, &[u8::from(is_closed)])
}

pub fn cash_shop_item_buy_request(
    package_main_index: u32,
    category: u32,
    product_main_index: u32,
    item_index: u16,
    coin_index: u32,
    mileage_flag: u8,
) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(19);
    payload.extend_from_slice(&package_main_index.to_le_bytes());
    payload.extend_from_slice(&category.to_le_bytes());
    payload.extend_from_slice(&product_main_index.to_le_bytes());
    payload.extend_from_slice(&item_index.to_le_bytes());
    payload.extend_from_slice(&coin_index.to_le_bytes());
    payload.push(mileage_flag);
    encode_short_packet_with_subcode(0xC1, 0xD2, 0x03, &payload)
}

#[allow(clippy::too_many_arguments)]
pub fn cash_shop_item_gift_request(
    package_main_index: u32,
    category: u32,
    product_main_index: u32,
    item_index: u16,
    coin_index: u32,
    mileage_flag: u8,
    gift_receiver_name: impl AsRef<[u8]>,
    gift_text: impl AsRef<[u8]>,
) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(230);
    payload.extend_from_slice(&package_main_index.to_le_bytes());
    payload.extend_from_slice(&category.to_le_bytes());
    payload.extend_from_slice(&product_main_index.to_le_bytes());
    payload.extend_from_slice(&item_index.to_le_bytes());
    payload.extend_from_slice(&coin_index.to_le_bytes());
    payload.push(mileage_flag);
    payload.extend_from_slice(&fixed_bytes::<11>(gift_receiver_name));
    payload.extend_from_slice(&fixed_bytes::<200>(gift_text));
    encode_short_packet_with_subcode(0xC1, 0xD2, 0x04, &payload)
}

pub fn cash_shop_storage_list_request(
    page_index: u32,
    inventory_type: u8,
) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(6);
    payload.extend_from_slice(&page_index.to_le_bytes());
    payload.push(inventory_type);
    payload.push(0x00);
    encode_short_packet_with_subcode(0xC1, 0xD2, 0x05, &payload)
}

pub fn cash_shop_delete_storage_item_request(
    base_item_code: u32,
    main_item_code: u32,
    product_type: u8,
) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(230);
    payload.extend_from_slice(&base_item_code.to_le_bytes());
    payload.extend_from_slice(&main_item_code.to_le_bytes());
    payload.push(product_type);
    payload.resize(230, 0x00);
    encode_short_packet_with_subcode(0xC1, 0xD2, 0x0A, &payload)
}

pub fn cash_shop_storage_item_consume_request(
    base_item_code: u32,
    main_item_code: u32,
    item_index: u16,
    product_type: u8,
) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(11);
    payload.extend_from_slice(&base_item_code.to_le_bytes());
    payload.extend_from_slice(&main_item_code.to_le_bytes());
    payload.extend_from_slice(&item_index.to_le_bytes());
    payload.push(product_type);
    encode_short_packet_with_subcode(0xC1, 0xD2, 0x0B, &payload)
}

pub fn cash_shop_event_item_list_request(category_index: u32) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, 0xD2, 0x13, &category_index.to_le_bytes())
}

#[cfg(test)]
mod tests {
    use super::{
        cash_shop_delete_storage_item_request, cash_shop_event_item_list_request,
        cash_shop_item_buy_request, cash_shop_item_gift_request, cash_shop_open_state,
        cash_shop_point_info_request, cash_shop_storage_item_consume_request,
        cash_shop_storage_list_request,
    };
    use crate::wire::fixed_bytes;

    #[test]
    fn encodes_cash_shop_packets() {
        assert_eq!(
            cash_shop_point_info_request().unwrap(),
            vec![0xC1, 0x04, 0xD2, 0x01]
        );
        assert_eq!(
            cash_shop_open_state(true).unwrap(),
            vec![0xC1, 0x05, 0xD2, 0x02, 0x01]
        );
        assert_eq!(
            cash_shop_item_buy_request(1, 2, 3, 4, 5, 6).unwrap(),
            vec![
                0xC1, 0x17, 0xD2, 0x03, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x03, 0x00,
                0x00, 0x00, 0x04, 0x00, 0x05, 0x00, 0x00, 0x00, 0x06
            ]
        );
        let gift_packet = cash_shop_item_gift_request(1, 2, 3, 4, 5, 6, b"Bob", b"hi").unwrap();
        assert_eq!(gift_packet.len(), 234);
        assert_eq!(&gift_packet[..4], &[0xC1, 0xEA, 0xD2, 0x04]);
        assert_eq!(
            &gift_packet[4..23],
            &[
                0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x04, 0x00,
                0x05, 0x00, 0x00, 0x00, 0x06
            ]
        );
        assert_eq!(&gift_packet[23..34], &fixed_bytes::<11>(b"Bob"));
        assert_eq!(&gift_packet[34..234], &fixed_bytes::<200>(b"hi"));
        assert_eq!(
            cash_shop_storage_list_request(0x01020304, 7).unwrap(),
            vec![0xC1, 0x0A, 0xD2, 0x05, 0x04, 0x03, 0x02, 0x01, 0x07, 0x00]
        );
        let delete_packet = cash_shop_delete_storage_item_request(1, 2, 3).unwrap();
        assert_eq!(delete_packet.len(), 234);
        assert_eq!(
            &delete_packet[..13],
            &[0xC1, 0xEA, 0xD2, 0x0A, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x03]
        );
        assert!(delete_packet[13..].iter().all(|&byte| byte == 0));
        assert_eq!(
            cash_shop_storage_item_consume_request(1, 2, 3, 4).unwrap(),
            vec![
                0xC1, 0x0F, 0xD2, 0x0B, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x03, 0x00,
                0x04
            ]
        );
        assert_eq!(
            cash_shop_event_item_list_request(0x01020304).unwrap(),
            vec![0xC1, 0x08, 0xD2, 0x13, 0x04, 0x03, 0x02, 0x01]
        );
    }
}
