use crate::error::PacketCodecError as EncodeError;
use crate::wire::encode_short_packet;

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

fn pack_walk_step_and_rotation(step_count: u8, target_rotation: u8) -> u8 {
    (step_count & 0x0F) | ((target_rotation & 0x0F) << 4)
}

#[cfg(test)]
mod tests {
    use super::{animation_request, instant_move_request, walk_request, walk_request_075};

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
}
