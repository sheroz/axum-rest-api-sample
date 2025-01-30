use std::fmt::Display;

use crate::application::{
    constants::{USER_ROLE_ADMIN, USER_ROLE_CUSTOMER, USER_ROLE_GUEST},
    security::auth::AuthError,
};

/// User roles.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum UserRole {
    Admin,
    Customer,
    Guest,
}

impl TryFrom<&str> for UserRole {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            USER_ROLE_ADMIN => Ok(Self::Admin),
            USER_ROLE_CUSTOMER => Ok(Self::Customer),
            USER_ROLE_GUEST => Ok(Self::Guest),
            _ => Err("Unknown role"),
        }
    }
}

impl Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Admin => write!(f, "{}", USER_ROLE_ADMIN),
            Self::Customer => write!(f, "{}", USER_ROLE_CUSTOMER),
            Self::Guest => write!(f, "{}", USER_ROLE_GUEST),
        }
    }
}

impl UserRole {
    pub fn is_role_admin(&self) -> bool {
        *self == Self::Admin
    }
}

pub fn contains_role_admin(roles: &str) -> bool {
    if roles.is_empty() {
        return false;
    }

    let role_admin = UserRole::Admin.to_string();
    roles.split(',').map(|s| s.trim()).any(|x| x == role_admin)
}

pub fn is_role_admin(roles: &str) -> Result<(), AuthError> {
    if !contains_role_admin(roles) {
        return Err(AuthError::Forbidden);
    }
    Ok(())
}
