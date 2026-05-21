use crate::error::PacketCodecError as EncodeError;
use crate::frame::PacketFrame;
use crate::wire::encode_short_packet;

const MOVE_CHARACTER_HEADCODE: u8 = 0xD4;
const MOVE_POSITION_PAYLOAD_LEN: usize = 3;
const MOVE_CHARACTER_MIN_PAYLOAD_LEN: usize = 6;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MoveCharacterUpdate {
    pub key: u16,
    pub source_x: u8,
    pub source_y: u8,
    pub target_x: u8,
    pub target_y: u8,
    pub path_metadata: u8,
    pub directions: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MovePositionUpdate {
    pub key: u16,
    pub position_x: u8,
    pub position_y: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MovementUpdate {
    Character(MoveCharacterUpdate),
    Position(MovePositionUpdate),
}

pub fn walk_request(
    source_x: u8,
    source_y: u8,
    step_count: u8,
    target_rotation: u8,
    directions: impl AsRef<[u8]>,
) -> Result<Vec<u8>, EncodeError> {
    let directions = directions.as_ref();
    let mut payload = Vec::with_capacity(3 + directions.len());
    payload.push(source_x);
    payload.push(source_y);
    payload.push(pack_walk_step_and_rotation(step_count, target_rotation));
    payload.extend_from_slice(directions);
    encode_short_packet(0xC1, 0xD4, &payload)
}

pub fn walk_request_075(
    source_x: u8,
    source_y: u8,
    step_count: u8,
    target_rotation: u8,
    directions: impl AsRef<[u8]>,
) -> Result<Vec<u8>, EncodeError> {
    let directions = directions.as_ref();
    let mut payload = Vec::with_capacity(3 + directions.len());
    payload.push(source_x);
    payload.push(source_y);
    payload.push(pack_walk_step_and_rotation(step_count, target_rotation));
    payload.extend_from_slice(directions);
    encode_short_packet(0xC1, 0x10, &payload)
}

pub fn instant_move_request(target_x: u8, target_y: u8) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet(0xC1, 0x15, &[target_x, target_y])
}

pub fn animation_request(rotation: u8, animation_number: u8) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet(0xC1, 0x18, &[rotation, animation_number])
}

pub fn encode_move_character_update(
    key: u16,
    source_x: u8,
    source_y: u8,
    target_x: u8,
    target_y: u8,
    path_metadata: u8,
    directions: impl AsRef<[u8]>,
) -> Result<Vec<u8>, EncodeError> {
    let directions = directions.as_ref();
    let mut payload = Vec::with_capacity(7 + directions.len());
    payload.extend_from_slice(&key.to_be_bytes());
    payload.push(source_x);
    payload.push(source_y);
    payload.push(target_x);
    payload.push(target_y);
    payload.push(path_metadata);
    payload.extend_from_slice(directions);
    encode_short_packet(0xC1, MOVE_CHARACTER_HEADCODE, &payload)
}

pub fn encode_move_position_update(
    key: u16,
    position_x: u8,
    position_y: u8,
) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet(
        0xC1,
        MOVE_CHARACTER_HEADCODE,
        &[
            key.to_be_bytes()[0],
            key.to_be_bytes()[1],
            position_x,
            position_y,
        ],
    )
}

pub fn decode_movement_update(frame: &PacketFrame<'_>) -> Option<MovementUpdate> {
    if frame.headcode != MOVE_CHARACTER_HEADCODE {
        return None;
    }

    if frame.payload.len() == MOVE_POSITION_PAYLOAD_LEN {
        decode_move_position_update(frame).map(MovementUpdate::Position)
    } else if frame.payload.len() >= MOVE_CHARACTER_MIN_PAYLOAD_LEN {
        decode_move_character_update(frame).map(MovementUpdate::Character)
    } else {
        None
    }
}

