use anyhow::Result;
use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine as _};
use openssl::rsa::{self, Rsa};

use std::{
    io::Read,
};

pub struct Decrypter<'a> {
    decrypted_key: Vec<u8>,
    encrypted_data: Vec<u8>,
    pub decrypted_data: Vec<u8>,
    full_file_name: &'a str,
}

impl<'a> Decrypter<'a> {
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
    ) -> Result<Self> {
        let decrypted_key = Self::decrypt_file_key(encrypted_key)?;
        let encrypted_data = encrypted_data.map_or_else(
            || Vec::with_capacity(0),
            |data| Self::decode_encrypted_base64(data).unwrap(),
        );

        Ok(Decrypter {
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
    fn decrypt_file_key(encrypted_key: &str) -> Result<Vec<u8>> {
        let decoded_test_key = Self::decode_encrypted_base64(encrypted_key)?;

        println!("Decoded Test Key Length: {}", decoded_test_key.len());

        let private_key = std::fs::read_to_string(".private")?;

        let rsa = Rsa::private_key_from_pem(private_key.as_bytes()).unwrap();
        let mut rsa_buffer = vec![0; rsa.size() as usize];
        let decrypted_test_key_length = rsa.private_decrypt(
            &decoded_test_key,
            rsa_buffer.as_mut_slice(),
            rsa::Padding::PKCS1_OAEP,
        )?;

        println!("Decrypted Test Key Length: {}", decrypted_test_key_length);
        rsa_buffer.truncate(decrypted_test_key_length);
        let actual_key = BASE64_STANDARD.decode(&rsa_buffer)?;
        println!("Final Key Length: {}", actual_key.len());
        Ok(actual_key)
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

    /// Decrypts a symmetrically encrypted file using AES-256-CBC.
    ///
    /// The function assumes that the first 16 bytes of the encrypted data contain
    /// the initialization vector (IV), and the rest is the encrypted content.
    ///
    /// # Returns
    ///
    /// A `Result` containing a new instance of `Decrypter` with the decrypted data,
    /// or an error if the decryption process fails.
    ///
    /// # Errors
    ///
    /// This function will return an error if the decryption process fails due to
    /// invalid key, IV, or corrupted encrypted content.
    pub fn decrypt_symetric_file(&self) -> Result<Self> {
        // First 16 bytes are the IV

        let iv = &self.encrypted_data[..16];
        let encrypted_content = &self.encrypted_data[16..];

        println!("IV Length: {}", iv.len());
        println!("Encrypted Content Length: {}", encrypted_content.len());

        let cipher = openssl::symm::Cipher::aes_256_cbc();
        let decrypted_data =
            openssl::symm::decrypt(cipher, &self.decrypted_key, Some(iv), encrypted_content)?;

        Ok(Decrypter {
            decrypted_key: self.decrypted_key.clone(),
            encrypted_data: Vec::default(),
            decrypted_data,
            full_file_name: self.full_file_name,
        })
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn can_decrypt_blob() {
        let full_file_name = "test.txt";
        let document_id = "581430de-6555-4d08-9487-09c93ab8bff6";

        let encrypted_raw = "z6UIZ+F5DkRkgE6YndrM7glv+O3zO/luBsr/uRrF8k8=";
        let encrypted_key = "BNIuDZHMYR43YPmkQg3HaPRAeDSXrmqfaNKl+p7vB44sVRowExg5OT9fQ1lNk4Gi7r2Kzk5oLJfOmzqt1BRmmmm7zI4jPUV9ng3FrCg23WZW+OBLywGi17YFmQW8CJUfmVz20yl5k82jrTBLLEqGAr/1b1krv0+UHr2dPsqiOKdreT9cVsLGUTJP2rw7ysxPH4WQEL+zzpA6LqIj4QXM+uvR6XSzyAwIpz6Zb7/t2IkulRWe1gnEXg+7hNnIlhmA5FQNjPliw1flcsEY0itWBb8cmT6fHa23jYmiaQ7AvCTG/IxohTFWgzIz7wMfyfD+ARf+dJpXqVXnkI0uY7tPSg==";
        let test_data = "Hello!";

        let decrypter = super::Decrypter::new(
            encrypted_key,
            Some(encrypted_raw),
            full_file_name,
        )
        .unwrap();

        let decrypted_blob = decrypter.decrypt_symetric_file().unwrap();
        let decrypted_string = String::from_utf8(decrypted_blob.decrypted_data).unwrap();
        println!("{}", decrypted_string);
        assert_eq!(&decrypted_string, test_data)
    }
}
