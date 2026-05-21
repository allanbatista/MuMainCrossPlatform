use crate::error::PacketCodecError as EncodeError;
use crate::session::GuildMemberRole;
use crate::wire::{encode_short_packet, encode_short_packet_with_subcode, fixed_bytes};

pub type GuildRelationshipType = u8;
pub type GuildRequestType = u8;

pub fn guild_kick_player_request(
    player_name: impl AsRef<[u8]>,
    security_code: impl AsRef<[u8]>,
) -> Result<Vec<u8>, EncodeError> {
    let security_code = security_code.as_ref();
    let mut payload = Vec::with_capacity(10 + security_code.len());
    payload.extend_from_slice(&fixed_bytes::<10>(player_name));
    payload.extend_from_slice(security_code);
    encode_short_packet(0xC1, 0x53, &payload)
}

pub fn guild_join_request(guild_master_player_id: u16) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet(0xC1, 0x50, &guild_master_player_id.to_be_bytes())
}

pub fn guild_join_response(accepted: bool, requester_id: u16) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(3);
    payload.push(u8::from(accepted));
    payload.extend_from_slice(&requester_id.to_be_bytes());
    encode_short_packet(0xC1, 0x51, &payload)
}

pub fn guild_list_request() -> Result<Vec<u8>, EncodeError> {
    encode_short_packet(0xC1, 0x52, &[])
}

pub fn guild_create_request(
    guild_name: impl AsRef<[u8]>,
    guild_emblem: impl AsRef<[u8]>,
) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(41);
    payload.push(0x00);
    payload.extend_from_slice(&fixed_bytes::<8>(guild_name));
    payload.extend_from_slice(&fixed_bytes::<32>(guild_emblem));
    encode_short_packet(0xC1, 0x55, &payload)
}

pub fn guild_create_request_075(
    guild_name: impl AsRef<[u8]>,
    guild_emblem: impl AsRef<[u8]>,
) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(40);
    payload.extend_from_slice(&fixed_bytes::<8>(guild_name));
    payload.extend_from_slice(&fixed_bytes::<32>(guild_emblem));
    encode_short_packet(0xC1, 0x55, &payload)
}

pub fn guild_master_answer(show_creation_dialog: bool) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet(0xC1, 0x54, &[u8::from(show_creation_dialog)])
}

pub fn cancel_guild_creation() -> Result<Vec<u8>, EncodeError> {
    encode_short_packet(0xC1, 0x57, &[])
}

pub fn guild_war_response(accepted: bool) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet(0xC1, 0x61, &[u8::from(accepted)])
}

pub fn guild_info_request(guild_id: u32) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet(0xC1, 0x66, &guild_id.to_le_bytes())
}

pub fn guild_role_assign_request(
    role: GuildMemberRole,
    player_name: impl AsRef<[u8]>,
    type_: u8,
) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(12);
    payload.push(type_);
    payload.push(role);
    payload.extend_from_slice(&fixed_bytes::<10>(player_name));
    encode_short_packet(0xC1, 0xE1, &payload)
}

pub fn guild_type_change_request(guild_type: u8) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet(0xC1, 0xE2, &[guild_type])
}

pub fn guild_relationship_change_request(
    relationship_type: GuildRelationshipType,
    request_type: GuildRequestType,
    target_player_id: u16,
) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(4);
    payload.push(relationship_type);
    payload.push(request_type);
    payload.extend_from_slice(&target_player_id.to_be_bytes());
    encode_short_packet(0xC1, 0xE5, &payload)
}

pub fn guild_relationship_change_response(
    relationship_type: GuildRelationshipType,
    request_type: GuildRequestType,
    response: bool,
    target_player_id: u16,
) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(5);
    payload.push(relationship_type);
    payload.push(request_type);
    payload.push(u8::from(response));
    payload.extend_from_slice(&target_player_id.to_be_bytes());
    encode_short_packet(0xC1, 0xE6, &payload)
}

pub fn request_alliance_list() -> Result<Vec<u8>, EncodeError> {
    encode_short_packet(0xC1, 0xE9, &[])
}

pub fn remove_alliance_guild_request(guild_name: impl AsRef<[u8]>) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, 0xEB, 0x01, &fixed_bytes::<8>(guild_name))
}

