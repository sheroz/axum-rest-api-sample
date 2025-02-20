use std::collections::HashMap;

use redis::{AsyncCommands, RedisResult, aio::MultiplexedConnection};
use tokio::sync::MutexGuard;

use crate::application::{
    constants::*,
    security::jwt::{ClaimsMethods, RefreshClaims},
    state::SharedState,
};

pub async fn revoke_global(state: &SharedState) -> RedisResult<()> {
    let timestamp_now = chrono::Utc::now().timestamp() as usize;
    tracing::debug!("setting a timestamp for global revoke: {}", timestamp_now);
    state
        .redis
        .lock()
        .await
        .set(JWT_REDIS_REVOKE_GLOBAL_BEFORE_KEY, timestamp_now)
        .await
}

pub async fn revoke_user_tokens(user_id: &str, state: &SharedState) -> RedisResult<()> {
    let timestamp_now = chrono::Utc::now().timestamp() as usize;
    tracing::debug!(
        "adding a timestamp for user revoke, user:{}, timestamp: {}",
        user_id,
        timestamp_now
    );
    state
        .redis
        .lock()
        .await
        .hset(JWT_REDIS_REVOKE_USER_BEFORE_KEY, user_id, timestamp_now)
        .await
}

async fn is_global_revoked<T: ClaimsMethods + Sync + Send>(
    claims: &T,
    redis: &mut MutexGuard<'_, redis::aio::MultiplexedConnection>,
) -> RedisResult<bool> {
    // Check in global revoke.
    let opt_exp: Option<String> = redis.get(JWT_REDIS_REVOKE_GLOBAL_BEFORE_KEY).await?;
    if let Some(exp) = opt_exp {
        let global_exp = exp.parse::<usize>().unwrap();
        if global_exp >= claims.get_iat() {
            return Ok(true);
        }
    }
    Ok(false)
}

async fn is_user_revoked<T: ClaimsMethods + Sync + Send>(
    claims: &T,
    redis: &mut MutexGuard<'_, redis::aio::MultiplexedConnection>,
) -> RedisResult<bool> {
    // Check in user revoke.
    let user_id = claims.get_sub();
    let opt_exp: Option<String> = redis
        .hget(JWT_REDIS_REVOKE_USER_BEFORE_KEY, user_id)
        .await?;
    if let Some(exp) = opt_exp {
        let global_exp = exp.parse::<usize>().unwrap();
        if global_exp >= claims.get_iat() {
            return Ok(true);
        }
    }

    Ok(false)
}

async fn is_token_revoked<T: ClaimsMethods + Sync + Send>(
    claims: &T,
    redis: &mut MutexGuard<'_, redis::aio::MultiplexedConnection>,
) -> RedisResult<bool> {
    // Check the token in revoked list.
    redis
        .hexists(JWT_REDIS_REVOKED_TOKENS_KEY, claims.get_jti())
        .await
}

pub async fn is_revoked<T: std::fmt::Debug + ClaimsMethods + Send + Sync>(
    claims: &T,
    state: &SharedState,
) -> RedisResult<bool> {
    let mut redis = state.redis.lock().await;

    let global_revoked = is_global_revoked(claims, &mut redis).await?;
    if global_revoked {
        tracing::error!("Access denied (globally revoked): {:#?}", claims);
        return Ok(true);
    }

    let user_revoked = is_user_revoked(claims, &mut redis).await?;
    if user_revoked {
        tracing::error!("Access denied (user revoked): {:#?}", claims);
        return Ok(true);
    }

    let token_revoked = is_token_revoked(claims, &mut redis).await?;
    if token_revoked {
        tracing::error!("Access denied (token revoked): {:#?}", claims);
        return Ok(true);
    }

    drop(redis);
    Ok(false)
}

pub async fn revoke_refresh_token(claims: &RefreshClaims, state: &SharedState) -> RedisResult<()> {
    // Adds refersh token and its paired access token into revoked list in Redis.
    // Tokens are tracked by JWT ID that handles the cases of reusing lost tokens and multi-device scenarios.

    let list_to_revoke = vec![&claims.jti, &claims.prf];
    tracing::debug!("adding jwt tokens into revoked list: {:#?}", list_to_revoke);

    let mut redis = state.redis.lock().await;
    for claims_jti in list_to_revoke {
        let _: () = redis
            .hset(JWT_REDIS_REVOKED_TOKENS_KEY, claims_jti, claims.exp)
            .await?;
    }

    if tracing::enabled!(tracing::Level::TRACE) {
        log_revoked_tokens_count(&mut redis).await;
    }
    drop(redis);

    Ok(())
}

pub async fn cleanup_expired(state: &SharedState) -> RedisResult<usize> {
    let timestamp_now = chrono::Utc::now().timestamp() as usize;

    let mut redis = state.redis.lock().await;

    let revoked_tokens: HashMap<String, String> =
        redis.hgetall(JWT_REDIS_REVOKED_TOKENS_KEY).await?;

    let mut deleted = 0;
    for (key, exp) in revoked_tokens {
        match exp.parse::<usize>() {
            Ok(timestamp_exp) => {
                if timestamp_now > timestamp_exp {
                    // Workaround for https://github.com/redis-rs/redis-rs/issues/1322
                    let _: () = redis.hdel(JWT_REDIS_REVOKED_TOKENS_KEY, key).await?;
                    deleted += 1;
                }
            }
            Err(e) => {
                tracing::error!("{}", e);
            }
        }
    }

    if tracing::enabled!(tracing::Level::TRACE) {
        log_revoked_tokens_count(&mut redis).await;
    }
    drop(redis);

    Ok(deleted)
}

pub async fn log_revoked_tokens_count(redis: &mut MultiplexedConnection) {
    let redis_result: RedisResult<usize> = redis.hlen(JWT_REDIS_REVOKED_TOKENS_KEY).await;
    match redis_result {
        Ok(revoked_tokens_count) => {
            tracing::debug!(
                "REDIS: count of revoked jwt tokens: {}",
                revoked_tokens_count
            );
        }
        Err(e) => {
            tracing::error!("{}", e);
        }
    }
}

pub async fn log_revoked_tokens(redis: &mut MultiplexedConnection) {
    let redis_result: RedisResult<HashMap<String, String>> =
        redis.hgetall(JWT_REDIS_REVOKED_TOKENS_KEY).await;

    match redis_result {
        Ok(revoked_tokens) => {
            tracing::trace!("REDIS: list of revoked jwt tokens: {:#?}", revoked_tokens);
        }
        Err(e) => {
            tracing::error!("{}", e);
        }
    }
}
