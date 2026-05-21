use crate::error::PacketCodecError as EncodeError;
use crate::wire::{encode_short_packet, encode_short_packet_with_subcode};

pub use crate::admin::{enter_market_place_request, lucky_number_request};

pub type LegacyQuestState = u8;
pub type MiniGameType = u8;
pub type QuestProceedAction = u8;

pub fn legacy_quest_state_request() -> Result<Vec<u8>, EncodeError> {
    encode_short_packet(0xC1, 0xA0, &[])
}

pub fn legacy_quest_state_set_request(
    quest_number: u8,
    new_state: LegacyQuestState,
) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet(0xC1, 0xA2, &[quest_number, new_state])
}

pub fn quest_select_request(
    quest_number: u16,
    quest_group: u16,
    selected_text_index: u8,
) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(5);
    payload.extend_from_slice(&quest_number.to_le_bytes());
    payload.extend_from_slice(&quest_group.to_le_bytes());
    payload.push(selected_text_index);
    encode_short_packet_with_subcode(0xC1, 0xF6, 0x0A, &payload)
}

pub fn quest_proceed_request(
    quest_number: u16,
    quest_group: u16,
    proceed_action: QuestProceedAction,
) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(5);
    payload.extend_from_slice(&quest_number.to_le_bytes());
    payload.extend_from_slice(&quest_group.to_le_bytes());
    payload.push(proceed_action);
    encode_short_packet_with_subcode(0xC1, 0xF6, 0x0B, &payload)
}

pub fn quest_completion_request(
    quest_number: u16,
    quest_group: u16,
) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(4);
    payload.extend_from_slice(&quest_number.to_le_bytes());
    payload.extend_from_slice(&quest_group.to_le_bytes());
    encode_short_packet_with_subcode(0xC1, 0xF6, 0x0D, &payload)
}

pub fn quest_cancel_request(quest_number: u16, quest_group: u16) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(4);
    payload.extend_from_slice(&quest_number.to_le_bytes());
    payload.extend_from_slice(&quest_group.to_le_bytes());
    encode_short_packet_with_subcode(0xC1, 0xF6, 0x0F, &payload)
}

pub fn quest_client_action_request(
    quest_number: u16,
    quest_group: u16,
) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(4);
    payload.extend_from_slice(&quest_number.to_le_bytes());
    payload.extend_from_slice(&quest_group.to_le_bytes());
    encode_short_packet_with_subcode(0xC1, 0xF6, 0x10, &payload)
}

pub fn active_quest_list_request() -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, 0xF6, 0x1A, &[])
}

pub fn quest_state_request(quest_number: u16, quest_group: u16) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(4);
    payload.extend_from_slice(&quest_number.to_le_bytes());
    payload.extend_from_slice(&quest_group.to_le_bytes());
    encode_short_packet_with_subcode(0xC1, 0xF6, 0x1B, &payload)
}

pub fn event_quest_state_list_request() -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, 0xF6, 0x21, &[])
}

pub fn available_quests_request() -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, 0xF6, 0x30, &[])
}

pub fn npc_buff_request() -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, 0xF6, 0x31, &[])
}

pub fn enter_empire_guardian_event(item_slot: u8) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, 0xF7, 0x01, &[item_slot])
}

pub fn devil_square_enter_request(
    square_level: u8,
    ticket_item_inventory_index: u8,
) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet(0xC1, 0x90, &[square_level, ticket_item_inventory_index])
}

pub fn mini_game_opening_state_request(
    event_type: MiniGameType,
    event_level: u8,
) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet(0xC1, 0x91, &[event_type, event_level])
}

pub fn event_chip_registration_request(type_: u8, item_index: u8) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet(0xC1, 0x95, &[type_, item_index])
}

pub fn muto_number_request() -> Result<Vec<u8>, EncodeError> {
    encode_short_packet(0xC1, 0x96, &[])
}

pub fn event_chip_exit_dialog() -> Result<Vec<u8>, EncodeError> {
    encode_short_packet(0xC1, 0x97, &[])
}

pub fn event_chip_exchange_request(type_: u8) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet(0xC1, 0x98, &[type_])
}

pub fn white_angel_item_request() -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, 0xD0, 0x03, &[])
}

pub fn enter_on_werewolf_request() -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, 0xD0, 0x07, &[])
}

pub fn enter_on_gatekeeper_request() -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, 0xD0, 0x08, &[])
}

pub fn leo_helper_item_request() -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, 0xD0, 0x09, &[])
}

pub fn move_to_devias_by_snowman_request() -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, 0xD0, 0x0A, &[])
}

