use std::cmp::max;

const RC5_P32: u32 = 0xB7E1_5163;
const RC5_Q32: u32 = 0x9E37_79B9;

pub(crate) struct Rc5Cipher {
    schedule: Vec<u32>,
    rounds: usize,
}

impl Rc5Cipher {
    pub(crate) fn new(key: &[u8], rounds: usize) -> Self {
        Self {
            schedule: expand_key(key, rounds),
            rounds,
        }
    }

    pub(crate) const fn block_size(&self) -> usize {
        8
    }

    #[cfg(test)]
    pub(crate) fn encrypt_block(&self, block: &mut [u8]) {
        let mut a = u32::from_le_bytes(block[0..4].try_into().unwrap());
        let mut b = u32::from_le_bytes(block[4..8].try_into().unwrap());

        a = a.wrapping_add(self.schedule[0]);
        b = b.wrapping_add(self.schedule[1]);

        for round in 1..=self.rounds {
            a = (a ^ b)
                .rotate_left(b & 31)
                .wrapping_add(self.schedule[2 * round]);
            b = (b ^ a)
                .rotate_left(a & 31)
                .wrapping_add(self.schedule[2 * round + 1]);
        }

        block[0..4].copy_from_slice(&a.to_le_bytes());
        block[4..8].copy_from_slice(&b.to_le_bytes());
    }

    pub(crate) fn decrypt_block(&self, block: &mut [u8]) {
        let mut a = u32::from_le_bytes(block[0..4].try_into().unwrap());
        let mut b = u32::from_le_bytes(block[4..8].try_into().unwrap());

        for round in (1..=self.rounds).rev() {
            b = b
                .wrapping_sub(self.schedule[2 * round + 1])
                .rotate_right(a & 31)
                ^ a;
            a = a
                .wrapping_sub(self.schedule[2 * round])
                .rotate_right(b & 31)
                ^ b;
        }

        a = a.wrapping_sub(self.schedule[0]);
        b = b.wrapping_sub(self.schedule[1]);

        block[0..4].copy_from_slice(&a.to_le_bytes());
        block[4..8].copy_from_slice(&b.to_le_bytes());
    }
}

fn expand_key(key: &[u8], rounds: usize) -> Vec<u32> {
    let schedule_len = 2 * (rounds + 1);
    let word_count = max(1, (key.len() * 8).div_ceil(32));

    let mut key_words = vec![0u32; word_count];
    for index in (0..key.len()).rev() {
        let word_index = index / 4;
        key_words[word_index] = (key_words[word_index] << 8) + u32::from(key[index]);
    }

    let mut schedule = vec![0u32; schedule_len];
    schedule[0] = RC5_P32;
    for index in 1..schedule_len {
        schedule[index] = schedule[index - 1].wrapping_add(RC5_Q32);
    }

    let mut a = 0u32;
    let mut b = 0u32;
    let mut schedule_index = 0usize;
    let mut key_index = 0usize;
    let mix_iterations = 3 * max(word_count, schedule_len);

    for _ in 0..mix_iterations {
        a = schedule[schedule_index]
            .wrapping_add(a)
            .wrapping_add(b)
            .rotate_left(3);
        schedule[schedule_index] = a;

        b = key_words[key_index]
            .wrapping_add(a)
            .wrapping_add(b)
            .rotate_left((a.wrapping_add(b)) & 31);
        key_words[key_index] = b;

        schedule_index = (schedule_index + 1) % schedule_len;
        key_index = (key_index + 1) % word_count;
    }

    schedule
}

#[cfg(test)]
mod tests {
    use super::Rc5Cipher;

    #[test]
    fn encrypts_and_decrypts_known_vector() {
        let key = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
            0x0E, 0x0F,
        ];
        let block = [
            0x96, 0x95, 0x0D, 0xDA, 0x65, 0x4A, 0x3D, 0x62,
        ];
        let cipher = Rc5Cipher::new(&key, 12);

        let mut encrypted = block;
        cipher.encrypt_block(&mut encrypted);
        assert_eq!(
            encrypted,
            [0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77]
        );

        cipher.decrypt_block(&mut encrypted);
        assert_eq!(encrypted, block);
    }
}
