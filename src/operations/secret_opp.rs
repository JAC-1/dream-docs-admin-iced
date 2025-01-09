use anyhow::Result;
use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine as _};
use openssl::rsa::{self, Rsa};

use std::{
    fs::{write, File},
    io::Read,
    path::Path,
};

pub struct Decrypter {
    decrypted_key: Vec<u8>,
    encrypted_data: Vec<u8>,
    decrypted_data: Vec<u8>,
    file_extension: String,
    file_name: String,
}

impl Decrypter {
    pub fn new(
        encrypted_key: String,
        encrypted_data: String,
        file_extension: String,
        file_name: String,
    ) -> Result<Self> {
        Ok(Decrypter {
            decrypted_key: Self::decrypt_file_key(encrypted_key)?,
            encrypted_data: Self::decode_encrypted_base64(encrypted_data)?,
            decrypted_data: Vec::default(),
            file_extension,
            file_name,
        })
    }

    fn decrypt_file_key(encrypted_key: String) -> Result<Vec<u8>> {
        let decoded_test_key = Self::decode_encrypted_base64(encrypted_key)?;

        let private_key = std::fs::read_to_string(".private")?;

        let rsa = Rsa::private_key_from_pem(private_key.as_bytes()).unwrap();
        let mut rsa_buffer = vec![0; rsa.size() as usize];
        let decrypted_test_key_length = rsa.private_decrypt(
            &decoded_test_key,
            rsa_buffer.as_mut_slice(),
            rsa::Padding::PKCS1_OAEP,
        )?;
        rsa_buffer.truncate(decrypted_test_key_length);
        Ok(rsa_buffer)
    }

    fn decode_encrypted_base64(encrypted_data: String) -> Result<Vec<u8>> {
        let encrypted_data_string = encrypted_data.as_bytes();
        BASE64_STANDARD
            .decode(encrypted_data_string)
            .map_err(|err| anyhow::format_err!("Unable to decode the string from BASE64: {}", err))
    }

    pub fn decrypt_symetric_file(&self) -> Result<Self> {
        // First 16 bytes are the IV
        let iv = &self.encrypted_data[..16];
        let encrypted_content = &self.encrypted_data[16..];

        let cipher = openssl::symm::Cipher::aes_256_cbc();
        let decrypted_data =
            openssl::symm::decrypt(cipher, &self.decrypted_key, Some(iv), encrypted_content)?;

        Ok(Decrypter {
            decrypted_key: self.decrypted_key.clone(),
            encrypted_data: Vec::default(),
            decrypted_data,
            file_extension: self.file_extension.clone(),
            file_name: self.file_name.clone(),
        })
    }

    pub fn write_decrypted(&self) -> Result<()> {
        //TODO: Construct path from file extension and file file_name
        //TODO: Remove from the actual struct and require it in the function call?
        let decrypted_path = self.path_to_encrypted_file.with_extension("");
        write(&decrypted_path, &self.decrypted_data).unwrap();
        println!("Decrypted file written.");
        Ok(())
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::{fs::remove_file, path::PathBuf};
//
//     #[test]
//     fn test_decrypt_file_key() {
//         let encrypted_key = "kq5BAdSKMl7swM0lkoy9ouert7JgvH4Mnt0TXnlSCHlZrVqZxymvucI2xnUb5bq4ECv2Pl+PZJuV45KJ6BleoRiR5K1iGjGvIrcADwLwlNNH+ArN8P69POGG9JEuG75QIpfspkcxmHXVn69s3J1QxwmpNK3C9BaQBML7lbf4/Ys54R75GZD+nuzD7Y74IGhsFbVtSrYvGxw3sUrKxYZEkmLFcesYDEme4MBnEzMOzdVgrGfNoeFiS24cIsgGf+G664B47+WdKnFREOfNjvHlWC4jBrU8rGnNtMZAGISF4WpLpG7c6r3Ot0H1gb1CT1F9f/kDvv/mHX2Iw6o9ThEI4w==";
//         let decrypted_key = decrypt_file_key(&encrypted_key).unwrap();
//
//         assert_eq!(
//             "k0nKWBtTfbr4fe8ay8WZq06zyi9BcSwQoHlK0d9MHQU=",
//             String::from_utf8_lossy(&decrypted_key)
//         );
//     }
//
//     #[test]
//     fn can_generate_rsa_key_pair() {
//         let (public_key, private_key) = super::generate_rsa_key_pair().unwrap();
//         assert!(
//             public_key.starts_with("-----BEGIN PUBLIC KEY-----"),
//             "Public key does not contain proper begin header."
//         );
//         assert!(
//             private_key.starts_with("-----BEGIN RSA PRIVATE KEY-----"),
//             "Private key does not contain proper begin header."
//         );
//     }
//
//     fn decrypt_and_write_file(encrypted_path: &Path, key_str: &str) -> (Vec<u8>, PathBuf) {
//         let key = BASE64_STANDARD.decode(key_str).unwrap();
//         let decrypted_content = decrypt_symetric_file(encrypted_path, key).unwrap();
//         let decrypted_path = encrypted_path.with_extension("");
//         write(&decrypted_path, &decrypted_content).unwrap();
//         (decrypted_content, decrypted_path.to_path_buf())
//     }
//
//     fn verify_png_file(path: &Path) {
//         let decoder = png::Decoder::new(std::fs::File::open(path).unwrap());
//         let reader = decoder.read_info().expect("Should be valid PNG file");
//         assert!(
//             reader.info().width > 0 && reader.info().height > 0,
//             "Should have valid dimensions"
//         );
//     }
//
//     #[test]
//     fn can_decrypt_text_based_symetric_file() {
//         let encrypted_path = Path::new("test.txt.enc");
//         let key = "h3tV3hdMgipp98uu9QhGwYhlbkaV00q6ln3iAh1GX60=";
//         let (decrypted_content, decrypted_path) = decrypt_and_write_file(encrypted_path, key);
//
//         assert_eq!(decrypted_content, b"Hello!");
//         assert_eq!(std::fs::read_to_string(&decrypted_path).unwrap(), "Hello!");
//
//         remove_file(decrypted_path).unwrap();
//     }
//
//     #[test]
//     fn can_decrypt_image_based_symetric_file() {
//         let encrypted_path = Path::new("test.png.enc");
//         let key = "j/HldO0y9Jp+rGxIDAgGjxwhHn34ja+r4PaGXxlf9Kk=";
//         let (_, decrypted_path) = decrypt_and_write_file(encrypted_path, key);
//
//         verify_png_file(&decrypted_path);
//         remove_file(decrypted_path).unwrap();
//     }
// }
