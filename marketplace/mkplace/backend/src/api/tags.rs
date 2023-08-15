use actix_web::{web, HttpResponse};
use crate::models::Tag;
use crate::error::{AppError, AppErrorCode};
use crate::db::Pool;

// Estructura para representar una solicitud de creación de etiquetas
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTagRequest {
    // Campos relevantes para la creación de una etiqueta
    // Por ejemplo: tag_name, tag_description, etc.
}

// Función para crear una nueva etiqueta
pub async fn create_tag(
    req: web::Json<CreateTagRequest>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, AppError> {
    // Obtener los datos de la solicitud JSON
    let tag_data = req.into_inner();

    // Procesar la creación de la etiqueta
    let new_tag = NewTag {
        // Asignar los campos desde tag_data
    };

    let tag = crate::db::create_tag(&pool, new_tag)
        .map_err(|_| AppError::new(AppErrorCode::DatabaseError))?;

    // Retornar una respuesta exitosa con los detalles de la etiqueta creada
    Ok(HttpResponse::Ok().json(tag))
}

// Función para obtener todas las etiquetas
pub async fn get_all_tags(
    pool: web::Data<Pool>,
) -> Result<HttpResponse, AppError> {
    // Consultar la base de datos para obtener todas las etiquetas
    let tags = crate::db::get_all_tags(&pool)
        .map_err(|_| AppError::new(AppErrorCode::DatabaseError))?;

    // Retornar una respuesta con todas las etiquetas
    Ok(HttpResponse::Ok().json(tags))
}

// Función para obtener detalles de una etiqueta por su ID
pub async fn get_tag_by_id(
    tag_id: web::Path<i32>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, AppError> {
    let tag_id = tag_id.into_inner();

    // Consultar la base de datos para obtener los detalles de una etiqueta por su ID
    let tag = crate::db::get_tag_by_id(&pool, tag_id)
        .map_err(|_| AppError::new(AppErrorCode::DatabaseError))?;

    // Retornar una respuesta con los detalles de la etiqueta
    Ok(HttpResponse::Ok().json(tag))
}

// Otras funciones para actualizar una etiqueta, eliminar una etiqueta por su ID, etc.
// Puedes agregar aquí las funciones específicas para tu API de etiquetas
