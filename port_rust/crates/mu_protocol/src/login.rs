use crate::error::PacketCodecError as EncodeError;
use crate::wire::{encode_short_packet_with_subcode, fixed_bytes, xor3_encrypt};

pub use crate::session::LogoutType;

pub fn login_long_password(
    username: impl AsRef<[u8]>,
    password: impl AsRef<[u8]>,
    tick_count: u32,
    client_version: impl AsRef<[u8]>,
    client_serial: impl AsRef<[u8]>,
) -> Result<Vec<u8>, EncodeError> {
    encode_login_packet::<20, 5>(
        username,
        password,
        tick_count,
        client_version,
        client_serial,
    )
}

pub fn login_short_password(
    username: impl AsRef<[u8]>,
    password: impl AsRef<[u8]>,
    tick_count: u32,
    client_version: impl AsRef<[u8]>,
    client_serial: impl AsRef<[u8]>,
) -> Result<Vec<u8>, EncodeError> {
    encode_login_packet::<10, 5>(
        username,
        password,
        tick_count,
        client_version,
        client_serial,
    )
}

pub fn login_075(
    username: impl AsRef<[u8]>,
    password: impl AsRef<[u8]>,
    tick_count: u32,
    client_version: impl AsRef<[u8]>,
    client_serial: impl AsRef<[u8]>,
) -> Result<Vec<u8>, EncodeError> {
    encode_login_packet::<10, 3>(
        username,
        password,
        tick_count,
        client_version,
        client_serial,
    )
}

pub fn logout(type_: LogoutType) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC3, 0xF1, 0x02, &[type_])
}

pub fn logout_by_cheat_detection(param: u8) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC3, 0xF1, 0x03, &[4, param])
}

pub fn request_character_list(language: u8) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, 0xF3, 0x00, &[language])
}

pub fn create_character(
    name: impl AsRef<[u8]>,
    class_: crate::session::CharacterClassNumber,
) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(11);
    payload.extend_from_slice(&fixed_bytes::<10>(name));
    payload.push(class_);
    encode_short_packet_with_subcode(0xC1, 0xF3, 0x01, &payload)
}

pub fn delete_character(
    name: impl AsRef<[u8]>,
    security_code: impl AsRef<[u8]>,
) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(20);
    payload.extend_from_slice(&fixed_bytes::<10>(name));
    payload.extend_from_slice(&fixed_bytes::<10>(security_code));
    encode_short_packet_with_subcode(0xC1, 0xF3, 0x02, &payload)
}

pub fn select_character(name: impl AsRef<[u8]>) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(10);
    payload.extend_from_slice(&fixed_bytes::<10>(name));
    encode_short_packet_with_subcode(0xC1, 0xF3, 0x03, &payload)
}

fn encode_login_packet<const PASSWORD_LEN: usize, const CLIENT_VERSION_LEN: usize>(
    username: impl AsRef<[u8]>,
    password: impl AsRef<[u8]>,
    tick_count: u32,
    client_version: impl AsRef<[u8]>,
    client_serial: impl AsRef<[u8]>,
) -> Result<Vec<u8>, EncodeError> {
    let mut username = fixed_bytes::<10>(username);
    xor3_encrypt(&mut username, 0);

    let mut password = fixed_bytes::<PASSWORD_LEN>(password);
    xor3_encrypt(&mut password, 0);

    let client_version = fixed_bytes::<CLIENT_VERSION_LEN>(client_version);
    let client_serial = fixed_bytes::<16>(client_serial);

    let mut payload = Vec::with_capacity(10 + PASSWORD_LEN + 4 + CLIENT_VERSION_LEN + 16 + 1);
    payload.extend_from_slice(&username);
    payload.extend_from_slice(&password);
    payload.extend_from_slice(&tick_count.to_be_bytes());
    payload.extend_from_slice(&client_version);
    payload.extend_from_slice(&client_serial);
    payload.push(0x00);

    encode_short_packet_with_subcode(0xC3, 0xF1, 0x01, &payload)
}

#[cfg(test)]
mod tests {
    use super::{
        create_character, delete_character, login_075, login_long_password, login_short_password,
        logout, logout_by_cheat_detection, request_character_list, select_character,
    };

    #[test]
    fn encodes_login_packets() {
        assert_eq!(
            login_long_password(b"user", b"password", 0x01020304, b"1.0.0", b"serial")
                .unwrap()
                .len(),
            60
        );
        assert_eq!(
            login_short_password(b"user", b"pass", 0x01020304, b"1.0.0", b"serial")
                .unwrap()
                .len(),
            50
        );
        assert_eq!(
            login_075(b"user", b"pass", 0x01020304, b"1.0", b"serial")
                .unwrap()
                .len(),
            48
        );
    }

    #[test]
    fn encodes_logout_and_character_packets() {
        assert_eq!(logout(0).unwrap(), vec![0xC3, 0x05, 0xF1, 0x02, 0x00]);
        assert_eq!(
            logout_by_cheat_detection(9).unwrap(),
            vec![0xC3, 0x06, 0xF1, 0x03, 0x04, 0x09]
        );
        assert_eq!(
            request_character_list(1).unwrap(),
            vec![0xC1, 0x05, 0xF3, 0x00, 0x01]
        );
        assert_eq!(
            create_character(b"Alice", 4).unwrap(),
            vec![0xC1, 0x0F, 0xF3, 0x01, b'A', b'l', b'i', b'c', b'e', 0, 0, 0, 0, 0, 4]
        );
        assert_eq!(
            delete_character(b"Alice", b"1234").unwrap(),
            vec![
                0xC1, 0x18, 0xF3, 0x02, b'A', b'l', b'i', b'c', b'e', 0, 0, 0, 0, 0, b'1', b'2',
                b'3', b'4', 0, 0, 0, 0, 0, 0
            ]
        );
        assert_eq!(
            select_character(b"Alice").unwrap(),
            vec![0xC1, 0x0E, 0xF3, 0x03, b'A', b'l', b'i', b'c', b'e', 0, 0, 0, 0, 0]
        );
    }
}
