use std::convert::TryFrom;

use crate::error::PacketCodecError as EncodeError;
use crate::wire::{encode_short_packet_with_subcode, fixed_bytes};

pub type CharacterCreationUnlockFlags = u8;
pub type CharacterClassNumber = u8;
pub type CharacterStatus = u8;
pub type GuildMemberRole = u8;
pub type LoginResult = u8;
pub type LogoutType = u8;
pub type CharacterDeleteResult = u8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CharacterListEntry<'a> {
    pub slot_index: u8,
    pub name: &'a [u8],
    pub level: u16,
    pub status: CharacterStatus,
    pub is_item_block_active: bool,
    pub appearance: &'a [u8],
    pub guild_position: GuildMemberRole,
}

pub fn game_server_entered(
    success: bool,
    player_id: u16,
    version: impl AsRef<[u8]>,
) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(8);
    payload.push(if success { 1 } else { 0 });
    payload.extend_from_slice(&player_id.to_be_bytes());
    payload.extend_from_slice(&fixed_bytes::<5>(version));
    encode_short_packet_with_subcode(0xC1, 0xF1, 0x00, &payload)
}

pub fn login_response(result: LoginResult) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, 0xF1, 0x01, &[result])
}

pub fn logout_response(type_: LogoutType) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC3, 0xF1, 0x02, &[type_])
}

pub fn character_class_creation_unlock(
    flags: CharacterCreationUnlockFlags,
) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, 0xDE, 0x00, &[flags])
}

pub fn character_creation_successful(
    character_name: impl AsRef<[u8]>,
    character_slot: u8,
    level: u16,
    class_: CharacterClassNumber,
    character_status: CharacterStatus,
    preview_data: impl AsRef<[u8]>,
) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(38);
    payload.push(1);
    payload.extend_from_slice(&fixed_bytes::<10>(character_name));
    payload.push(character_slot);
    payload.extend_from_slice(&level.to_le_bytes());
    payload.push(class_);
    payload.push(character_status);
    payload.extend_from_slice(&fixed_bytes::<22>(preview_data));
    encode_short_packet_with_subcode(0xC1, 0xF3, 0x01, &payload)
}

pub fn character_creation_failed() -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, 0xF3, 0x01, &[0x00])
}

pub fn character_delete_response(result: CharacterDeleteResult) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, 0xF3, 0x02, &[result])
}