pub fn castle_siege_status_request() -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, 0xB2, 0x00, &[])
}

pub fn castle_siege_registration_request() -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, 0xB2, 0x01, &[])
}

pub fn castle_siege_unregister_request(is_giving_up: bool) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, 0xB2, 0x02, &[u8::from(is_giving_up)])
}

pub fn castle_siege_registration_state_request() -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, 0xB2, 0x03, &[])
}

pub fn castle_siege_mark_registration(item_index: u8) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, 0xB2, 0x04, &[item_index])
}

pub fn castle_siege_defense_buy_request(
    npc_number: u32,
    npc_index: u32,
) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(8);
    payload.extend_from_slice(&npc_number.to_le_bytes());
    payload.extend_from_slice(&npc_index.to_le_bytes());
    encode_short_packet_with_subcode(0xC1, 0xB2, 0x05, &payload)
}

pub fn castle_siege_defense_repair_request(
    npc_number: u32,
    npc_index: u32,
) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(8);
    payload.extend_from_slice(&npc_number.to_le_bytes());
    payload.extend_from_slice(&npc_index.to_le_bytes());
    encode_short_packet_with_subcode(0xC1, 0xB2, 0x06, &payload)
}

pub fn castle_siege_defense_upgrade_request(
    npc_number: u32,
    npc_index: u32,
    npc_upgrade_type: u32,
    npc_upgrade_value: u32,
) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(16);
    payload.extend_from_slice(&npc_number.to_le_bytes());
    payload.extend_from_slice(&npc_index.to_le_bytes());
    payload.extend_from_slice(&npc_upgrade_type.to_le_bytes());
    payload.extend_from_slice(&npc_upgrade_value.to_le_bytes());
    encode_short_packet_with_subcode(0xC1, 0xB2, 0x07, &payload)
}

pub fn castle_siege_tax_info_request() -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, 0xB2, 0x08, &[])
}

pub fn castle_siege_tax_change_request(
    tax_type: u8,
    tax_rate: u32,
) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(5);
    payload.push(tax_type);
    payload.extend_from_slice(&tax_rate.to_be_bytes());
    encode_short_packet_with_subcode(0xC1, 0xB2, 0x09, &payload)
}

pub fn castle_siege_tax_money_withdraw(amount: u32) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, 0xB2, 0x10, &amount.to_be_bytes())
}

pub fn toggle_castle_gate_request(close_state: bool, gate_id: u16) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(3);
    payload.push(u8::from(close_state));
    payload.extend_from_slice(&gate_id.to_be_bytes());
    encode_short_packet_with_subcode(0xC1, 0xB2, 0x12, &payload)
}

pub fn castle_guild_command(
    team: u8,
    position_x: u8,
    position_y: u8,
    command: u8,
) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, 0xB2, 0x1D, &[team, position_x, position_y, command])
}

pub fn castle_siege_hunting_zone_entrance_setting(is_public: bool) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, 0xB2, 0x1F, &[u8::from(is_public)])
}

pub fn castle_siege_gate_list_request() -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, 0xB3, 0x01, &[])
}

pub fn castle_siege_statue_list_request() -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, 0xB3, 0x02, &[])
}

pub fn castle_siege_registered_guilds_list_request() -> Result<Vec<u8>, EncodeError> {
    encode_short_packet(0xC1, 0xB4, &[])
}

pub fn castle_owner_list_request() -> Result<Vec<u8>, EncodeError> {
    encode_short_packet(0xC1, 0xB5, &[])
}

pub fn fire_catapult_request(
    catapult_id: u16,
    target_area_index: u8,
) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(3);
    payload.extend_from_slice(&catapult_id.to_be_bytes());
    payload.push(target_area_index);
    encode_short_packet_with_subcode(0xC1, 0xB7, 0x01, &payload)
}

pub fn weapon_explosion_request(catapult_id: u16) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, 0xB7, 0x04, &catapult_id.to_be_bytes())
}

pub fn guild_logo_of_castle_owner_request() -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, 0xB9, 0x02, &[])
}

pub fn castle_siege_hunting_zone_enter_request(money: u32) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, 0xB9, 0x05, &money.to_le_bytes())
}

