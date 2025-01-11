use base64::engine::general_purpose::STANDARD;
use base64::{DecodeError, Engine};

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
}
