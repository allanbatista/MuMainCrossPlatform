use crate::error::PacketCodecError as EncodeError;
use crate::skills::SkillIndex;
use crate::wire::encode_short_packet;

const RAGE_ATTACK_REQUEST_PADDING: u8 = 0x00;

pub fn hit_request(
    target_id: u16,
    attack_animation: u8,
    looking_direction: u8,
) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(4);
    payload.extend_from_slice(&target_id.to_be_bytes());
    payload.push(attack_animation);
    payload.push(looking_direction);
    encode_short_packet(0xC1, 0x11, &payload)
}

pub fn magic_effect_cancel_request(skill_id: u16, player_id: u16) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(4);
    payload.extend_from_slice(&skill_id.to_be_bytes());
    payload.extend_from_slice(&player_id.to_be_bytes());
    encode_short_packet(0xC1, 0x1B, &payload)
}

pub fn rage_attack_request(skill_id: u16, target_id: u16) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(5);
    payload.extend_from_slice(&skill_id.to_be_bytes());
    payload.push(RAGE_ATTACK_REQUEST_PADDING);
    payload.extend_from_slice(&target_id.to_be_bytes());
    encode_short_packet(0xC3, 0x4A, &payload)
}

pub fn rage_attack_range_request(skill_id: u16, target_id: u16) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(4);
    payload.extend_from_slice(&skill_id.to_be_bytes());
    payload.extend_from_slice(&target_id.to_be_bytes());
    encode_short_packet(0xC1, 0x4B, &payload)
}

pub fn area_skill_hit_075(
    skill_index: SkillIndex,
    target_x: u8,
    target_y: u8,
    targets: impl AsRef<[u16]>,
) -> Result<Vec<u8>, EncodeError> {
    encode_area_skill_hit(0xC1, skill_index, target_x, target_y, None, targets)
}

pub fn area_skill_hit_095(
    skill_index: SkillIndex,
    target_x: u8,
    target_y: u8,
    counter: u8,
    targets: impl AsRef<[u16]>,
) -> Result<Vec<u8>, EncodeError> {
    encode_area_skill_hit(
        0xC3,
        skill_index,
        target_x,
        target_y,
        Some(counter),
        targets,
    )
}

fn encode_area_skill_hit(
    code: u8,
    skill_index: SkillIndex,
    target_x: u8,
    target_y: u8,
    counter: Option<u8>,
    targets: impl AsRef<[u16]>,
) -> Result<Vec<u8>, EncodeError> {
    let targets = targets.as_ref();
    let target_bytes = targets
        .len()
        .checked_mul(2)
        .ok_or(EncodeError::PacketTooLarge { size: usize::MAX })?;
    let payload_len = 4usize
        .checked_add(target_bytes)
        .and_then(|len| len.checked_add(usize::from(counter.is_some())))
        .ok_or(EncodeError::PacketTooLarge { size: usize::MAX })?;
    let packet_len = 3usize
        .checked_add(payload_len)
        .ok_or(EncodeError::PacketTooLarge { size: usize::MAX })?;
    if targets.len() > u8::MAX as usize {
        return Err(EncodeError::PacketTooLarge { size: packet_len });
    }

    let mut payload = Vec::with_capacity(payload_len);
    payload.push(skill_index);
    payload.push(target_x);
    payload.push(target_y);
    if let Some(counter) = counter {
        payload.push(counter);
    }
    payload.push(targets.len() as u8);
    for target_id in targets {
        payload.extend_from_slice(&target_id.to_be_bytes());
    }

    encode_short_packet(code, 0x1D, &payload)
}

#[cfg(test)]
mod tests {
    use super::{
        area_skill_hit_075, area_skill_hit_095, hit_request, magic_effect_cancel_request,
        rage_attack_range_request, rage_attack_request,
    };
    use proptest::prelude::*;

    #[test]
    fn encodes_combat_packets() {
        assert_eq!(
            hit_request(0x1234, 5, 6).unwrap(),
            vec![0xC1, 0x07, 0x11, 0x12, 0x34, 5, 6]
        );
        assert_eq!(
            magic_effect_cancel_request(0x1234, 0x5678).unwrap(),
            vec![0xC1, 0x07, 0x1B, 0x12, 0x34, 0x56, 0x78]
        );
        assert_eq!(
            rage_attack_request(0x1234, 0x5678).unwrap(),
            vec![0xC3, 0x08, 0x4A, 0x12, 0x34, 0x00, 0x56, 0x78]
        );
        assert_eq!(
            rage_attack_range_request(0x1234, 0x5678).unwrap(),
            vec![0xC1, 0x07, 0x4B, 0x12, 0x34, 0x56, 0x78]
        );
        assert_eq!(
            area_skill_hit_075(3, 4, 5, [0x1234, 0xABCD]).unwrap(),
            vec![0xC1, 0x0B, 0x1D, 3, 4, 5, 2, 0x12, 0x34, 0xAB, 0xCD]
        );
        assert_eq!(
            area_skill_hit_095(3, 4, 5, 6, [0x1234, 0xABCD]).unwrap(),
            vec![0xC3, 0x0C, 0x1D, 3, 4, 5, 6, 2, 0x12, 0x34, 0xAB, 0xCD]
        );
    }

    proptest! {
        #[test]
        fn encodes_area_skill_hit_target_lists(targets in prop::collection::vec(any::<u16>(), 0..64)) {
            let packet_075 = area_skill_hit_075(1, 2, 3, &targets).unwrap();
            assert_eq!(packet_075.len(), 7 + targets.len() * 2);
            assert_eq!(packet_075[6], targets.len() as u8);
            for (index, target) in targets.iter().enumerate() {
                let start = 7 + index * 2;
                assert_eq!(&packet_075[start..start + 2], &target.to_be_bytes());
            }

            let packet_095 = area_skill_hit_095(1, 2, 3, 4, &targets).unwrap();
            assert_eq!(packet_095.len(), 8 + targets.len() * 2);
            assert_eq!(packet_095[6], 4);
            assert_eq!(packet_095[7], targets.len() as u8);
            for (index, target) in targets.iter().enumerate() {
                let start = 8 + index * 2;
                assert_eq!(&packet_095[start..start + 2], &target.to_be_bytes());
            }
        }
    }
}
