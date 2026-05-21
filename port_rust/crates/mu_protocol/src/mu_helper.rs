use crate::error::PacketCodecError as EncodeError;
use crate::wire::{encode_long_packet, encode_short_packet_with_subcode, fixed_bytes};

pub fn mu_helper_status_change_request(pause_status: bool) -> Result<Vec<u8>, EncodeError> {
    encode_short_packet_with_subcode(0xC1, 0xBF, 0x51, &[u8::from(pause_status)])
}

pub fn mu_helper_save_data_request(helper_data: impl AsRef<[u8]>) -> Result<Vec<u8>, EncodeError> {
    encode_long_packet(0xC2, 0xAE, &fixed_bytes::<257>(helper_data))
}

#[cfg(test)]
mod tests {
    use super::{mu_helper_save_data_request, mu_helper_status_change_request};
    use crate::wire::fixed_bytes;
    use proptest::prelude::*;

    #[test]
    fn encodes_mu_helper_status_packets() {
        assert_eq!(
            mu_helper_status_change_request(true).unwrap(),
            vec![0xC1, 0x05, 0xBF, 0x51, 0x01]
        );
    }

    proptest! {
        #[test]
        fn mu_helper_save_data_request_pads_and_truncates(helper_data in prop::collection::vec(any::<u8>(), 0..512)) {
            let packet = mu_helper_save_data_request(&helper_data).unwrap();
            assert_eq!(packet.len(), 261);
            assert_eq!(&packet[..4], &[0xC2, 0x01, 0x05, 0xAE]);
            assert_eq!(&packet[4..261], &fixed_bytes::<257>(&helper_data));
        }
    }
}
