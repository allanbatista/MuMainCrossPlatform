use crate::error::PacketCodecError as EncodeError;
use crate::wire::encode_short_packet;

pub type SkillIndex = u8;

const AREA_SKILL_RESERVED_BYTES: [u8; 2] = [0x00, 0x00];

pub fn targeted_skill(skill_id: u16, target_id: u16) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(4);
    payload.extend_from_slice(&skill_id.to_be_bytes());
    payload.extend_from_slice(&target_id.to_be_bytes());
    encode_short_packet(0xC3, 0x19, &payload)
}

pub fn targeted_skill_075(skill_index: SkillIndex, target_id: u16) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(3);
    payload.push(skill_index);
    payload.extend_from_slice(&target_id.to_be_bytes());
    encode_short_packet(0xC1, 0x19, &payload)
}

pub fn targeted_skill_095(skill_index: SkillIndex, target_id: u16) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(3);
    payload.push(skill_index);
    payload.extend_from_slice(&target_id.to_be_bytes());
    encode_short_packet(0xC3, 0x19, &payload)
}

pub fn area_skill(
    skill_id: u16,
    target_x: u8,
    target_y: u8,
    rotation: u8,
    extra_target_id: u16,
    animation_counter: u8,
) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(10);
    payload.extend_from_slice(&skill_id.to_be_bytes());
    payload.push(target_x);
    payload.push(target_y);
    payload.push(rotation);
    payload.extend_from_slice(&AREA_SKILL_RESERVED_BYTES);
    payload.extend_from_slice(&extra_target_id.to_be_bytes());
    payload.push(animation_counter);
    encode_short_packet(0xC3, 0x1E, &payload)
}

pub fn area_skill_075(
    skill_index: SkillIndex,
    target_x: u8,
    target_y: u8,
    rotation: u8,
) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet(0xC1, 0x1E, &[skill_index, target_x, target_y, rotation])
}

pub fn area_skill_095(
    skill_index: SkillIndex,
    target_x: u8,
    target_y: u8,
    rotation: u8,
) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet(0xC3, 0x1E, &[skill_index, target_x, target_y, rotation])
}

#[cfg(test)]
mod tests {
    use super::{
        area_skill, area_skill_075, area_skill_095, targeted_skill, targeted_skill_075,
        targeted_skill_095,
    };

    #[test]
    fn encodes_targeted_skill_packets() {
        assert_eq!(
            targeted_skill(0x1234, 0x5678).unwrap(),
            vec![0xC3, 0x07, 0x19, 0x12, 0x34, 0x56, 0x78]
        );
        assert_eq!(
            targeted_skill_075(3, 0x5678).unwrap(),
            vec![0xC1, 0x06, 0x19, 3, 0x56, 0x78]
        );
        assert_eq!(
            targeted_skill_095(3, 0x5678).unwrap(),
            vec![0xC3, 0x06, 0x19, 3, 0x56, 0x78]
        );
    }

    #[test]
    fn encodes_area_skill_packets() {
        assert_eq!(
            area_skill(0x1234, 5, 6, 7, 0x9ABC, 8).unwrap(),
            vec![0xC3, 0x0D, 0x1E, 0x12, 0x34, 5, 6, 7, 0x00, 0x00, 0x9A, 0xBC, 8]
        );
        assert_eq!(
            area_skill_075(3, 4, 5, 6).unwrap(),
            vec![0xC1, 0x07, 0x1E, 3, 4, 5, 6]
        );
        assert_eq!(
            area_skill_095(3, 4, 5, 6).unwrap(),
            vec![0xC3, 0x07, 0x1E, 3, 4, 5, 6]
        );
    }
}
