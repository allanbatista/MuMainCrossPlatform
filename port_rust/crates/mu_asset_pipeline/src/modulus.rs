mod mars;
mod rc5;
mod threeway;

use cast5::cipher::{Array as Cast5Block, BlockCipherDecrypt as _, KeyInit as _};
use cast5::Cast5;
use cipher::consts::U32;
use gost_crypto::{Gost28147, SBOX_CRYPTOPRO};
use idea::cipher::Block as IdeaBlock;
use idea::Idea;
use rc6_rs::Rc6;
use tea_soft::block_cipher::generic_array::GenericArray as TeaBlock;
use tea_soft::block_cipher::{BlockCipher as _, NewBlockCipher as _};
use tea_soft::Tea32;

use self::mars::MarsCipher;
use self::rc5::Rc5Cipher;
use self::threeway::ThreeWayCipher;

const MODULUS_MAGIC_SIZE: usize = 4;
const MODULUS_HEADER_SIZE: usize = 34;
const MODULUS_STAGE1_WINDOW: usize = 1024;
const MODULUS_KEY_1: &[u8; 32] = b"webzen#@!01webzen#@!01webzen#@!0";
const RC5_ROUNDS: usize = 16;
const RC6_ROUNDS: usize = 20;
const TEA_KEY_SIZE: usize = 16;
const THREE_WAY_KEY_SIZE: usize = 12;
const CAST5_KEY_SIZE: usize = 16;
const IDEA_KEY_SIZE: usize = 16;
const MARS_KEY_SIZE: usize = 16;
const GOST_KEY_SIZE: usize = 32;

enum ModulusCipher {
    Tea(Tea32),
    ThreeWay(ThreeWayCipher),
    Cast5(Cast5),
    Rc5(Rc5Cipher),
    Rc6(Rc6<U32>),
    Mars(MarsCipher),
    Idea(Idea),
    Gost(Box<Gost28147>),
}

impl ModulusCipher {
    fn new(algorithm: u8, key: &[u8]) -> Result<Self, String> {
        match algorithm & 7 {
            0 => {
                if key.len() < TEA_KEY_SIZE {
                    return Err(format!(
                        "TEA key too short: {} < {}",
                        key.len(),
                        TEA_KEY_SIZE
                    ));
                }
                let mut key_block = TeaBlock::default();
                key_block.copy_from_slice(&key[..TEA_KEY_SIZE]);
                Ok(Self::Tea(Tea32::new(&key_block)))
            }
            1 => {
                if key.len() < THREE_WAY_KEY_SIZE {
                    return Err(format!(
                        "ThreeWay key too short: {} < {}",
                        key.len(),
                        THREE_WAY_KEY_SIZE
                    ));
                }
                Ok(Self::ThreeWay(ThreeWayCipher::new(
                    &key[..THREE_WAY_KEY_SIZE],
                )))
            }
            2 => {
                if key.len() < CAST5_KEY_SIZE {
                    return Err(format!(
                        "CAST5 key too short: {} < {}",
                        key.len(),
                        CAST5_KEY_SIZE
                    ));
                }
                let cipher = Cast5::new_from_slice(&key[..CAST5_KEY_SIZE])
                    .map_err(|error| error.to_string())?;
                Ok(Self::Cast5(cipher))
            }
            3 => {
                if key.len() < TEA_KEY_SIZE {
                    return Err(format!(
                        "RC5 key too short: {} < {}",
                        key.len(),
                        TEA_KEY_SIZE
                    ));
                }
                Ok(Self::Rc5(Rc5Cipher::new(&key[..TEA_KEY_SIZE], RC5_ROUNDS)))
            }
            4 => {
                if key.len() < TEA_KEY_SIZE {
                    return Err(format!(
                        "RC6 key too short: {} < {}",
                        key.len(),
                        TEA_KEY_SIZE
                    ));
                }
                Ok(Self::Rc6(Rc6::<U32>::new(&key[..TEA_KEY_SIZE], RC6_ROUNDS)))
            }
            5 => {
                if key.len() < MARS_KEY_SIZE {
                    return Err(format!(
                        "MARS key too short: {} < {}",
                        key.len(),
                        MARS_KEY_SIZE
                    ));
                }
                Ok(Self::Mars(MarsCipher::new(&key[..MARS_KEY_SIZE])))
            }
            6 => {
                if key.len() < IDEA_KEY_SIZE {
                    return Err(format!(
                        "IDEA key too short: {} < {}",
                        key.len(),
                        IDEA_KEY_SIZE
                    ));
                }
                let cipher = Idea::new_from_slice(&key[..IDEA_KEY_SIZE])
                    .map_err(|error| error.to_string())?;
                Ok(Self::Idea(cipher))
            }
            7 => {
                if key.len() < GOST_KEY_SIZE {
                    return Err(format!(
                        "GOST key too short: {} < {}",
                        key.len(),
                        GOST_KEY_SIZE
                    ));
                }
                let key = key[..GOST_KEY_SIZE]
                    .try_into()
                    .map_err(|_| "failed to build GOST key".to_string())?;
                Ok(Self::Gost(Box::new(Gost28147::with_sbox(
                    &key,
                    &SBOX_CRYPTOPRO,
                ))))
            }
            other => Err(format!("unsupported ModulusCryptor algorithm: {other}")),
        }
    }

