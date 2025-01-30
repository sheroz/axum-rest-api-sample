use serde::{Deserialize, Serialize};

use crate::application::{
    config::Config,
    security::{auth::AuthError, roles},
};

// [JWT Claims]
// [RFC7519](https://datatracker.ietf.org/doc/html/rfc7519#section-4)
// roles, groups: https://www.rfc-editor.org/rfc/rfc7643.html#section-4.1.2
// https://www.rfc-editor.org/rfc/rfc9068.html#name-authorization-claims

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessClaims {
    /// Subject.
    pub sub: String,
    /// JWT ID.
    pub jti: String,
    /// Issued time.
    pub iat: usize,
    /// Expiration time.
    pub exp: usize,
    /// Token type.
    pub typ: u8,
    /// Roles.
    pub roles: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshClaims {
    /// Subject.
    pub sub: String,
    /// JWT ID.
    pub jti: String,
    /// Issued time.
    pub iat: usize,
    /// Expiration time.
    pub exp: usize,
    /// Reference to paired access token,
    pub prf: String,
    /// Expiration time of paired access token,
    pub pex: usize,
    /// Token type.
    pub typ: u8,
    /// Roles.
    pub roles: String,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum JwtTokenType {
    AccessToken,
    RefreshToken,
    UnknownToken,
}
impl From<u8> for JwtTokenType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::AccessToken,
            1 => Self::RefreshToken,
            _ => Self::UnknownToken,
        }
    }
}

pub trait ClaimsMethods {
    fn validate_role_admin(&self) -> Result<(), AuthError>;
    fn get_sub(&self) -> &str;
    fn get_exp(&self) -> usize;
    fn get_iat(&self) -> usize;
    fn get_jti(&self) -> &str;
}

impl ClaimsMethods for AccessClaims {
    fn validate_role_admin(&self) -> Result<(), AuthError> {
        roles::is_role_admin(&self.roles)
    }
    fn get_sub(&self) -> &str {
        &self.sub
    }

    fn get_iat(&self) -> usize {
        self.iat
    }

    fn get_exp(&self) -> usize {
        self.exp
    }

    fn get_jti(&self) -> &str {
        &self.jti
    }
}
impl ClaimsMethods for RefreshClaims {
    fn validate_role_admin(&self) -> Result<(), AuthError> {
        roles::is_role_admin(&self.roles)
    }
    fn get_sub(&self) -> &str {
        &self.sub
    }

    fn get_iat(&self) -> usize {
        self.iat
    }

    fn get_exp(&self) -> usize {
        self.exp
    }

    fn get_jti(&self) -> &str {
        &self.jti
    }
}

pub fn decode_token<T: for<'de> serde::Deserialize<'de>>(
    token: &str,
    config: &Config,
) -> Result<T, AuthError> {
    let mut validation = jsonwebtoken::Validation::default();
    validation.leeway = config.jwt_validation_leeway_seconds as u64;
    let token_data = jsonwebtoken::decode::<T>(token, &config.jwt_keys.decoding, &validation)
        .map_err(|_| {
            tracing::error!("Invalid token: {}", token);
            AuthError::WrongCredentials
        })?;

    Ok(token_data.claims)
}
