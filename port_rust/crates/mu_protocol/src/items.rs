use crate::error::PacketCodecError as EncodeError;
use crate::wire::{encode_short_packet, encode_short_packet_with_subcode, fixed_bytes};

pub type ChaosMachineMixType = u8;
pub type FruitUsage = u8;
pub type ItemStorageKind = u8;

pub fn pickup_item_request(item_id: u16) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet(0xC3, 0x22, &item_id.to_be_bytes())
}

pub fn pickup_item_request_075(item_id: u16) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet(0xC1, 0x22, &item_id.to_be_bytes())
}

pub fn drop_item_request(
    target_x: u8,
    target_y: u8,
    item_slot: u8,
) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet(0xC3, 0x23, &[target_x, target_y, item_slot])
}

pub fn item_move_request(
    from_storage: ItemStorageKind,
    from_slot: u8,
    item_data: impl AsRef<[u8]>,
    to_storage: ItemStorageKind,
    to_slot: u8,
) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(16);
    payload.push(from_storage);
    payload.push(from_slot);
    payload.extend_from_slice(&fixed_bytes::<12>(item_data));
    payload.push(to_storage);
    payload.push(to_slot);
    encode_short_packet(0xC3, 0x24, &payload)
}

pub fn item_move_request_extended(
    from_storage: ItemStorageKind,
    from_slot: u8,
    to_storage: ItemStorageKind,
    to_slot: u8,
) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet(0xC3, 0x24, &[from_storage, from_slot, to_storage, to_slot])
}

pub fn consume_item_request(
    item_slot: u8,
    target_slot: u8,
    fruit_consumption: FruitUsage,
) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet(0xC3, 0x26, &[item_slot, target_slot, fruit_consumption])
}

pub fn consume_item_request_075(item_slot: u8, target_slot: u8) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet(0xC1, 0x26, &[item_slot, target_slot])
}

pub fn unlock_vault(pin: u16) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(3);
    payload.extend_from_slice(&pin.to_le_bytes());
    payload.push(0x00);
    encode_short_packet_with_subcode(0xC1, 0x83, 0x00, &payload)
}

pub fn set_vault_pin(pin: u16, password: impl AsRef<[u8]>) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(23);
    payload.extend_from_slice(&pin.to_le_bytes());
    payload.extend_from_slice(&fixed_bytes::<20>(password));
    payload.push(0x00);
    encode_short_packet_with_subcode(0xC1, 0x83, 0x01, &payload)
}

pub fn remove_vault_pin(password: impl AsRef<[u8]>) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(23);
    payload.extend_from_slice(&[0x00, 0x00]);
    payload.extend_from_slice(&fixed_bytes::<20>(password));
    payload.push(0x00);
    encode_short_packet_with_subcode(0xC1, 0x83, 0x02, &payload)
}

pub fn vault_closed() -> Result<Vec<u8>, EncodeError> {
    encode_short_packet(0xC1, 0x82, &[])
}

pub fn vault_move_money_request(
    direction: crate::VaultMoneyMoveDirection,
    amount: u32,
) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(5);
    payload.push(direction);
    payload.extend_from_slice(&amount.to_le_bytes());
    encode_short_packet(0xC1, 0x81, &payload)
}

pub fn item_repair(inventory_item_slot: u8) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet(0xC3, 0x34, &[inventory_item_slot])
}

pub fn talk_to_npc_request(npc_id: u16) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet(0xC3, 0x30, &npc_id.to_be_bytes())
}

pub fn close_npc_request() -> Result<Vec<u8>, EncodeError> {
    encode_short_packet(0xC1, 0x31, &[])
}

pub fn buy_item_from_npc_request(item_slot: u8) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet(0xC3, 0x32, &[item_slot])
}

pub fn sell_item_to_npc_request(item_slot: u8) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet(0xC3, 0x33, &[item_slot])
}

