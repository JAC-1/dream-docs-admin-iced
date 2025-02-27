use ring::aead::{Aad, LessSafeKey, Nonce, UnboundKey, AES_256_GCM};
use ring::pbkdf2;
use std::num::NonZeroU32;

pub struct LoginAuth {
    pub raw_password: String,
    pub salt: String,
    nonce_len: usize,
    itterations: u32,
    pub enc_envs: Vec<Vec<u8>>,
}

impl LoginAuth {
    pub fn new(raw_password: String, salt: String, enc_envs: Vec<Vec<u8>>) -> Self {
        println!("Creating LoginAuth");
        LoginAuth {
            raw_password,
            salt,
            nonce_len: 12,
            itterations: 100_000,
            enc_envs,
        }
    }

    pub fn derive_key(&self) -> [u8; 32] {
        let mut key = [0u8; 32];
        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA256,
            NonZeroU32::new(self.itterations).unwrap(),
            self.salt.as_bytes(),
            self.raw_password.as_bytes(),
            &mut key,
        );
        key
    }

    // Function that tries all .envs in the env_paths and returns the first one that can be
    // decrypted
    pub fn try_decrypt_env(&self) -> Result<Vec<u8>, String> {
        let key_bytes = self.derive_key();
        let key =
            UnboundKey::new(&AES_256_GCM, &key_bytes).map_err(|e| format!("Key error: {:?}", e))?;
        let key = LessSafeKey::new(key);
        for (index, enc_env) in self.enc_envs.iter().enumerate() {
            let (nonce, ciphertext) = enc_env.split_at(self.nonce_len);
            println!(
                "Trying to decrypt env file {}: nonece: {:?}, ciphertext: {:?}",
                index, nonce, ciphertext
            );
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
                    println!("Decryption successful for env file {}", index);
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
    ) -> Result<std::collections::HashMap<String, String>, String> {
        let plain_text =
            String::from_utf8(plain_text).map_err(|e| format!("UTF-8 error: {:?}", e))?;
        let mut env_map = std::collections::HashMap::new();

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
    }
}