    fn block_size(&self) -> usize {
        match self {
            Self::Tea(_) => 8,
            Self::ThreeWay(cipher) => cipher.block_size(),
            Self::Cast5(_) => 8,
            Self::Rc5(cipher) => cipher.block_size(),
            Self::Rc6(_) => 16,
            Self::Mars(cipher) => cipher.block_size(),
            Self::Idea(_) => 8,
            Self::Gost(_) => 8,
        }
    }

    fn decrypt_block(&self, block: &mut [u8]) -> Result<(), String> {
        match self {
            Self::Tea(cipher) => {
                let mut buffer = TeaBlock::default();
                buffer.copy_from_slice(block);
                cipher.decrypt_block(&mut buffer);
                block.copy_from_slice(buffer.as_slice());
                Ok(())
            }
            Self::ThreeWay(cipher) => {
                cipher.decrypt_block(block);
                Ok(())
            }
            Self::Cast5(cipher) => {
                let mut buffer = Cast5Block::default();
                buffer.copy_from_slice(block);
                cipher.decrypt_block(&mut buffer);
                block.copy_from_slice(buffer.as_slice());
                Ok(())
            }
            Self::Rc5(cipher) => {
                cipher.decrypt_block(block);
                Ok(())
            }
            Self::Rc6(cipher) => {
                let decrypted = cipher.decrypt(block).map_err(|error| error.to_string())?;
                block.copy_from_slice(&decrypted);
                Ok(())
            }
            Self::Mars(cipher) => {
                cipher.decrypt_block(block);
                Ok(())
            }
            Self::Idea(cipher) => {
                let mut buffer = IdeaBlock::<Idea>::default();
                buffer.copy_from_slice(block);
                cipher.decrypt_block(&mut buffer);
                block.copy_from_slice(buffer.as_slice());
                Ok(())
            }
            Self::Gost(cipher) => {
                let buffer: [u8; 8] = block
                    .try_into()
                    .map_err(|_| "invalid GOST block size".to_string())?;
                let decrypted = cipher.decrypt_block_raw(buffer);
                block.copy_from_slice(&decrypted);
                Ok(())
            }
        }
    }

    fn decrypt_in_place(&self, data: &mut [u8]) -> Result<(), String> {
        let block_size = self.block_size();
        if !data.len().is_multiple_of(block_size) {
            return Err(format!(
                "ciphertext length {} is not a multiple of block size {}",
                data.len(),
                block_size
            ));
        }

        for block in data.chunks_exact_mut(block_size) {
            self.decrypt_block(block)?;
        }
        Ok(())
    }
}

