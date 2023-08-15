use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize}; // Asegurarse de importar serde

use crate::models::{NewUser, UpdatedUser}; // Asumiendo que las estructuras NewUser y UpdatedUser están definidas en crate::models
use crate::error::{AppError, AppErrorCode}; // Asumiendo que AppError y AppErrorCode están definidos en crate::error
use crate::db::Pool; // Asumiendo que Pool está definido en crate::db

// Estructura para representar una solicitud de creación de usuarios
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    // Otros campos de seguridad y perfil que consideres necesarios
}

// Estructura para representar una solicitud de actualización de usuarios
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    // Otros campos de seguridad y perfil que se pueden actualizar
}

// Función para crear un nuevo usuario
pub async fn create_user(
    req: web::Json<CreateUserRequest>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, AppError> {
    let user_data = req.into_inner(); // Obtener los datos de la solicitud JSON

    let new_user = NewUser {
        username: user_data.username,
        email: user_data.email,
        password: user_data.password,
        // Otros campos de seguridad y perfil que consideres necesarios
    };

    let user = crate::db::create_user(&pool, new_user)?; // Cambiar .map_err(|_| ...) a ?

    Ok(HttpResponse::Ok().json(user)) // Retornar una respuesta exitosa con los detalles del usuario creado
}

// Función para obtener detalles de un usuario por su ID
pub async fn get_user_by_id(
    user_id: web::Path<i32>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, AppError> {
    let user_id = user_id.into_inner();

    let user = crate::db::get_user_by_id(&pool, user_id)?; // Cambiar .map_err(|_| ...) a ?

    Ok(HttpResponse::Ok().json(user)) // Retornar una respuesta con los detalles del usuario
}

// Función para actualizar los detalles de un usuario por su ID
pub async fn update_user(
    user_id: web::Path<i32>,
    req: web::Json<UpdateUserRequest>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, AppError> {
    let user_id = user_id.into_inner();
    let updated_user_data = req.into_inner();

    let updated_user = UpdatedUser {
        username: updated_user_data.username,
        email: updated_user_data.email,
        password: updated_user_data.password,
        // Otros campos de seguridad y perfil que se pueden actualizar
    };

    let user = crate::db::update_user(&pool, user_id, updated_user)?; // Cambiar .map_err(|_| ...) a ?

    Ok(HttpResponse::Ok().json(user)) // Retornar una respuesta exitosa con los detalles del usuario actualizado
}

// Otras funciones para autenticación, eliminación de usuarios, etc.
// Puedes agregar aquí las funciones específicas para tu API de usuarios
