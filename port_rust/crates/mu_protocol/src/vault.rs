use crate::error::PacketCodecError as EncodeError;
use crate::wire::encode_short_packet;

pub type VaultMoneyMoveDirection = u8;

pub fn vault_closed() -> Result<Vec<u8>, EncodeError> {
    encode_short_packet(0xC1, 0x82, &[])
}

pub fn vault_move_money_request(
    direction: VaultMoneyMoveDirection,
    amount: u32,
) -> Result<Vec<u8>, EncodeError> {
    let mut payload = Vec::with_capacity(5);
    payload.push(direction);
    payload.extend_from_slice(&amount.to_le_bytes());
    encode_short_packet(0xC1, 0x81, &payload)
}

#[cfg(test)]
mod tests {
    use super::{vault_closed, vault_move_money_request};

    #[test]
    fn encodes_vault_packets() {
        assert_eq!(vault_closed().unwrap(), vec![0xC1, 0x03, 0x82]);
        assert_eq!(
            vault_move_money_request(1, 0x01020304).unwrap(),
            vec![0xC1, 0x08, 0x81, 0x01, 0x04, 0x03, 0x02, 0x01]
        );
    }
}
