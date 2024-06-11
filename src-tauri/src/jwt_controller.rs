use std::env;

use jsonwebtoken::{Algorithm, decode, encode, Header, Validation};
use rand::distributions::Alphanumeric;
use rand::Rng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::errors::{ENCODING_ERR, ENV_VAR_NOT_SET};
use crate::table_jwt_tokens::JwtToken;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub(crate) user_uuid: Uuid,
}

pub fn generate_jwt(user_uuid: Uuid) -> JwtToken {
    let my_claims = Claims { user_uuid };
    let key = env::var("SCHEDULEFLOW_JWT_SECRET").unwrap_or_else(|_| {
        let key = generate_secret_key();
        env::set_var("SCHEDULEFLOW_JWT_SECRET", &key);
        key
    });

    let encoding_key = jsonwebtoken::EncodingKey::from_secret(key.as_ref());
    let token = encode(&Header::default(), &my_claims, &encoding_key);

    JwtToken {
        token: token.expect(ENCODING_ERR),
        user_uuid,
    }
}

pub fn decode_jwt(
    token: &str,
) -> Result<jsonwebtoken::TokenData<Claims>, jsonwebtoken::errors::Error> {
    let key = env::var("SCHEDULEFLOW_JWT_SECRET").expect(ENV_VAR_NOT_SET);
    let decoding_key = jsonwebtoken::DecodingKey::from_secret(key.as_ref());
    let token_data = decode::<Claims>(token, &decoding_key, &Validation::new(Algorithm::HS256));
    token_data
}

fn generate_secret_key() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect()
}