pub(crate) fn decrypt_modulus_payload(raw: &[u8]) -> Result<Vec<u8>, String> {
    if raw.len() < MODULUS_MAGIC_SIZE + MODULUS_HEADER_SIZE {
        return Err(format!(
            "ModulusCryptor terrain payload too short: {} < {}",
            raw.len(),
            MODULUS_MAGIC_SIZE + MODULUS_HEADER_SIZE
        ));
    }

    let mut body = raw[MODULUS_MAGIC_SIZE..].to_vec();
    let data_size = body.len() - MODULUS_HEADER_SIZE;
    let algorithm1 = body[1];
    let algorithm2 = body[0];

    let cipher1 = ModulusCipher::new(algorithm1, MODULUS_KEY_1)?;
    let stage1_window = stage1_window(cipher1.block_size());

    if data_size > 4 * stage1_window {
        let start = 2 + (data_size >> 1);
        let end = start + stage1_window;
        cipher1.decrypt_in_place(&mut body[start..end])?;
    }

    if data_size > stage1_window {
        let tail_start = body.len() - stage1_window;
        cipher1.decrypt_in_place(&mut body[tail_start..])?;
        cipher1.decrypt_in_place(&mut body[2..2 + stage1_window])?;
    }

    let key2 = body[2..34].to_vec();
    let cipher2 = ModulusCipher::new(algorithm2, &key2)?;
    let decrypt_size = data_size - (data_size % cipher2.block_size());
    cipher2.decrypt_in_place(&mut body[34..34 + decrypt_size])?;

    Ok(body.split_off(MODULUS_HEADER_SIZE))
}

fn stage1_window(block_size: usize) -> usize {
    MODULUS_STAGE1_WINDOW - (MODULUS_STAGE1_WINDOW % block_size)
}

#[cfg(test)]
mod tests {
    use super::decrypt_modulus_payload;
    use crate::modulus::rc5::Rc5Cipher;
    use crate::terrain::apply_bux_convert;

    fn build_modulus_payload(
        magic: &[u8; 4],
        algorithm2: u8,
        algorithm1: u8,
        key2: &[u8],
        data: &[u8],
    ) -> Vec<u8> {
        let mut raw = Vec::with_capacity(4 + 34 + data.len());
        raw.extend_from_slice(magic);
        raw.push(algorithm2);
        raw.push(algorithm1);
        raw.extend_from_slice(key2);
        raw.extend_from_slice(data);
        raw
    }

    #[test]
    fn decrypts_modulus_map_payload() {
        let key2 = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
            0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B,
            0x1C, 0x1D, 0x1E, 0x1F,
        ];
        let plaintext = [0x96, 0x95, 0x0D, 0xDA, 0x65, 0x4A, 0x3D, 0x62];
        let cipher = Rc5Cipher::new(&key2[..16], 16);
        let mut ciphertext = plaintext;
        cipher.encrypt_block(&mut ciphertext);

        let raw = build_modulus_payload(b"MAP\x01", 3, 0, &key2, &ciphertext);
        let decoded = decrypt_modulus_payload(&raw).unwrap();

        assert_eq!(decoded, plaintext);
    }

    #[test]
    fn decrypts_modulus_att_payload_then_applies_bux() {
        let key2 = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
            0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B,
            0x1C, 0x1D, 0x1E, 0x1F,
        ];
        let plaintext = [0x96, 0x95, 0x0D, 0xDA, 0x65, 0x4A, 0x3D, 0x62];
        let cipher = Rc5Cipher::new(&key2[..16], 16);
        let mut ciphertext = plaintext;
        cipher.encrypt_block(&mut ciphertext);

        let raw = build_modulus_payload(b"ATT\x01", 3, 0, &key2, &ciphertext);
        let decoded = apply_bux_convert(&decrypt_modulus_payload(&raw).unwrap());

        assert_eq!(decoded, [0x6A, 0x5A, 0xA6, 0x26, 0xAA, 0xE1, 0xC1, 0xAD]);
    }
}
