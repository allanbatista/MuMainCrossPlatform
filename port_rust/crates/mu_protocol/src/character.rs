use crate::error::PacketCodecError as EncodeError;
pub use crate::login::{
    create_character, delete_character, request_character_list, select_character,
};
pub use crate::session::CharacterClassNumber;
use crate::wire::{encode_short_packet_with_subcode, fixed_bytes};

pub type CharacterStatAttribute = u8;

pub fn focus_character(name: impl AsRef<[u8]>) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(10);
    payload.extend_from_slice(&fixed_bytes::<10>(name));
    encode_short_packet_with_subcode(0xC1, 0xF3, 0x15, &payload)
}

pub fn increase_character_stat_point(
    stat_type: CharacterStatAttribute,
) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, 0xF3, 0x06, &[stat_type])
}

pub fn inventory_request() -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC3, 0xF3, 0x10, &[])
}

pub fn client_ready_after_map_change() -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, 0xF3, 0x12, &[])
}

pub fn reset_character_point_request() -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, 0xF2, 0x00, &[])
}

pub fn save_key_configuration(configuration: impl AsRef<[u8]>) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, 0xF3, 0x30, configuration.as_ref())
}

pub fn add_master_skill_point(skill_id: u16) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, 0xF3, 0x52, &skill_id.to_le_bytes())
}

#[cfg(test)]
mod tests {
    use super::{
        add_master_skill_point, client_ready_after_map_change, focus_character,
        increase_character_stat_point, inventory_request, reset_character_point_request,
        save_key_configuration,
    };

    #[test]
    fn encodes_character_packets() {
        assert_eq!(
            focus_character(b"Alice").unwrap(),
            vec![0xC1, 0x0E, 0xF3, 0x15, b'A', b'l', b'i', b'c', b'e', 0, 0, 0, 0, 0]
        );
        assert_eq!(
            increase_character_stat_point(4).unwrap(),
            vec![0xC1, 0x05, 0xF3, 0x06, 4]
        );
        assert_eq!(inventory_request().unwrap(), vec![0xC3, 0x04, 0xF3, 0x10]);
        assert_eq!(
            client_ready_after_map_change().unwrap(),
            vec![0xC1, 0x04, 0xF3, 0x12]
        );
        assert_eq!(
            reset_character_point_request().unwrap(),
            vec![0xC1, 0x04, 0xF2, 0x00]
        );
        assert_eq!(
            save_key_configuration(b"abc").unwrap(),
            vec![0xC1, 0x07, 0xF3, 0x30, b'a', b'b', b'c']
        );
        assert_eq!(
            add_master_skill_point(0x1234).unwrap(),
            vec![0xC1, 0x06, 0xF3, 0x52, 0x34, 0x12]
        );
    }
}
