use hyper::StatusCode;
use uuid::Uuid;

use crate::{
    api::APIError, // TODO: refactor the APIError dependency.
    application::{
        config::Config,
        repository::user_repo,
        security::{auth_error::*, jwt_claims::*},
        service::token_service,
        state::SharedState,
    },
    domain::models::user::User,
};

pub struct JwtTokens {
    pub access_token: String,
    pub refresh_token: String,
}

pub async fn logout(refresh_claims: RefreshClaims, state: SharedState) -> Result<(), APIError> {
    // Checking the configuration if the usage of the list of revoked tokens is enabled.
    if state.config.jwt_enable_revoked_tokens {
        // Decode and validate the refresh token.
        if !validate_token_type(&refresh_claims, JwtTokenType::RefreshToken) {
            return Err(AuthError::InvalidToken.into());
        }
        revoke_refresh_token(&refresh_claims, &state).await
    } else {
        Err(StatusCode::NOT_ACCEPTABLE)?
    }
}

pub async fn refresh(
    refresh_claims: RefreshClaims,
    state: SharedState,
) -> Result<JwtTokens, APIError> {
    // Decode and validate the refresh token.
    if !validate_token_type(&refresh_claims, JwtTokenType::RefreshToken) {
        return Err(AuthError::InvalidToken.into());
    }

    // Checking the configuration if the usage of the list of revoked tokens is enabled.
    if state.config.jwt_enable_revoked_tokens {
        revoke_refresh_token(&refresh_claims, &state).await?;
    }

    let user_id = refresh_claims.sub.parse().unwrap();
    let user = user_repo::get_by_id(user_id, &state).await?;
    let tokens = generate_tokens(user, &state.config);
    Ok(tokens)
}

pub async fn cleanup_revoked_and_expired(
    _access_claims: &AccessClaims,
    state: &SharedState,
) -> Result<usize, APIError> {
    // Checking the configuration if the usage of the list of revoked tokens is enabled.
    if !state.config.jwt_enable_revoked_tokens {
        Err(StatusCode::NOT_ACCEPTABLE)?;
    }

    if let Some(deleted) = token_service::cleanup_expired(state).await {
        return Ok(deleted);
    }

    Err(StatusCode::INTERNAL_SERVER_ERROR)?
}

pub fn validate_token_type(claims: &RefreshClaims, expected_type: JwtTokenType) -> bool {
    if claims.typ == expected_type as u8 {
        return true;
    }
    tracing::error!(
        "Invalid token type. Expected {:?}, Found {:?}",
        expected_type,
        JwtTokenType::from(claims.typ),
    );
    false
}

async fn revoke_refresh_token(
    refresh_claims: &RefreshClaims,
    state: &SharedState,
) -> Result<(), APIError> {
    // Check the validity of refresh token.
    validate_revoked(refresh_claims, state).await?;
    if token_service::revoke_refresh_token(refresh_claims, state).await {
        return Ok(());
    }
    Err(StatusCode::INTERNAL_SERVER_ERROR)?
}

pub fn generate_tokens(user: User, config: &Config) -> JwtTokens {
    let time_now = chrono::Utc::now();
    let iat = time_now.timestamp() as usize;
    let sub = user.id.to_string();

    let access_token_id = Uuid::new_v4().to_string();
    let refresh_token_id = Uuid::new_v4().to_string();
    let access_token_exp = (time_now
        + chrono::Duration::seconds(config.jwt_expire_access_token_seconds))
    .timestamp() as usize;

    let access_claims = AccessClaims {
        sub: sub.clone(),
        jti: access_token_id.clone(),
        iat,
        exp: access_token_exp,
        typ: JwtTokenType::AccessToken as u8,
        roles: user.roles.clone(),
    };

    let refresh_claims = RefreshClaims {
        sub,
        jti: refresh_token_id,
        iat,
        exp: (time_now + chrono::Duration::seconds(config.jwt_expire_refresh_token_seconds))
            .timestamp() as usize,
        prf: access_token_id,
        pex: access_token_exp,
        typ: JwtTokenType::RefreshToken as u8,
        roles: user.roles,
    };

    tracing::info!(
        "JWT: generated claims\naccess {:#?}\nrefresh {:#?}",
        access_claims,
        refresh_claims
    );

    let access_token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &access_claims,
        &jsonwebtoken::EncodingKey::from_secret(config.jwt_secret.as_ref()),
    )
    .unwrap();

    let refresh_token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &refresh_claims,
        &jsonwebtoken::EncodingKey::from_secret(config.jwt_secret.as_ref()),
    )
    .unwrap();

    tracing::info!(
        "JWT: generated tokens\naccess {:#?}\nrefresh {:#?}",
        access_token,
        refresh_token
    );

    JwtTokens {
        access_token,
        refresh_token,
    }
}

pub async fn validate_revoked<T: std::fmt::Debug + ClaimsMethods + Sync + Send>(
    claims: &T,
    state: &SharedState,
) -> Result<(), APIError> {
    match token_service::is_revoked(claims, state).await {
        Some(revoked) => {
            if revoked {
                Err(AuthError::WrongCredentials)?;
            }
        }
        None => Err(StatusCode::INTERNAL_SERVER_ERROR)?,
    }
    Ok(())
}
