use anyhow::{Context, Result};
use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine as _};
use openssl::encrypt::Decrypter;
use openssl::hash::MessageDigest;
use openssl::pkey::PKey;
use openssl::rsa::{Padding, Rsa};

pub struct DecrypterMachine<'a> {
    decrypted_key: Vec<u8>,
    encrypted_data: Vec<u8>,
    pub decrypted_data: Vec<u8>,
    full_file_name: &'a str,
}

impl<'a> DecrypterMachine<'a> {
    /// Create a new `Decrypter` instance.
    ///
    /// # Arguments
    ///
    /// * `encrypted_key` - The base64-encoded encrypted key for the file.
    /// * `encrypted_data` - The base64-encoded encrypted data from the database.
    /// * `file_extension` - The file extension of the decrypted file.
    /// * `file_name` - The name of the decrypted file.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `Decrypter` instance, or an error if either the
    /// `encrypted_key` or `encrypted_data` are invalid.
    pub fn new(
        encrypted_key: &'a str,
        encrypted_data: Option<&'a str>,
        full_file_name: &'a str,
        private: String,
    ) -> Result<Self> {
        let decrypted_key = Self::decrypt_file_key(encrypted_key, private)?;
        let encrypted_data = encrypted_data.map_or_else(
            || Vec::with_capacity(0),
            |data| Self::decode_encrypted_base64(data).unwrap(),
        );

        Ok(DecrypterMachine {
            decrypted_key,
            encrypted_data,
            decrypted_data: Vec::with_capacity(0),
            full_file_name,
        })
    }

    /// Decrypts an encrypted file key.
    ///
    /// This function takes an encrypted key as a base64-encoded string,
    /// decodes it, and then decrypts it using a private RSA key stored
    /// in a file named `.private`. The decrypted key is further processed
    /// by decoding it from base64 format to obtain the actual key.
    ///
    /// # Arguments
    ///
    /// * `encrypted_key` - A base64-encoded string representing the encrypted key.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of bytes representing the decrypted key,
    /// or an error if the process fails at any step.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The encrypted key cannot be decoded from base64.
    /// - The private key cannot be read from the `.private` file.
    /// - The RSA decryption process fails.
    /// - The final key cannot be decoded from base64.
    fn decrypt_file_key(encrypted_key: &str, private: String) -> Result<Vec<u8>> {
        // Decode the base64-encoded encrypted key
        let decoded_test_key = Self::decode_encrypted_base64(encrypted_key).map_err(|e| {
            anyhow::format_err!("Error decoding the encrypted key from base64: {}", e)
        })?;
        println!("Decoded test key length {}", decoded_test_key.len());

        // // TODO: Should get private key from state
        // let pem_key = Self::format_pem_key(&private)
        //     .map_err(|e| anyhow::format_err!("Error formatting the private key: {}", e))?;
        let pem_key = include_bytes!("../../.private");
        // dbg!("Private key length {}", pem_key.len());
        let rsa = Rsa::private_key_from_pem(pem_key)
            .map_err(|e| anyhow::format_err!("Error loading the private key from PEM: {}", e))?;
        let pkey = PKey::from_rsa(rsa.clone())
            .map_err(|e| anyhow::format_err!("Error converting RSA key to PKey: {}", e))?;

        // Initialize the decrypter with the private key
        let mut decrypter = Decrypter::new(&pkey).map_err(|e| {
            anyhow::format_err!(
                "Error initializing the decrypter with the private key: {}",
                e
            )
        })?;

        // Set the RSA padding and message digest
        decrypter
            .set_rsa_padding(Padding::PKCS1_OAEP)
            .map_err(|e| anyhow::format_err!("Error setting RSA padding to PKCS1_OAEP: {}", e))?;
        decrypter
            .set_rsa_oaep_md(MessageDigest::sha256())
            .map_err(|e| {
                anyhow::format_err!("Error setting RSA-OAEP message digest to SHA-256: {}", e)
            })?;

        // Decrypt the key
        let mut decrypted = vec![0; rsa.size() as usize];
        let decrypted_len = decrypter
            .decrypt(&decoded_test_key, &mut decrypted)
            .map_err(|e| anyhow::format_err!("Error decrypting the key: {}", e))?;
        decrypted.truncate(decrypted_len);

        println!("Decrypted key length {}", decrypted.len());
        Ok(decrypted)
    }

    /// Decode a base64-encoded string.
    ///
    /// # Arguments
    ///
    /// * `encrypted_data` - A base64-encoded string representing the encrypted data.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of bytes representing the decoded data,
    /// or an error if the decoding process fails.
    ///
    /// # Errors
    ///
    /// This function will return an error if the input string cannot be
    /// decoded from base64.
    fn decode_encrypted_base64(encrypted_data: &str) -> Result<Vec<u8>> {
        let encrypted_data_string = encrypted_data.as_bytes();
        BASE64_STANDARD
            .decode(encrypted_data_string)
            .map_err(|err| anyhow::format_err!("Unable to decode the string from BASE64: {}", err))
    }