pub fn santa_claus_item_request() -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, 0xD0, 0x10, &[])
}

pub fn illusion_temple_enter_request(
    map_number: u8,
    item_slot: u8,
) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, 0xBF, 0x00, &[map_number, item_slot])
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

pub fn illusion_temple_reward_request() -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, 0xBF, 0x05, &[])
}

pub fn lucky_coin_count_request() -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, 0xBF, 0x0B, &[])
}

pub fn lucky_coin_registration_request() -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, 0xBF, 0x0C, &[])
}

pub fn lucky_coin_exchange_request(coin_count: u32) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, 0xBF, 0x0D, &coin_count.to_le_bytes())
}

pub fn doppelganger_enter_request(ticket_item_slot: u8) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, 0xBF, 0x0E, &[ticket_item_slot])
}

pub fn blood_castle_enter_request(
    castle_level: u8,
    ticket_item_inventory_index: u8,
) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet(0xC1, 0x9A, &[castle_level, ticket_item_inventory_index])
}

pub fn mini_game_event_count_request(mini_game: MiniGameType) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet(0xC1, 0x9F, &[mini_game])
}

pub fn chaos_castle_enter_request(
    castle_level: u8,
    ticket_item_inventory_index: u8,
) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(
        0xC1,
        0xAF,
        0x01,
        &[castle_level, ticket_item_inventory_index],
    )
}

pub fn chaos_castle_position_set(position_x: u8, position_y: u8) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, 0xAF, 0x02, &[position_x, position_y])
}

#[cfg(test)]
mod tests {
    use super::{
        active_quest_list_request, available_quests_request, blood_castle_enter_request,
        chaos_castle_enter_request, chaos_castle_position_set, devil_square_enter_request,
        doppelganger_enter_request, enter_empire_guardian_event, enter_market_place_request,
        enter_on_gatekeeper_request, enter_on_werewolf_request, event_chip_exchange_request,
        event_chip_exit_dialog, event_chip_registration_request, event_quest_state_list_request,
        illusion_temple_enter_request, illusion_temple_reward_request,
        illusion_temple_skill_request, legacy_quest_state_request, legacy_quest_state_set_request,
        leo_helper_item_request, lucky_coin_count_request, lucky_coin_exchange_request,
        lucky_coin_registration_request, lucky_number_request, mini_game_event_count_request,
        mini_game_opening_state_request, move_to_devias_by_snowman_request, muto_number_request,
        npc_buff_request, quest_cancel_request, quest_client_action_request,
        quest_completion_request, quest_proceed_request, quest_select_request, quest_state_request,
        santa_claus_item_request, white_angel_item_request,
    };

    #[test]
    fn encodes_quest_packets() {
        assert_eq!(
            legacy_quest_state_request().unwrap(),
            vec![0xC1, 0x03, 0xA0]
        );
        assert_eq!(
            legacy_quest_state_set_request(7, 1).unwrap(),
            vec![0xC1, 0x05, 0xA2, 0x07, 0x01]
        );
        assert_eq!(
            quest_select_request(0x0102, 0x0304, 5).unwrap(),
            vec![0xC1, 0x09, 0xF6, 0x0A, 0x02, 0x01, 0x04, 0x03, 0x05]
        );
        assert_eq!(
            quest_proceed_request(0x0102, 0x0304, 1).unwrap(),
            vec![0xC1, 0x09, 0xF6, 0x0B, 0x02, 0x01, 0x04, 0x03, 0x01]
        );
        assert_eq!(
            quest_completion_request(0x0102, 0x0304).unwrap(),
            vec![0xC1, 0x08, 0xF6, 0x0D, 0x02, 0x01, 0x04, 0x03]
        );
        assert_eq!(
            quest_cancel_request(0x0102, 0x0304).unwrap(),
            vec![0xC1, 0x08, 0xF6, 0x0F, 0x02, 0x01, 0x04, 0x03]
        );
        assert_eq!(
            quest_client_action_request(0x0102, 0x0304).unwrap(),
            vec![0xC1, 0x08, 0xF6, 0x10, 0x02, 0x01, 0x04, 0x03]
        );
        assert_eq!(
            active_quest_list_request().unwrap(),
            vec![0xC1, 0x04, 0xF6, 0x1A]
        );
        assert_eq!(
            quest_state_request(0x0102, 0x0304).unwrap(),
            vec![0xC1, 0x08, 0xF6, 0x1B, 0x02, 0x01, 0x04, 0x03]
        );
        assert_eq!(
            event_quest_state_list_request().unwrap(),
            vec![0xC1, 0x04, 0xF6, 0x21]
        );
        assert_eq!(
            available_quests_request().unwrap(),
            vec![0xC1, 0x04, 0xF6, 0x30]
        );
        assert_eq!(npc_buff_request().unwrap(), vec![0xC1, 0x04, 0xF6, 0x31]);
        assert_eq!(
            enter_market_place_request().unwrap(),
            vec![0xC1, 0x04, 0xBF, 0x17]
        );
        assert_eq!(
            enter_empire_guardian_event(1).unwrap(),
            vec![0xC1, 0x05, 0xF7, 0x01, 0x01]
        );
        assert_eq!(
            devil_square_enter_request(2, 3).unwrap(),
            vec![0xC1, 0x05, 0x90, 0x02, 0x03]
        );
        assert_eq!(
            mini_game_opening_state_request(4, 5).unwrap(),
            vec![0xC1, 0x05, 0x91, 0x04, 0x05]
        );
        assert_eq!(
            event_chip_registration_request(6, 7).unwrap(),
            vec![0xC1, 0x05, 0x95, 0x06, 0x07]
        );
        assert_eq!(muto_number_request().unwrap(), vec![0xC1, 0x03, 0x96]);
        assert_eq!(event_chip_exit_dialog().unwrap(), vec![0xC1, 0x03, 0x97]);
        assert_eq!(
            event_chip_exchange_request(8).unwrap(),
            vec![0xC1, 0x04, 0x98, 0x08]
        );
    }