#[cfg(test)]
mod tests {
    use super::{
        cancel_guild_creation, castle_guild_command, castle_owner_list_request,
        castle_siege_defense_buy_request, castle_siege_defense_repair_request,
        castle_siege_defense_upgrade_request, castle_siege_gate_list_request,
        castle_siege_hunting_zone_enter_request, castle_siege_hunting_zone_entrance_setting,
        castle_siege_mark_registration, castle_siege_registered_guilds_list_request,
        castle_siege_registration_request, castle_siege_registration_state_request,
        castle_siege_statue_list_request, castle_siege_status_request,
        castle_siege_tax_change_request, castle_siege_tax_info_request,
        castle_siege_tax_money_withdraw, castle_siege_unregister_request, fire_catapult_request,
        guild_create_request, guild_create_request_075, guild_info_request, guild_join_request,
        guild_join_response, guild_kick_player_request, guild_list_request,
        guild_logo_of_castle_owner_request, guild_master_answer, guild_relationship_change_request,
        guild_relationship_change_response, guild_role_assign_request, guild_type_change_request,
        guild_war_response, remove_alliance_guild_request, request_alliance_list,
        toggle_castle_gate_request, weapon_explosion_request,
    };
    use crate::wire::fixed_bytes;

    #[test]
    fn encodes_guild_packets() {
        assert_eq!(
            guild_kick_player_request(b"Alice", b"123").unwrap(),
            vec![0xC1, 0x10, 0x53, b'A', b'l', b'i', b'c', b'e', 0, 0, 0, 0, 0, b'1', b'2', b'3']
        );
        assert_eq!(
            guild_join_request(0x1234).unwrap(),
            vec![0xC1, 0x05, 0x50, 0x12, 0x34]
        );
        assert_eq!(
            guild_join_response(true, 0x1234).unwrap(),
            vec![0xC1, 0x06, 0x51, 0x01, 0x12, 0x34]
        );
        assert_eq!(guild_list_request().unwrap(), vec![0xC1, 0x03, 0x52]);
        let mut guild_create_expected = vec![0xC1, 0x2C, 0x55, 0x00];
        guild_create_expected.extend_from_slice(&fixed_bytes::<8>(b"Guild"));
        guild_create_expected.extend_from_slice(&[1u8; 32]);
        assert_eq!(
            guild_create_request(b"Guild", [1u8; 32]).unwrap(),
            guild_create_expected
        );
        assert_eq!(guild_create_request_075(b"Guild", [1u8; 32]).unwrap(), {
            let mut guild_create_expected = vec![0xC1, 0x2B, 0x55];
            guild_create_expected.extend_from_slice(&fixed_bytes::<8>(b"Guild"));
            guild_create_expected.extend_from_slice(&[1u8; 32]);
            guild_create_expected
        });
        assert_eq!(
            guild_master_answer(true).unwrap(),
            vec![0xC1, 0x04, 0x54, 0x01]
        );
        assert_eq!(cancel_guild_creation().unwrap(), vec![0xC1, 0x03, 0x57]);
        assert_eq!(
            guild_war_response(false).unwrap(),
            vec![0xC1, 0x04, 0x61, 0x00]
        );
        assert_eq!(
            guild_info_request(0x01020304).unwrap(),
            vec![0xC1, 0x07, 0x66, 0x04, 0x03, 0x02, 0x01]
        );
        assert_eq!(
            guild_role_assign_request(2, b"Alice", 1).unwrap(),
            vec![0xC1, 0x0F, 0xE1, 0x01, 0x02, b'A', b'l', b'i', b'c', b'e', 0, 0, 0, 0, 0]
        );
        assert_eq!(
            guild_type_change_request(0xFF).unwrap(),
            vec![0xC1, 0x04, 0xE2, 0xFF]
        );
        assert_eq!(
            guild_relationship_change_request(1, 2, 0x1234).unwrap(),
            vec![0xC1, 0x07, 0xE5, 0x01, 0x02, 0x12, 0x34]
        );
        assert_eq!(
            guild_relationship_change_response(1, 2, true, 0x1234).unwrap(),
            vec![0xC1, 0x08, 0xE6, 0x01, 0x02, 0x01, 0x12, 0x34]
        );
        assert_eq!(request_alliance_list().unwrap(), vec![0xC1, 0x03, 0xE9]);
        assert_eq!(
            remove_alliance_guild_request(b"Alliance").unwrap(),
            vec![0xC1, 0x0C, 0xEB, 0x01, b'A', b'l', b'l', b'i', b'a', b'n', b'c', b'e']
        );
    }

