use std::collections::HashSet;
use std::env;
use std::ops::DerefMut;

use crate::db::db_actions::DbActions;
use crate::db::model::jwt_token::JwtToken;
use crate::db::repository::jwt_token_repository::JwtTokenRepository;
use crate::db::repository::user_repository::UserRepository;
use crate::errors::error_messages::{BCRYPT_ENCODING_ERR, ENV_VAR_NOT_SET};
use crate::runtime_objects::{driver, set_current_user};
use jsonwebtoken::{decode, encode, Algorithm, Header, Validation};
use rand::distributions::Alphanumeric;
use rand::Rng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub(crate) user_uuid: Uuid,
}

#[tauri::command]
pub fn is_valid_session(token: String) -> bool {
    let token_data = decode_jwt(&token);
    let mut token_obj: JwtToken = JwtToken::empty();
    let mut user_tokens: Vec<JwtToken> = vec![];
    let mut user_uuid;

    if let Ok(data) = token_data {
        user_uuid = data.claims.user_uuid;
        token_obj = JwtToken { token, user_uuid };
        let user_matches = format!("user_uuid = '{}'", &user_uuid);

        user_tokens =
            JwtTokenRepository::retrieve(driver().lock().unwrap().deref_mut(), Some(user_matches));
    } else {
        user_uuid = Uuid::default();
    }

    if user_tokens.contains(&token_obj) {
        if let Ok(user) =
            UserRepository::get_by_uuid(driver().lock().unwrap().deref_mut(), user_uuid)
        {
            set_current_user(user);
        }
        true
    } else {
        false
    }
}

pub fn generate_jwt(user_uuid: Uuid) -> JwtToken {
    let my_claims = Claims { user_uuid };
    let key = env::var("SCHEDULEFLOW_JWT_SECRET").expect(ENV_VAR_NOT_SET);

    let encoding_key = jsonwebtoken::EncodingKey::from_secret(key.as_ref());
    let token = encode(&Header::default(), &my_claims, &encoding_key);

    JwtToken {
        token: token.expect(BCRYPT_ENCODING_ERR),
        user_uuid,
    }
}

pub fn decode_jwt(
    token: &str,
) -> Result<jsonwebtoken::TokenData<Claims>, jsonwebtoken::errors::Error> {
    let key = env::var("SCHEDULEFLOW_JWT_SECRET").expect(ENV_VAR_NOT_SET);
    let decoding_key = jsonwebtoken::DecodingKey::from_secret(&key.as_ref());

    let mut validation = Validation::new(Algorithm::HS256);
    validation.required_spec_claims = HashSet::new();

    decode::<Claims>(token, &decoding_key, &validation)
}

fn generate_secret_key() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect()
}
