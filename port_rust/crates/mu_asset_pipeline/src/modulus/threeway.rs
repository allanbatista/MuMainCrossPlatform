const THREE_WAY_KEY_WORDS: usize = 3;
const THREE_WAY_ROUNDS: usize = 11;
const THREE_WAY_START_D: u32 = 0xB1B1;

pub(crate) struct ThreeWayCipher {
    key: [u32; THREE_WAY_KEY_WORDS],
}

impl ThreeWayCipher {
    pub(crate) fn new(key: &[u8]) -> Self {
        let mut key_words = [0u32; THREE_WAY_KEY_WORDS];
        for index in 0..THREE_WAY_KEY_WORDS {
            let offset = index * 4;
            key_words[index] = u32::from_le_bytes(key[offset..offset + 4].try_into().unwrap());
        }

        let (a0, a1, a2) = theta(key_words[0], key_words[1], key_words[2]);
        let (a0, a1, a2) = mu(a0, a1, a2);

        Self {
            key: [a2.swap_bytes(), a1.swap_bytes(), a0.swap_bytes()],
        }
    }

    pub(crate) const fn block_size(&self) -> usize {
        12
    }

    pub(crate) fn decrypt_block(&self, block: &mut [u8]) {
        let a0 = u32::from_le_bytes(block[0..4].try_into().unwrap());
        let a1 = u32::from_le_bytes(block[4..8].try_into().unwrap());
        let a2 = u32::from_le_bytes(block[8..12].try_into().unwrap());
        let mut rc = THREE_WAY_START_D;

        let (a0, a1, a2) = mu(a0, a1, a2);
        let mut a0 = a0;
        let mut a1 = a1;
        let mut a2 = a2;

        for _ in 0..THREE_WAY_ROUNDS {
            a0 ^= self.key[0] ^ (rc << 16);
            a1 ^= self.key[1];
            a2 ^= self.key[2] ^ rc;
            let (b0, b1, b2) = rho(a0, a1, a2);
            a0 = b0;
            a1 = b1;
            a2 = b2;

            rc <<= 1;
            if rc & 0x1_0000 != 0 {
                rc ^= 0x1_1011;
            }
            rc &= 0xFFFF;
        }

        a0 ^= self.key[0] ^ (rc << 16);
        a1 ^= self.key[1];
        a2 ^= self.key[2] ^ rc;

        let (a0, a1, a2) = theta(a0, a1, a2);
        let (a0, a1, a2) = mu(a0, a1, a2);

        block[0..4].copy_from_slice(&a0.to_le_bytes());
        block[4..8].copy_from_slice(&a1.to_le_bytes());
        block[8..12].copy_from_slice(&a2.to_le_bytes());
    }
}

fn theta(a0: u32, a1: u32, a2: u32) -> (u32, u32, u32) {
    let c0 = a0 ^ a1 ^ a2;
    let c = c0.rotate_left(16) ^ c0.rotate_left(8);
    let b0 = (a0 << 24) ^ (a2 >> 8) ^ (a1 << 8) ^ (a0 >> 24);
    let b1 = (a1 << 24) ^ (a0 >> 8) ^ (a2 << 8) ^ (a1 >> 24);

    (a0 ^ c ^ b0, a1 ^ c ^ b1, a2 ^ c ^ ((b0 >> 16) ^ (b1 << 16)))
}

fn mu(a0: u32, a1: u32, a2: u32) -> (u32, u32, u32) {
    (a2.reverse_bits(), a1.reverse_bits(), a0.reverse_bits())
}

fn pi_gamma_pi(a0: u32, a1: u32, a2: u32) -> (u32, u32, u32) {
    let b2 = a2.rotate_left(1);
    let b0 = a0.rotate_left(22);

    (
        (b0 ^ (a1 | !b2)).rotate_left(1),
        a1 ^ (b2 | !b0),
        (b2 ^ (b0 | !a1)).rotate_left(22),
    )
}

fn rho(a0: u32, a1: u32, a2: u32) -> (u32, u32, u32) {
    let (a0, a1, a2) = theta(a0, a1, a2);
    pi_gamma_pi(a0, a1, a2)
}

#[cfg(test)]
mod tests {
    use super::ThreeWayCipher;

    #[test]
    fn decrypts_known_vector() {
        let key = [0u8; 12];
        let mut block = [
            0x6E, 0xC7, 0x59, 0x40, 0xC4, 0x9D, 0xAE, 0x83, 0xF7, 0xEC, 0x21, 0xAD,
        ];
        let cipher = ThreeWayCipher::new(&key);

        cipher.decrypt_block(&mut block);

        assert_eq!(
            block,
            [0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00]
        );
    }
}
