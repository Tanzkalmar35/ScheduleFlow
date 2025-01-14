use base64::engine::general_purpose::STANDARD;
use base64::{DecodeError, Engine};
use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;

pub struct PKIAuthenticationKey {
    key: Vec<u8>,
}

impl PKIAuthenticationKey {
    pub fn get_key(&self) -> &Vec<u8> {
        &self.key
    }

    pub fn to_base64(&self) -> String {
        STANDARD.encode(&self.key)
    }

    pub fn from_base64(str: &str) -> Result<Self, DecodeError> {
        let bytes = STANDARD.decode(str)?;
        Ok(Self { key: bytes })
    }

    pub fn new_ed25519_key_pair() -> (SigningKey, VerifyingKey) {
        let mut csprng = OsRng;

        let signing_key = SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();

        (signing_key, verifying_key)
    }
}