pub fn repair_item_request(
    inventory_item_slot: u8,
    is_self_repair: bool,
) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet(0xC1, 0x34, &[inventory_item_slot, u8::from(is_self_repair)])
}

pub fn chaos_machine_mix_request(
    mix_type: ChaosMachineMixType,
    socket_slot: u8,
) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet(0xC1, 0x86, &[mix_type, socket_slot])
}

pub fn crafting_dialog_close_request() -> Result<Vec<u8>, EncodeError> {
    encode_short_packet(0xC1, 0x87, &[])
}

#[cfg(test)]
mod tests {
    use super::{
        buy_item_from_npc_request, chaos_machine_mix_request, close_npc_request,
        consume_item_request, consume_item_request_075, crafting_dialog_close_request,
        drop_item_request, fixed_bytes, item_move_request, item_move_request_extended, item_repair,
        pickup_item_request, pickup_item_request_075, repair_item_request,
        sell_item_to_npc_request, talk_to_npc_request,
    };
    use proptest::prelude::*;

    #[test]
    fn encodes_item_packets() {
        assert_eq!(
            pickup_item_request(0x1234).unwrap(),
            vec![0xC3, 0x05, 0x22, 0x12, 0x34]
        );
        assert_eq!(
            pickup_item_request_075(0x1234).unwrap(),
            vec![0xC1, 0x05, 0x22, 0x12, 0x34]
        );
        assert_eq!(
            drop_item_request(1, 2, 3).unwrap(),
            vec![0xC3, 0x06, 0x23, 0x01, 0x02, 0x03]
        );
        assert_eq!(
            item_move_request(0, 1, b"abcdefghijkl", 2, 3).unwrap(),
            vec![
                0xC3, 0x13, 0x24, 0x00, 0x01, b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i',
                b'j', b'k', b'l', 0x02, 0x03
            ]
        );
        assert_eq!(
            item_move_request_extended(0, 1, 2, 3).unwrap(),
            vec![0xC3, 0x07, 0x24, 0x00, 0x01, 0x02, 0x03]
        );
        assert_eq!(
            consume_item_request(1, 2, 0).unwrap(),
            vec![0xC3, 0x06, 0x26, 0x01, 0x02, 0x00]
        );
        assert_eq!(
            consume_item_request_075(1, 2).unwrap(),
            vec![0xC1, 0x05, 0x26, 0x01, 0x02]
        );
        assert_eq!(item_repair(0xFF).unwrap(), vec![0xC3, 0x04, 0x34, 0xFF]);
        assert_eq!(
            talk_to_npc_request(0x1234).unwrap(),
            vec![0xC3, 0x05, 0x30, 0x12, 0x34]
        );
        assert_eq!(close_npc_request().unwrap(), vec![0xC1, 0x03, 0x31]);
        assert_eq!(
            buy_item_from_npc_request(7).unwrap(),
            vec![0xC3, 0x04, 0x32, 0x07]
        );
        assert_eq!(
            sell_item_to_npc_request(8).unwrap(),
            vec![0xC3, 0x04, 0x33, 0x08]
        );
        assert_eq!(
            repair_item_request(0xFF, true).unwrap(),
            vec![0xC1, 0x05, 0x34, 0xFF, 0x01]
        );
        assert_eq!(
            chaos_machine_mix_request(1, 2).unwrap(),
            vec![0xC1, 0x05, 0x86, 0x01, 0x02]
        );
        assert_eq!(
            crafting_dialog_close_request().unwrap(),
            vec![0xC1, 0x03, 0x87]
        );
    }

    proptest! {
        #[test]
        fn item_move_request_pads_item_data(item_data in prop::collection::vec(any::<u8>(), 0..64)) {
            let packet = item_move_request(0, 1, &item_data, 2, 3).unwrap();
            assert_eq!(packet.len(), 19);
            assert_eq!(&packet[5..17], &fixed_bytes::<12>(&item_data));
        }
    }
}
