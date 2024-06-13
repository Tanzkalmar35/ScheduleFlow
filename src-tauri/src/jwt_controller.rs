use std::collections::HashSet;
use std::env;
use std::ops::DerefMut;

use crate::{CURRENT_USER, driver};
use jsonwebtoken::{decode, encode, Algorithm, Header, Validation};
use jsonwebtoken::errors::ErrorKind;
use rand::distributions::Alphanumeric;
use rand::Rng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::db_actions::DbActions;

use crate::errors::{ENCODING_ERR, ENV_VAR_NOT_SET};
use crate::table_jwt_tokens::JwtToken;
use crate::table_users::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub(crate) user_uuid: Uuid,
}

#[tauri::command]
pub fn is_valid_session(token: String) -> bool {
    let token_data = decode_jwt(&token);
    let mut token_obj: JwtToken = JwtToken::empty();
    let mut user_tokens: Vec<JwtToken> = vec![];

    if let Ok(data) = token_data {
        let user_uuid = data.claims.user_uuid;
        token_obj = JwtToken {token, user_uuid};
        let condition_user_matches = format!("user_uuid = '{}'", user_uuid);

        user_tokens = JwtToken::retrieve(driver().lock().unwrap().deref_mut(), Some(condition_user_matches));
    }

    user_tokens.contains(&token_obj)
}

pub fn generate_jwt(user_uuid: Uuid) -> JwtToken {
    let my_claims = Claims {
        user_uuid,
    };
    let key = env::var("SCHEDULEFLOW_JWT_SECRET").expect(ENV_VAR_NOT_SET);

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
