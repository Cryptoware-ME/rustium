pub mod auth_request;
pub mod auth_service;
pub mod auth_user;
pub mod authorization;
pub mod token;

use argon2::Argon2;

use crate::prelude::*;

pub fn hash_password(password: String) -> RustiumResult<String> {
    let mut output_key_material = [0u8; 32];
    let v8 = match Argon2::default().hash_password_into(
        password.into_bytes().as_slice(),
        b"SuperSecret",
        &mut output_key_material,
    ) {
        Ok(()) => output_key_material.to_vec(),
        Err(e) => {
            println!("Error: {}", e);
            [0u8; 16].to_vec()
        }
    };

    Ok(hex::encode(v8))
}
