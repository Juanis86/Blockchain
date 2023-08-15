use bcrypt::{hash, verify, DEFAULT_COST};
use crate::models::{User, Permission};
use crate::error::AppError;

// Función para verificar la autenticación de un usuario
pub fn authenticate_user(username: &str, password: &str, users: &Vec<User>) -> Result<User, AppError> {
    if let Some(user) = users.iter().find(|u| u.username == username) {
        if verify_password(&user.password_hash, password) {
            Ok(user.clone())
        } else {
            Err(AppError::AuthenticationError("Incorrect password".to_string()))
        }
    } else {
        Err(AppError::AuthenticationError("User not found".to_string()))
    }
}

// Función para verificar los permisos de un usuario
pub fn check_user_permissions(user: &User, required_permissions: Vec<Permission>) -> bool {
    user.permissions.iter().any(|permission| required_permissions.contains(permission))
}

// Función para cifrar una contraseña
pub fn hash_password(password: &str) -> Result<String, AppError> {
    hash(password, DEFAULT_COST).map_err(|_| AppError::InternalServerError)
}

// Función para verificar si una contraseña coincida con su versión cifrada
pub fn verify_password(hash: &str, password: &str) -> bool {
    verify(password, hash).unwrap_or(false)
}
