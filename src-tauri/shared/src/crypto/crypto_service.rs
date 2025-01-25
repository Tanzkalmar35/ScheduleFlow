use std::num::NonZeroU32;

use base64::{engine::general_purpose::STANDARD, Engine};
use ed25519_dalek::{ed25519::signature::SignerMut, SecretKey, SigningKey, VerifyingKey};
use rand::{rngs::OsRng, RngCore};
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
    pub fn encrypt_private_key(
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
    pub fn decrypt_private_key(
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

        // Extract the first 32 bytes for the SigningKey
        if decrypted_key.len() < 32 {
            return Err("Decrypted key must be at least 32 bytes long".into());
        }

        // Create a SigningKey from the first 32 bytes of the decrypted key
        let signing_key = SigningKey::try_from(&decrypted_key[..32])?; // Use only the first 32 bytes

        Ok(signing_key)
    }

    pub fn generate_challenge() -> Vec<u8> {
        let mut rng = OsRng; // Create a secure random number generator
        let mut challenge = vec![0u8; 16]; // Create a buffer for the challenge (16 bytes)
        rng.fill_bytes(&mut challenge); // Fill the buffer with random bytes
        challenge // Return the generated challenge
    }

    pub fn attempt_sign(prv_key: &Vec<u8>, pub_key: &Vec<u8>) -> bool {
        let secret_key = SecretKey::try_from(&prv_key[..32]);
        let mut signing_key = SigningKey::from_bytes(&secret_key.unwrap());
        let verifying_key = VerifyingKey::try_from(&pub_key[..32]);
        let challenge = Self::generate_challenge();

        let signature = signing_key.sign(&challenge);

        verifying_key
            .unwrap()
            .verify_strict(&challenge, &signature)
            .is_ok()
    }

    pub fn new_ed25519_key_pair() -> (SigningKey, VerifyingKey) {
        let mut csprng = OsRng;

        let signing_key = SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();

        (signing_key, verifying_key)
    }
}

#[cfg(test)]
mod tests {
    use crate::crypto::secure_storage::SecureStorage;

    use super::*;

    #[test]
    fn test_encryption() -> Result<(), String> {
        let (test_prv_key, _) = CryptoService::new_ed25519_key_pair();
        let encrypted_prv_key = CryptoService::encrypt_private_key(&test_prv_key, "test_pass")
            .expect("Encryption failed");
        let dec_prv_key = CryptoService::decrypt_private_key(&encrypted_prv_key, "test_pass")
            .expect("Decryption failed");

        assert_eq!(
            STANDARD.encode(&test_prv_key.as_ref()),
            STANDARD.encode(&dec_prv_key.as_ref())
        );
        Ok(())
    }

    #[test]
    fn test_encryption_retrieval() {
        let (prv_key, pub_key) = CryptoService::new_ed25519_key_pair();
        let enc_prv_key = CryptoService::encrypt_private_key(&prv_key, &String::from("pass"));
        assert!(SecureStorage::store_system_key(
            enc_prv_key.as_ref().unwrap(),
            &String::from("email"),
        )
        .is_ok());
        let key = SecureStorage::get_system_key(&String::from("email")).unwrap();
        let dec_prv_key = CryptoService::decrypt_private_key(key.as_str(), &String::from("pass"));
        assert!(CryptoService::attempt_sign(
            &dec_prv_key.unwrap().to_bytes().to_vec(),
            &pub_key.to_bytes().to_vec(),
        ));
    }

    #[test]
    fn test_signing() {
        let (prv_key1, pub_key1) = CryptoService::new_ed25519_key_pair();
        assert!(CryptoService::attempt_sign(
            &prv_key1.to_bytes().to_vec(),
            &pub_key1.to_bytes().to_vec()
        ));

        let (prv_key2, pub_key2) = CryptoService::new_ed25519_key_pair();
        assert!(CryptoService::attempt_sign(
            &prv_key2.to_bytes().to_vec(),
            &pub_key2.to_bytes().to_vec()
        ));
        assert_eq!(
            CryptoService::attempt_sign(
                &prv_key1.to_bytes().to_vec(),
                &pub_key2.to_bytes().to_vec()
            ),
            false
        );
        assert_eq!(
            CryptoService::attempt_sign(
                &prv_key2.to_bytes().to_vec(),
                &pub_key1.to_bytes().to_vec()
            ),
            false
        );
    }
}
