use std::env;
use jsonwebtoken::{Algorithm, decode, encode, Header, Validation};
use rand::distributions::Alphanumeric;
use rand::Rng;
use serde::{Deserialize, Serialize};
use crate::errors::ENV_VAR_NOT_SET;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    subject: String,
}

pub fn generate_jwt(subject: String, expiration: usize) -> Result<String, jsonwebtoken::errors::Error> {
    let my_claims = Claims {
        subject,
    };
    let key = generate_secret_key();
    env::set_var("SCHEDULEFLOW_JWT_SECRET", &key);

    // Encode
    let token = encode(&Header::default(), &my_claims, key.as_ref());
    token
}

pub fn decode_jwt(token: &str) -> Result<jsonwebtoken::TokenData<Claims>, jsonwebtoken::errors::Error> {
    let key = env::var("SCHEDULEFLOW_JWT_SECRET").expect(ENV_VAR_NOT_SET);
    let token_data = decode::<Claims>(token, key.as_ref(), &Validation::new(Algorithm::HS256));
    token_data
}

fn generate_secret_key() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect()
}
