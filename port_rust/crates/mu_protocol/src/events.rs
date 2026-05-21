use crate::error::PacketCodecError as EncodeError;
use crate::frame::PacketFrame;
use crate::wire::{encode_short_packet, encode_short_packet_with_subcode, fixed_bytes};

pub type GensType = u8;

pub const GENS_RANKING_HEADCODE: u8 = 0xF8;
pub const GENS_RANKING_SUBCODE: u8 = 0x07;
pub const GENS_RANKING_PAYLOAD_LEN: usize = 17;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GensRankingInfo {
    pub influence: u8,
    pub ranking: i32,
    pub gens_class: i32,
    pub contribution_point: i32,
    pub next_contribution_point: i32,
}

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

pub fn decode_gens_ranking_info(frame: &PacketFrame<'_>) -> Result<GensRankingInfo, String> {
    if frame.headcode != GENS_RANKING_HEADCODE || frame.subcode != GENS_RANKING_SUBCODE {
        return Err(format!(
            "unexpected gens ranking packet: headcode=0x{:02X}, subcode=0x{:02X}",
            frame.headcode, frame.subcode
        ));
    }

    let payload = frame.payload;
    if payload.len() < GENS_RANKING_PAYLOAD_LEN {
        return Err(format!(
            "gens ranking packet truncated: expected at least {GENS_RANKING_PAYLOAD_LEN} payload bytes, found {}",
            payload.len()
        ));
    }

    Ok(GensRankingInfo {
        influence: payload[0],
        ranking: i32::from_le_bytes(
            payload[1..5]
                .try_into()
                .map_err(|_| "gens ranking packet missing ranking field".to_string())?,
        ),
        gens_class: i32::from_le_bytes(
            payload[5..9]
                .try_into()
                .map_err(|_| "gens ranking packet missing gens class field".to_string())?,
        ),
        contribution_point: i32::from_le_bytes(
            payload[9..13]
                .try_into()
                .map_err(|_| "gens ranking packet missing contribution field".to_string())?,
        ),
        next_contribution_point: i32::from_le_bytes(
            payload[13..17]
                .try_into()
                .map_err(|_| "gens ranking packet missing next contribution field".to_string())?,
        ),
    })
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
        decode_gens_ranking_info, duel_channel_join_request, duel_channel_quit_request,
        duel_start_request, duel_start_response, duel_stop_request, gens_join_request,
        gens_leave_request, gens_ranking_request, gens_reward_request, kanturu_enter_request,
        kanturu_info_request, raklion_state_info_request, server_immigration_request,
        GensRankingInfo, GENS_RANKING_HEADCODE, GENS_RANKING_SUBCODE,
    };
    use crate::decode_packet;
    use crate::encode_packet;

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
    fn decodes_gens_ranking_packet() {
        let packet = encode_packet(
            0xC1,
            GENS_RANKING_HEADCODE,
            GENS_RANKING_SUBCODE,
            &[
                1, // Duprian
                0x09, 0x00, 0x00, 0x00, // ranking
                0x02, 0x00, 0x00, 0x00, // gens class
                0x64, 0x00, 0x00, 0x00, // contribution
                0xC8, 0x00, 0x00, 0x00, // next contribution
            ],
        )
        .unwrap();

        let frame = decode_packet(&packet).unwrap();
        assert_eq!(
            decode_gens_ranking_info(&frame).unwrap(),
            GensRankingInfo {
                influence: 1,
                ranking: 9,
                gens_class: 2,
                contribution_point: 100,
                next_contribution_point: 200,
            }
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