    /// Decrypts a symmetrically encrypted file using AES-256-GCM.
    ///
    /// The function assumes that the first 12 bytes of the encrypted data contain
    /// the initialization vector (IV), followed by the encrypted content and a 16-byte
    /// authentication tag.
    ///
    /// # Returns
    ///
    /// A `Result` containing a new instance of `Decrypter` with the decrypted data,
    /// or an error if the decryption process fails.
    ///
    /// # Errors
    ///
    /// This function will return an error if the decryption process fails due to
    /// invalid key, IV, corrupted encrypted content, or authentication tag mismatch.
    pub fn decrypt_symetric_file(&self) -> Result<Self> {
        // First 16 bytes are the IV

        let iv = &self.encrypted_data[..12];
        let encrypted_content = &self.encrypted_data[12..];

        let cipher = openssl::symm::Cipher::aes_256_gcm();

        // In GCM there is an encryption tag that comes along with an authentication tag.
        // GCM is an AEAD cipher (Authenticated Encryption with Assoicated Data) have authentication tags.
        // The tag has a length of 16 and is appended at the end
        let tag_size = 16;
        let (encrypted_data, tag) = encrypted_content.split_at(encrypted_content.len() - tag_size);

        // We create a context
        let mut decrypt_context = openssl::symm::Crypter::new(
            cipher,
            openssl::symm::Mode::Decrypt,
            &self.decrypted_key,
            Some(iv),
        )?;

        // We set the tag
        decrypt_context.set_tag(tag)?;

        // We then extract the data from the tag and prepare a buffer to receive the decrypted data.
        // We then update the context so that it compares the encrypted_data to the decrypted data by feeding it through the decryption cipher.
        // The length of 'count' becomes the length of the result of 'feeding' the encrypted data to the update cipher and storing the intermediate decrypted data.
        // The 'rest' count comes from finalizing the decryption process and writing any remaining decrypted bytes.
        let mut decrypted_data = vec![0; encrypted_data.len()];
        let count = decrypt_context.update(encrypted_data, &mut decrypted_data)?;
        let rest = decrypt_context.finalize(&mut decrypted_data[count..])?;

        // The decrypted data is then truncated to the actual decrypted length, as the original encrypted_data length includes padding.
        decrypted_data.truncate(count + rest);

        Ok(DecrypterMachine {
            decrypted_key: self.decrypted_key.clone(),
            encrypted_data: Vec::default(),
            decrypted_data,
            full_file_name: self.full_file_name,
        })
    }

    /// Ensures the private key is properly formatted as PEM.
    fn format_pem_key(key: &str) -> Result<String> {
        const LINE_LENGTH: usize = 64;
        let header = "-----BEGIN PRIVATE KEY-----";
        let footer = "-----END PRIVATE KEY-----";

        // Reformat the body with proper line breaks
        let formatted_body = key
            .as_bytes()
            .chunks(LINE_LENGTH)
            .map(|chunk| {
                std::str::from_utf8(chunk).map_err(|e| anyhow::format_err!("UTF-8 error: {}", e))
            })
            .collect::<Result<Vec<_>>>()?
            .join("\n");

        let formatted_key = format!("{}\n{}\n{}", header, formatted_body, footer);

        // Save the formatted PEM key to a debug file
        let debug_file_path = "debug_formatted_pem_key.pem";
        let mut file = File::create(debug_file_path)
            .map_err(|e| anyhow::format_err!("Failed to create debug file: {:?}", e))?;
        file.write_all(formatted_key.as_bytes())
            .map_err(|e| anyhow::format_err!("Failed to write to debug file: {:?}", e))?;
        println!("Formatted PEM key saved to {}", debug_file_path);

        Ok(formatted_key)
    }
}

// #[cfg(test)]
// mod tests {

//     #[test]
//     fn can_decrypt_blob() {
//         let full_file_name = "test.txt";
//         let document_id = "581430de-6555-4d08-9487-09c93ab8bff6";

//         let encrypted_raw = "z6UIZ+F5DkRkgE6YndrM7glv+O3zO/luBsr/uRrF8k8=";
//         let encrypted_key = "BNIuDZHMYR43YPmkQg3HaPRAeDSXrmqfaNKl+p7vB44sVRowExg5OT9fQ1lNk4Gi7r2Kzk5oLJfOmzqt1BRmmmm7zI4jPUV9ng3FrCg23WZW+OBLywGi17YFmQW8CJUfmVz20yl5k82jrTBLLEqGAr/1b1krv0+UHr2dPsqiOKdreT9cVsLGUTJP2rw7ysxPH4WQEL+zzpA6LqIj4QXM+uvR6XSzyAwIpz6Zb7/t2IkulRWe1gnEXg+7hNnIlhmA5FQNjPliw1flcsEY0itWBb8cmT6fHa23jYmiaQ7AvCTG/IxohTFWgzIz7wMfyfD+ARf+dJpXqVXnkI0uY7tPSg==";
//         let test_data = "Hello!";

//         let decrypter = super::Decrypter::new(
//             encrypted_key,
//             Some(encrypted_raw),
//             full_file_name,
//         )
//         .unwrap();

//         let decrypted_blob = decrypter.decrypt_symetric_file().unwrap();
//         let decrypted_string = String::from_utf8(decrypted_blob.decrypted_data).unwrap();
//         assert_eq!(&decrypted_string, test_data)
//     }
// }