    #[test]
    fn encodes_castle_packets() {
        assert_eq!(
            castle_siege_status_request().unwrap(),
            vec![0xC1, 0x04, 0xB2, 0x00]
        );
        assert_eq!(
            castle_siege_registration_request().unwrap(),
            vec![0xC1, 0x04, 0xB2, 0x01]
        );
        assert_eq!(
            castle_siege_unregister_request(true).unwrap(),
            vec![0xC1, 0x05, 0xB2, 0x02, 0x01]
        );
        assert_eq!(
            castle_siege_registration_state_request().unwrap(),
            vec![0xC1, 0x04, 0xB2, 0x03]
        );
        assert_eq!(
            castle_siege_mark_registration(7).unwrap(),
            vec![0xC1, 0x05, 0xB2, 0x04, 0x07]
        );
        assert_eq!(
            castle_siege_defense_buy_request(1, 2).unwrap(),
            vec![0xC1, 0x0C, 0xB2, 0x05, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00]
        );
        assert_eq!(
            castle_siege_defense_repair_request(1, 2).unwrap(),
            vec![0xC1, 0x0C, 0xB2, 0x06, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00]
        );
        assert_eq!(
            castle_siege_defense_upgrade_request(1, 2, 3, 4).unwrap(),
            vec![
                0xC1, 0x14, 0xB2, 0x07, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x03, 0x00,
                0x00, 0x00, 0x04, 0x00, 0x00, 0x00
            ]
        );
        assert_eq!(
            castle_siege_tax_info_request().unwrap(),
            vec![0xC1, 0x04, 0xB2, 0x08]
        );
        assert_eq!(
            castle_siege_tax_change_request(3, 4).unwrap(),
            vec![0xC1, 0x09, 0xB2, 0x09, 0x03, 0x00, 0x00, 0x00, 0x04]
        );
        assert_eq!(
            castle_siege_tax_money_withdraw(4).unwrap(),
            vec![0xC1, 0x08, 0xB2, 0x10, 0x00, 0x00, 0x00, 0x04]
        );
        assert_eq!(
            toggle_castle_gate_request(false, 0x1234).unwrap(),
            vec![0xC1, 0x07, 0xB2, 0x12, 0x00, 0x12, 0x34]
        );
        assert_eq!(
            castle_guild_command(1, 2, 3, 4).unwrap(),
            vec![0xC1, 0x08, 0xB2, 0x1D, 0x01, 0x02, 0x03, 0x04]
        );
        assert_eq!(
            castle_siege_hunting_zone_entrance_setting(true).unwrap(),
            vec![0xC1, 0x05, 0xB2, 0x1F, 0x01]
        );
        assert_eq!(
            castle_siege_gate_list_request().unwrap(),
            vec![0xC1, 0x04, 0xB3, 0x01]
        );
        assert_eq!(
            castle_siege_statue_list_request().unwrap(),
            vec![0xC1, 0x04, 0xB3, 0x02]
        );
        assert_eq!(
            castle_siege_registered_guilds_list_request().unwrap(),
            vec![0xC1, 0x03, 0xB4]
        );
        assert_eq!(castle_owner_list_request().unwrap(), vec![0xC1, 0x03, 0xB5]);
        assert_eq!(
            fire_catapult_request(0x1234, 5).unwrap(),
            vec![0xC1, 0x07, 0xB7, 0x01, 0x12, 0x34, 0x05]
        );
        assert_eq!(
            weapon_explosion_request(0x1234).unwrap(),
            vec![0xC1, 0x06, 0xB7, 0x04, 0x12, 0x34]
        );
        assert_eq!(
            guild_logo_of_castle_owner_request().unwrap(),
            vec![0xC1, 0x04, 0xB9, 0x02]
        );
        assert_eq!(
            castle_siege_hunting_zone_enter_request(4).unwrap(),
            vec![0xC1, 0x08, 0xB9, 0x05, 0x04, 0x00, 0x00, 0x00]
        );

        assert_eq!(
            fixed_bytes::<8>(b"Alliance"),
            [b'A', b'l', b'l', b'i', b'a', b'n', b'c', b'e']
        );
    }
}
