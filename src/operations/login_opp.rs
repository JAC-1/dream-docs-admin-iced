use ring::aead::{Aad, LessSafeKey, Nonce, UnboundKey, AES_256_GCM};
use ring::pbkdf2;
use std::collections::HashMap;
use std::num::NonZeroU32;

pub struct LoginAuth {
    pub raw_password: String,
    itterations: u32,
    pub enc_envs: Vec<Vec<u8>>,
}

impl LoginAuth {
    pub fn new(raw_password: String, enc_envs: Vec<Vec<u8>>) -> Self {
        println!("Creating LoginAuth");
        LoginAuth {
            raw_password,
            itterations: 100_000,
            enc_envs,
        }
    }

    fn derive_key(&self, salt: [u8; 16]) -> [u8; 32] {
        let mut key = [0u8; 32];
        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA256,
            NonZeroU32::new(self.itterations).unwrap(),
            &salt,
            self.raw_password.as_bytes(),
            &mut key,
        );
        key
    }

    // Function that tries all .envs in the env_paths and returns the first one that can be
    // decrypted
    pub fn try_decrypt_env(&self) -> Result<Vec<u8>, String> {
        const SALT_LEN: usize = 16;
        const NONCE_LEN: usize = 12;
        for enc_env in self.enc_envs.iter() {
            let (salt, rest) = enc_env.split_at(SALT_LEN);
            let (nonce, ciphertext) = rest.split_at(NONCE_LEN);

            // TODO: Handle unwrap
            let key_bytes = self.derive_key(
                salt.try_into()
                    .map_err(|e| format!("Salt conversion error: {:?}", e))?,
            );
            let key = UnboundKey::new(&AES_256_GCM, &key_bytes)
                .map_err(|e| format!("Key creation error: {:?}", e))?;
            let key = LessSafeKey::new(key);

            match key.open_in_place(
                Nonce::assume_unique_for_key(
                    nonce
                        .try_into()
                        .map_err(|e| format!("Nonce error: {:?}", e))?,
                ),
                Aad::empty(),
                &mut ciphertext.to_vec(),
            ) {
                Ok(plain_text) => {
                    println!("Decryption successful for env file.");
                    return Ok(plain_text.to_vec());
                }
                Err(_) => continue,
            }
        }
        Err("Decryption failed for all env files".to_string())
    }

    pub fn parse_plain_text_to_hashmap(
        &self,
        plain_text: Vec<u8>,
    ) -> Result<HashMap<String, String>, String> {
        let plain_text =
            String::from_utf8(plain_text).map_err(|e| format!("UTF-8 error: {:?}", e))?;
        let mut env_map = HashMap::new();

        for line in plain_text.lines() {
            if line.is_empty() {
                continue;
            }
            let mut split = line.split("=");
            let key = split
                .next()
                .ok_or_else(|| "Missing key in line".to_string())?;
            let value = split
                .next()
                .ok_or_else(|| "Missing value in line".to_string())?;
            env_map.insert(key.to_string(), value.to_string());
        }
        Ok(env_map)
        // Ok(plain_text)
    }
}