pub fn decode_move_character_update(frame: &PacketFrame<'_>) -> Option<MoveCharacterUpdate> {
    if frame.headcode != MOVE_CHARACTER_HEADCODE
        || frame.payload.len() < MOVE_CHARACTER_MIN_PAYLOAD_LEN
    {
        return None;
    }

    let key = u16::from_be_bytes([frame.subcode, *frame.payload.first()?]);
    let source_x = *frame.payload.get(1)?;
    let source_y = *frame.payload.get(2)?;
    let target_x = *frame.payload.get(3)?;
    let target_y = *frame.payload.get(4)?;
    let path_metadata = *frame.payload.get(5)?;
    let directions = frame.payload.get(6..).unwrap_or_default().to_vec();

    Some(MoveCharacterUpdate {
        key,
        source_x,
        source_y,
        target_x,
        target_y,
        path_metadata,
        directions,
    })
}

pub fn decode_move_position_update(frame: &PacketFrame<'_>) -> Option<MovePositionUpdate> {
    if frame.headcode != MOVE_CHARACTER_HEADCODE || frame.payload.len() != MOVE_POSITION_PAYLOAD_LEN
    {
        return None;
    }

    let key = u16::from_be_bytes([frame.subcode, *frame.payload.first()?]);
    let position_x = *frame.payload.get(1)?;
    let position_y = *frame.payload.get(2)?;

    Some(MovePositionUpdate {
        key,
        position_x,
        position_y,
    })
}

fn pack_walk_step_and_rotation(step_count: u8, target_rotation: u8) -> u8 {
    (step_count & 0x0F) | ((target_rotation & 0x0F) << 4)
}

#[cfg(test)]
mod tests {
    use super::{
        animation_request, decode_movement_update, encode_move_character_update,
        encode_move_position_update, instant_move_request, walk_request, walk_request_075,
        MovementUpdate,
    };
    use crate::decode_packet;

    #[test]
    fn encodes_movement_packets() {
        assert_eq!(
            walk_request(1, 2, 3, 4, [0x12, 0x34]).unwrap(),
            vec![0xC1, 0x08, 0xD4, 0x01, 0x02, 0x43, 0x12, 0x34]
        );
        assert_eq!(
            walk_request_075(1, 2, 3, 4, [0x12, 0x34]).unwrap(),
            vec![0xC1, 0x08, 0x10, 0x01, 0x02, 0x43, 0x12, 0x34]
        );
        assert_eq!(
            instant_move_request(5, 6).unwrap(),
            vec![0xC1, 0x05, 0x15, 0x05, 0x06]
        );
        assert_eq!(
            animation_request(7, 8).unwrap(),
            vec![0xC1, 0x05, 0x18, 0x07, 0x08]
        );
    }

    #[test]
    fn encodes_and_decodes_authoritative_movement_updates() {
        let move_position = encode_move_position_update(0x1234, 5, 6).unwrap();
        assert_eq!(
            move_position,
            vec![0xC1, 0x07, 0xD4, 0x12, 0x34, 0x05, 0x06]
        );

        let frame = decode_packet(&move_position).unwrap();
        assert_eq!(
            decode_movement_update(&frame),
            Some(MovementUpdate::Position(super::MovePositionUpdate {
                key: 0x1234,
                position_x: 5,
                position_y: 6,
            }))
        );

        let move_character =
            encode_move_character_update(0x1234, 1, 2, 3, 4, 0x43, [0x12, 0x34]).unwrap();
        assert_eq!(
            move_character,
            vec![0xC1, 0x0C, 0xD4, 0x12, 0x34, 0x01, 0x02, 0x03, 0x04, 0x43, 0x12, 0x34]
        );

        let frame = decode_packet(&move_character).unwrap();
        assert_eq!(
            decode_movement_update(&frame),
            Some(MovementUpdate::Character(super::MoveCharacterUpdate {
                key: 0x1234,
                source_x: 1,
                source_y: 2,
                target_x: 3,
                target_y: 4,
                path_metadata: 0x43,
                directions: vec![0x12, 0x34],
            }))
        );
    }
}