    #[test]
    fn encodes_event_packets() {
        assert_eq!(
            white_angel_item_request().unwrap(),
            vec![0xC1, 0x04, 0xD0, 0x03]
        );
        assert_eq!(
            enter_on_werewolf_request().unwrap(),
            vec![0xC1, 0x04, 0xD0, 0x07]
        );
        assert_eq!(
            enter_on_gatekeeper_request().unwrap(),
            vec![0xC1, 0x04, 0xD0, 0x08]
        );
        assert_eq!(
            leo_helper_item_request().unwrap(),
            vec![0xC1, 0x04, 0xD0, 0x09]
        );
        assert_eq!(
            move_to_devias_by_snowman_request().unwrap(),
            vec![0xC1, 0x04, 0xD0, 0x0A]
        );
        assert_eq!(
            santa_claus_item_request().unwrap(),
            vec![0xC1, 0x04, 0xD0, 0x10]
        );
        assert_eq!(
            illusion_temple_enter_request(1, 2).unwrap(),
            vec![0xC1, 0x06, 0xBF, 0x00, 0x01, 0x02]
        );
        assert_eq!(
            illusion_temple_skill_request(0x1234, 5, 6).unwrap(),
            vec![0xC1, 0x08, 0xBF, 0x02, 0x12, 0x34, 0x05, 0x06]
        );
        assert_eq!(
            illusion_temple_reward_request().unwrap(),
            vec![0xC1, 0x04, 0xBF, 0x05]
        );
        assert_eq!(
            lucky_coin_count_request().unwrap(),
            vec![0xC1, 0x04, 0xBF, 0x0B]
        );
        assert_eq!(
            lucky_coin_registration_request().unwrap(),
            vec![0xC1, 0x04, 0xBF, 0x0C]
        );
        assert_eq!(
            lucky_coin_exchange_request(0x01020304).unwrap(),
            vec![0xC1, 0x08, 0xBF, 0x0D, 0x04, 0x03, 0x02, 0x01]
        );
        assert_eq!(
            doppelganger_enter_request(7).unwrap(),
            vec![0xC1, 0x05, 0xBF, 0x0E, 0x07]
        );
        assert_eq!(
            blood_castle_enter_request(8, 9).unwrap(),
            vec![0xC1, 0x05, 0x9A, 0x08, 0x09]
        );
        assert_eq!(
            mini_game_event_count_request(3).unwrap(),
            vec![0xC1, 0x04, 0x9F, 0x03]
        );
        assert_eq!(
            chaos_castle_enter_request(4, 5).unwrap(),
            vec![0xC1, 0x06, 0xAF, 0x01, 0x04, 0x05]
        );
        assert_eq!(
            chaos_castle_position_set(6, 7).unwrap(),
            vec![0xC1, 0x06, 0xAF, 0x02, 0x06, 0x07]
        );
        assert_eq!(
            lucky_number_request(b"1234", b"5678", b"9012").unwrap(),
            vec![
                0xC1, 0x12, 0x9D, b'1', b'2', b'3', b'4', 0x00, b'5', b'6', b'7', b'8', 0x00, b'9',
                b'0', b'1', b'2', 0x00
            ]
        );
    }
}
