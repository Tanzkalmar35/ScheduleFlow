use std::num::NonZeroU32;

use base64::{engine::general_purpose::STANDARD, Engine};
use ed25519_dalek::SigningKey;
use ring::{
    aead, pbkdf2,
    rand::{SecureRandom, SystemRandom},
};

/// A service for encrypting and decrypting keys.
pub struct CryptoService;

/// Implementation of the CryptoService.
impl CryptoService {
    /// Encrypts a key using a passphrase.
    ///
    /// # Arguments
    ///
    /// * `signing_key` - The private key to encrypt.
    /// * `passphrase` - The passphrase to encrypt the key with.
    ///
    /// # Returns
    ///
    /// The encrypted private key as a base64 encoded string.
    ///
    /// # Fails
    ///
    /// If the encryption process fails.
    /// TODO: Add more specific error handling.
    fn encrypt_private_key(
        signing_key: &SigningKey,
        passphrase: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Get the raw bytes of the private key
        let private_key_bytes = signing_key.to_bytes();

        // Generate a random salt
        let mut salt = [0u8; 16];
        if let Err(e) = SystemRandom::new().fill(&mut salt) {
            panic!("Could not generate a random salt for key encryption: {}", e);
        }

        // Derive an encryption key
        let mut derived_key = [0u8; 32];
        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA256,
            NonZeroU32::new(100_000).unwrap(),
            &salt,
            passphrase.as_bytes(),
            &mut derived_key,
        );

        // Generate a random nonce
        let mut nonce = [0u8; 12];
        if let Err(e) = SystemRandom::new().fill(&mut nonce) {
            panic!(
                "Could not generate a random nonce for key encryption: {}",
                e
            );
        }

        // Create AEAD encryption key
        let aead_key = aead::LessSafeKey::new(
            aead::UnboundKey::new(&aead::AES_256_GCM, &derived_key)
                .expect("Woah don't know man (sorry)..."),
        );

        // Encrypt the private key
        let mut in_out = private_key_bytes.to_vec();
        aead_key
            .seal_in_place_append_tag(
                aead::Nonce::assume_unique_for_key(nonce),
                aead::Aad::empty(),
                &mut in_out,
            )
            .expect("Woah don't know man (sorry)...");

        // Combine salt, nonce, and encrypted key
        let mut encrypted_data = Vec::new();
        encrypted_data.extend_from_slice(&salt);
        encrypted_data.extend_from_slice(&nonce);
        encrypted_data.extend_from_slice(&in_out);

        // Base64 encode the entire package
        Ok(STANDARD.encode(encrypted_data))
    }

    /// Decrypts a key using a passphrase.
    ///
    /// # Arguments
    ///
    /// * `encrypted_data` - The encrypted private key as a base64 encoded string.
    /// * `passphrase` - The passphrase to decrypt the key with.
    ///
    /// # Returns
    ///
    /// The decrypted private key.
    ///
    /// # Fails
    ///
    /// If the decryption process fails.
    /// TODO: Add more specific error handling.
    fn decrypt_private_key(
        encrypted_data: &str,
        passphrase: &str,
    ) -> Result<SigningKey, Box<dyn std::error::Error>> {
        // Base64 decode the encrypted data
        let encrypted_bytes = STANDARD.decode(encrypted_data)?;

        // Extract salt, nonce, and encrypted key
        let salt = &encrypted_bytes[0..16]; // 16 bytes for salt
        let nonce = &encrypted_bytes[16..28]; // 12 bytes for nonce
        let encrypted_key = &encrypted_bytes[28..]; // Remaining bytes for encrypted key + tag

        // Derive the encryption key
        let mut derived_key = [0u8; 32];
        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA256,
            NonZeroU32::new(100_000).unwrap(),
            salt,
            passphrase.as_bytes(),
            &mut derived_key,
        );

        // Create AEAD decryption key
        let aead_key = aead::LessSafeKey::new(
            aead::UnboundKey::new(&aead::AES_256_GCM, &derived_key)
                .expect("Failed to create AEAD key"),
        );

        // Prepare a buffer for the decrypted key (48 bytes)
        let mut decrypted_key = vec![0u8; encrypted_key.len()]; // This should be 48 bytes
        decrypted_key.copy_from_slice(encrypted_key); // Copy the encrypted key into the buffer

        let nonce_array: [u8; 12] = nonce.try_into().map_err(|_| "Invalid nonce length")?;

        // Open the encrypted key in place
        aead_key
            .open_in_place(
                aead::Nonce::assume_unique_for_key(nonce_array),
                aead::Aad::empty(),
                &mut decrypted_key,
            )
            .map_err(|_| "Decryption failed")?;

        // Check the length of decrypted_key
        println!("Decrypted key length: {}", decrypted_key.len()); // Should be 48

        // Extract the first 32 bytes for the SigningKey
        if decrypted_key.len() < 32 {
            return Err("Decrypted key must be at least 32 bytes long".into());
        }

        // Create a SigningKey from the first 32 bytes of the decrypted key
        let signing_key = SigningKey::try_from(&decrypted_key[..32])?; // Use only the first 32 bytes

        Ok(signing_key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::pki_auth_key::PKIAuthenticationKey;

    #[test]
    fn test_encryption() -> Result<(), String> {
        let (test_prv_key, _) = PKIAuthenticationKey::new_ed25519_key_pair();
        let encrypted_prv_key =
            CryptoService::encrypt_private_key(&test_prv_key, "test_pass")
                .expect("Encryption failed");
        let dec_prv_key = CryptoService::decrypt_private_key(&encrypted_prv_key, "test_pass")
            .expect("Decryption failed");

        assert_eq!(STANDARD.encode(&test_prv_key.as_ref()), STANDARD.encode(&dec_prv_key.as_ref()));
        Ok(())
    }
}