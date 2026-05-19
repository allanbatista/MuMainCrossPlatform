use crate::error::PacketCodecError as EncodeError;
use crate::wire::{encode_short_packet, encode_short_packet_with_subcode, fixed_bytes};

pub type GensType = u8;

pub fn crywolf_info_request() -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, 0xBD, 0x00, &[])
}

pub fn crywolf_contract_request(statue_id: u16) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, 0xBD, 0x03, &statue_id.to_be_bytes())
}

pub fn crywolf_chaos_rate_benefit_request() -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, 0xBD, 0x09, &[])
}

pub fn kanturu_info_request() -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, 0xD1, 0x00, &[])
}

pub fn kanturu_enter_request() -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, 0xD1, 0x01, &[])
}

pub fn raklion_state_info_request() -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, 0xD1, 0x10, &[])
}

pub fn gens_join_request(gens_type: GensType) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, 0xF8, 0x01, &[gens_type])
}

pub fn gens_leave_request() -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, 0xF8, 0x03, &[])
}

pub fn gens_reward_request(gens_type: GensType) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, 0xF8, 0x09, &[gens_type])
}

pub fn gens_ranking_request() -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, 0xF8, 0x0B, &[])
}

pub fn server_immigration_request(security_code: impl AsRef<[u8]>) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet(0xC3, 0x99, security_code.as_ref())
}

pub fn duel_start_request(
    player_id: u16,
    player_name: impl AsRef<[u8]>,
) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(12);
    payload.extend_from_slice(&player_id.to_be_bytes());
    payload.extend_from_slice(&fixed_bytes::<10>(player_name));
    encode_short_packet_with_subcode(0xC3, 0xAA, 0x01, &payload)
}

pub fn duel_start_response(
    response: bool,
    player_id: u16,
    player_name: impl AsRef<[u8]>,
) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(13);
    payload.push(u8::from(response));
    payload.extend_from_slice(&player_id.to_le_bytes());
    payload.extend_from_slice(&fixed_bytes::<10>(player_name));
    encode_short_packet_with_subcode(0xC3, 0xAA, 0x02, &payload)
}

pub fn duel_stop_request() -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC3, 0xAA, 0x03, &[])
}

pub fn duel_channel_join_request(channel_id: u8) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC3, 0xAA, 0x07, &[channel_id])
}

pub fn duel_channel_quit_request() -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC3, 0xAA, 0x09, &[])
}

#[cfg(test)]
mod tests {
    use super::{
        crywolf_chaos_rate_benefit_request, crywolf_contract_request, crywolf_info_request,
        duel_channel_join_request, duel_channel_quit_request, duel_start_request,
        duel_start_response, duel_stop_request, gens_join_request, gens_leave_request,
        gens_ranking_request, gens_reward_request, kanturu_enter_request, kanturu_info_request,
        raklion_state_info_request, server_immigration_request,
    };

    #[test]
    fn encodes_region_packets() {
        assert_eq!(
            crywolf_info_request().unwrap(),
            vec![0xC1, 0x04, 0xBD, 0x00]
        );
        assert_eq!(
            crywolf_contract_request(0x1234).unwrap(),
            vec![0xC1, 0x06, 0xBD, 0x03, 0x12, 0x34]
        );
        assert_eq!(
            crywolf_chaos_rate_benefit_request().unwrap(),
            vec![0xC1, 0x04, 0xBD, 0x09]
        );
        assert_eq!(
            kanturu_info_request().unwrap(),
            vec![0xC1, 0x04, 0xD1, 0x00]
        );
        assert_eq!(
            kanturu_enter_request().unwrap(),
            vec![0xC1, 0x04, 0xD1, 0x01]
        );
        assert_eq!(
            raklion_state_info_request().unwrap(),
            vec![0xC1, 0x04, 0xD1, 0x10]
        );
        assert_eq!(
            gens_join_request(2).unwrap(),
            vec![0xC1, 0x05, 0xF8, 0x01, 0x02]
        );
        assert_eq!(gens_leave_request().unwrap(), vec![0xC1, 0x04, 0xF8, 0x03]);
        assert_eq!(
            gens_reward_request(3).unwrap(),
            vec![0xC1, 0x05, 0xF8, 0x09, 0x03]
        );
        assert_eq!(
            gens_ranking_request().unwrap(),
            vec![0xC1, 0x04, 0xF8, 0x0B]
        );
        assert_eq!(
            server_immigration_request(b"ABC").unwrap(),
            vec![0xC3, 0x06, 0x99, b'A', b'B', b'C']
        );
    }

    #[test]
    fn encodes_duel_packets() {
        assert_eq!(
            duel_start_request(0x1234, b"Alice").unwrap(),
            vec![0xC3, 0x10, 0xAA, 0x01, 0x12, 0x34, b'A', b'l', b'i', b'c', b'e', 0, 0, 0, 0, 0]
        );
        assert_eq!(
            duel_start_response(true, 0x1234, b"Alice").unwrap(),
            vec![
                0xC3, 0x11, 0xAA, 0x02, 0x01, 0x34, 0x12, b'A', b'l', b'i', b'c', b'e', 0, 0, 0, 0,
                0
            ]
        );
        assert_eq!(duel_stop_request().unwrap(), vec![0xC3, 0x04, 0xAA, 0x03]);
        assert_eq!(
            duel_channel_join_request(7).unwrap(),
            vec![0xC3, 0x05, 0xAA, 0x07, 0x07]
        );
        assert_eq!(
            duel_channel_quit_request().unwrap(),
            vec![0xC3, 0x04, 0xAA, 0x09]
        );
    }
}
