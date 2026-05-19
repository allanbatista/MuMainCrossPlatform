use crate::error::PacketCodecError as EncodeError;
use crate::wire::{encode_short_packet, encode_short_packet_with_subcode};

pub type PetType = u8;
pub type PetCommandMode = u8;
pub type StorageType = u8;

pub fn pet_command_request(
    pet_type: PetType,
    command_mode: PetCommandMode,
    target_id: u16,
) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(4);
    payload.push(pet_type);
    payload.push(command_mode);
    payload.extend_from_slice(&target_id.to_be_bytes());
    encode_short_packet(0xC1, 0xA7, &payload)
}

pub fn pet_info_request(
    pet: PetType,
    storage: StorageType,
    item_slot: u8,
) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet(0xC1, 0xA9, &[pet, storage, item_slot])
}

pub fn illusion_temple_skill_request(
    skill_number: u16,
    target_object_index: u8,
    distance: u8,
) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(4);
    payload.extend_from_slice(&skill_number.to_be_bytes());
    payload.push(target_object_index);
    payload.push(distance);
    encode_short_packet_with_subcode(0xC1, 0xBF, 0x02, &payload)
}

#[cfg(test)]
mod tests {
    use super::{illusion_temple_skill_request, pet_command_request, pet_info_request};

    #[test]
    fn encodes_pet_packets() {
        assert_eq!(
            pet_command_request(0, 3, 0x1234).unwrap(),
            vec![0xC1, 0x07, 0xA7, 0x00, 0x03, 0x12, 0x34]
        );
        assert_eq!(
            pet_info_request(1, 6, 0xFE).unwrap(),
            vec![0xC1, 0x06, 0xA9, 0x01, 0x06, 0xFE]
        );
        assert_eq!(
            illusion_temple_skill_request(0x1234, 5, 6).unwrap(),
            vec![0xC1, 0x08, 0xBF, 0x02, 0x12, 0x34, 5, 6]
        );
    }
}