pub fn character_list(
    unlock_flags: CharacterCreationUnlockFlags,
    move_cnt: u8,
    is_vault_extended: bool,
    entries: &[CharacterListEntry<'_>],
) -> Result<Vec<u8>, EncodeError> {
    let payload = build_character_list_payload(
        &[
            unlock_flags,
            move_cnt,
            count_to_u8(entries.len())?,
            is_vault_extended as u8,
        ],
        entries,
        18,
        LevelEndian::Little,
        true,
        1,
    )?;
    encode_short_packet_with_subcode(0xC1, 0xF3, 0x00, &payload)
}

pub fn character_list_extended(
    unlock_flags: CharacterCreationUnlockFlags,
    move_cnt: u8,
    is_vault_extended: bool,
    entries: &[CharacterListEntry<'_>],
) -> Result<Vec<u8>, EncodeError> {
    let payload = build_character_list_payload(
        &[
            unlock_flags,
            move_cnt,
            count_to_u8(entries.len())?,
            is_vault_extended as u8,
        ],
        entries,
        27,
        LevelEndian::Little,
        true,
        2,
    )?;
    encode_short_packet_with_subcode(0xC1, 0xF3, 0x00, &payload)
}

pub fn character_list_075(entries: &[CharacterListEntry<'_>]) -> Result<Vec<u8>, EncodeError> {
    let payload = build_character_list_payload(
        &[count_to_u8(entries.len())?],
        entries,
        9,
        LevelEndian::Big,
        false,
        1,
    )?;
    encode_short_packet_with_subcode(0xC1, 0xF3, 0x00, &payload)
}

pub fn character_list_095(entries: &[CharacterListEntry<'_>]) -> Result<Vec<u8>, EncodeError> {
    let payload = build_character_list_payload(
        &[count_to_u8(entries.len())?],
        entries,
        11,
        LevelEndian::Little,
        false,
        1,
    )?;
    encode_short_packet_with_subcode(0xC1, 0xF3, 0x00, &payload)
}

fn count_to_u8(count: usize) -> Result<u8, EncodeError> {
    u8::try_from(count).map_err(|_| EncodeError::PacketTooLarge { size: count })
}

enum LevelEndian {
    Big,
    Little,
}

fn build_character_list_payload(
    prefix: &[u8],
    entries: &[CharacterListEntry<'_>],
    appearance_len: usize,
    level_endian: LevelEndian,
    include_guild_position: bool,
    padding_bytes: usize,
) -> Result<Vec<u8>, EncodeError> {
    let entry_len =
        1 + 10 + 2 + 1 + appearance_len + include_guild_position as usize + padding_bytes;
    let payload_bytes = prefix
        .len()
        .checked_add(
            entries
                .len()
                .checked_mul(entry_len)
                .ok_or(EncodeError::PacketTooLarge { size: usize::MAX })?,
        )
        .ok_or(EncodeError::PacketTooLarge { size: usize::MAX })?;

    let mut payload = Vec::with_capacity(payload_bytes);
    payload.extend_from_slice(prefix);

    for entry in entries {
        payload.push(entry.slot_index);
        payload.extend_from_slice(&fixed_bytes::<10>(entry.name));
        match level_endian {
            LevelEndian::Big => payload.extend_from_slice(&entry.level.to_be_bytes()),
            LevelEndian::Little => payload.extend_from_slice(&entry.level.to_le_bytes()),
        }
        payload.push(pack_character_status(
            entry.status,
            entry.is_item_block_active,
        ));
        let appearance = fixed_bytes::<27>(entry.appearance);
        payload.extend_from_slice(&appearance[..appearance_len]);
        if include_guild_position {
            payload.push(entry.guild_position);
        }
        if padding_bytes > 0 {
            payload.resize(payload.len() + padding_bytes, 0x00);
        }
    }

    Ok(payload)
}

fn pack_character_status(status: CharacterStatus, is_item_block_active: bool) -> u8 {
    status | ((is_item_block_active as u8) << 4)
}

#[cfg(test)]
mod tests {
    use super::{
        character_class_creation_unlock, character_creation_failed, character_creation_successful,
        character_delete_response, character_list, character_list_075, character_list_095,
        character_list_extended, game_server_entered, login_response, logout_response,
        CharacterListEntry,
    };

    #[test]
    fn encodes_session_packets() {
        assert_eq!(
            game_server_entered(true, 7, b"1.0.0").unwrap(),
            vec![0xC1, 0x0C, 0xF1, 0x00, 0x01, 0x00, 0x07, b'1', b'.', b'0', b'.', b'0']
        );
        assert_eq!(
            login_response(1).unwrap(),
            vec![0xC1, 0x05, 0xF1, 0x01, 0x01]
        );
        assert_eq!(
            logout_response(2).unwrap(),
            vec![0xC3, 0x05, 0xF1, 0x02, 0x02]
        );
        assert_eq!(
            character_class_creation_unlock(0x0F).unwrap(),
            vec![0xC1, 0x05, 0xDE, 0x00, 0x0F]
        );
        assert_eq!(
            character_creation_failed().unwrap(),
            vec![0xC1, 0x05, 0xF3, 0x01, 0x00]
        );
        assert_eq!(
            character_delete_response(1).unwrap(),
            vec![0xC1, 0x05, 0xF3, 0x02, 0x01]
        );
        assert_eq!(
            character_creation_successful(b"Alice", 2, 99, 4, 32, b"preview")
                .unwrap()
                .len(),
            42
        );
    }

    #[test]
    fn encodes_character_lists() {
        let entry = CharacterListEntry {
            slot_index: 1,
            name: b"Alice",
            level: 42,
            status: 32,
            is_item_block_active: true,
            appearance: b"appearance",
            guild_position: 128,
        };

        assert_eq!(
            character_list(1, 2, true, &[entry]).unwrap().len(),
            4 + 4 + 34
        );
        assert_eq!(
            character_list_extended(1, 2, true, &[entry]).unwrap().len(),
            4 + 4 + 44
        );
        assert_eq!(character_list_075(&[entry]).unwrap().len(), 29);
        assert_eq!(character_list_095(&[entry]).unwrap().len(), 31);
    }
}
