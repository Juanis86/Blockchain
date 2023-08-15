use actix_web::{web, HttpResponse};
use crate::models::{Dispute, NewDispute};
use crate::error::{AppError, AppErrorCode};
use crate::db::Pool;

// Estructura para representar una solicitud de creación de disputa
// Puedes ajustar los campos según los detalles que necesites para una disputa
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDisputeRequest {
    // Campos relevantes para la creación de una disputa
    // Por ejemplo: contract_id, description, etc.
}

// Función para crear una nueva disputa
pub async fn create_dispute(
    req: web::Json<CreateDisputeRequest>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, AppError> {
    // Obtener los datos de la solicitud JSON
    let dispute_data = req.into_inner();

    // Procesar la creación de la disputa
    let new_dispute = NewDispute {
        // Asignar los campos desde dispute_data
    };

    let dispute = crate::db::create_dispute(&pool, new_dispute)
        .map_err(|_| AppError::new(AppErrorCode::DatabaseError))?;

    // Retornar una respuesta exitosa con los detalles de la disputa creada
    Ok(HttpResponse::Ok().json(dispute))
}

// Función para obtener detalles de una disputa por su ID
pub async fn get_dispute_by_id(
    id: web::Path<i32>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, AppError> {
    let dispute_id = id.into_inner();

    // Consultar la base de datos para obtener la disputa por su ID
    let dispute = crate::db::get_dispute_by_id(&pool, dispute_id)
        .ok_or_else(|| AppError::new(AppErrorCode::NotFound))?;

    // Retornar una respuesta con los detalles de la disputa
    Ok(HttpResponse::Ok().json(dispute))
}

// Otras funciones para actualizar, eliminar, listar disputas, etc.
// Puedes agregar aquí las funciones específicas para tu API de disputas
