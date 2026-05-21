use crate::error::PacketCodecError as EncodeError;
use crate::wire::encode_short_packet;

pub type TradeButtonState = u8;

pub fn trade_cancel() -> Result<Vec<u8>, EncodeError> {
    encode_short_packet(0xC1, 0x3D, &[])
}

pub fn trade_button_state_change(new_state: TradeButtonState) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet(0xC1, 0x3C, &[new_state])
}

pub fn trade_request(player_id: u16) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet(0xC3, 0x36, &player_id.to_be_bytes())
}

pub fn trade_request_response(trade_accepted: bool) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet(0xC1, 0x37, &[u8::from(trade_accepted)])
}

pub fn set_trade_money(amount: u32) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet(0xC1, 0x3A, &amount.to_le_bytes())
}

#[cfg(test)]
mod tests {
    use super::{
        set_trade_money, trade_button_state_change, trade_cancel, trade_request,
        trade_request_response,
    };

    #[test]
    fn encodes_trade_packets() {
        assert_eq!(trade_cancel().unwrap(), vec![0xC1, 0x03, 0x3D]);
        assert_eq!(
            trade_button_state_change(2).unwrap(),
            vec![0xC1, 0x04, 0x3C, 0x02]
        );
        assert_eq!(
            trade_request(0x1234).unwrap(),
            vec![0xC3, 0x05, 0x36, 0x12, 0x34]
        );
        assert_eq!(
            trade_request_response(true).unwrap(),
            vec![0xC1, 0x04, 0x37, 0x01]
        );
        assert_eq!(
            trade_request_response(false).unwrap(),
            vec![0xC1, 0x04, 0x37, 0x00]
        );
        assert_eq!(
            set_trade_money(0x01020304).unwrap(),
            vec![0xC1, 0x07, 0x3A, 0x04, 0x03, 0x02, 0x01]
        );
    }
}
