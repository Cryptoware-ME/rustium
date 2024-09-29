use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::{authentication::auth_user::AuthUser, RustiumResult};

type TokenResult = RustiumResult<String>;
type TokenDecodeResult = RustiumResult<TokenData<Claims>>;

static VALIDATION: Lazy<Validation> = Lazy::new(Validation::default);
static HEADER: Lazy<Header> = Lazy::new(Header::default);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenUser {
    pub id: String,
    pub username: String,
    pub is_admin: bool,
}

impl<T: AuthUser> From<T> for TokenUser {
    fn from(user: T) -> Self {
        Self {
            id: user.get_id(),
            username: user.get_username(),
            is_admin: user.is_admin(),
        }
    }
}

impl From<Box<dyn AuthUser>> for TokenUser {
    fn from(user: Box<dyn AuthUser>) -> Self {
        Self {
            id: user.get_id(),
            username: user.get_username(),
            is_admin: user.is_admin(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub user: TokenUser,
}

impl Claims {
    pub fn new<T: AuthUser>(user: T) -> Self {
        Self {
            exp: (chrono::Local::now() + chrono::Duration::days(30)).timestamp() as usize,
            iat: chrono::Local::now().timestamp() as usize,
            user: TokenUser::from(user),
        }
    }
}

pub fn create<T: AuthUser>(user: T, secret: &str) -> TokenResult {
    let encoding_key = EncodingKey::from_secret(secret.as_ref());
    let claims = Claims::new(user);

    Ok(jsonwebtoken::encode(&HEADER, &claims, &encoding_key)?)
}

pub fn encode_auth_token<T: AuthUser>(user: T, secret: &str) -> TokenResult {
    let encoding_key = EncodingKey::from_secret(secret.as_ref());
    let claims = Claims::new(user);

    Ok(jsonwebtoken::encode(&HEADER, &claims, &encoding_key)?)
}

pub fn decode_auth_token(token: &str, secret: &str) -> TokenDecodeResult {
    let decoding_key = DecodingKey::from_secret(secret.as_ref());

    Ok(jsonwebtoken::decode::<Claims>(
        token,
        &decoding_key,
        &VALIDATION,
    )?)
}
