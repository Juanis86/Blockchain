use actix_web::{web, HttpResponse};
use crate::models::{Notification, NewNotification};
use crate::error::{AppError, AppErrorCode};
use crate::db::Pool;

// Estructura para representar una solicitud de creación de notificación
// Puedes ajustar los campos según los detalles que necesites para una notificación
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateNotificationRequest {
    // Campos relevantes para la creación de una notificación
    // Por ejemplo: user_id, message, etc.
}

// Función para crear una nueva notificación
pub async fn create_notification(
    req: web::Json<CreateNotificationRequest>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, AppError> {
    // Obtener los datos de la solicitud JSON
    let notification_data = req.into_inner();

    // Procesar la creación de la notificación
    let new_notification = NewNotification {
        // Asignar los campos desde notification_data
    };

    let notification = crate::db::create_notification(&pool, new_notification)
        .map_err(|_| AppError::new(AppErrorCode::DatabaseError))?;

    // Retornar una respuesta exitosa con los detalles de la notificación creada
    Ok(HttpResponse::Ok().json(notification))
}

// Función para obtener detalles de una notificación por su ID
pub async fn get_notification_by_id(
    id: web::Path<i32>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, AppError> {
    let notification_id = id.into_inner();

    // Consultar la base de datos para obtener la notificación por su ID
    let notification = crate::db::get_notification_by_id(&pool, notification_id)
        .ok_or_else(|| AppError::new(AppErrorCode::NotFound))?;

    // Retornar una respuesta con los detalles de la notificación
    Ok(HttpResponse::Ok().json(notification))
}

// Otras funciones para actualizar, eliminar, listar notificaciones, etc.
// Puedes agregar aquí las funciones específicas para tu API de notificaciones
