// En el archivo controllers/user_controller.rs
use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use diesel::result::Error;
use crate::models::User;
use crate::schema::users;

pub async fn register_user(
    user_data: web::Json<User>,
    db: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let new_user = user_data.into_inner();
    let conn = db.get().expect("Error getting db connection");

    let result = diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&conn);

    match result {
        Ok(_) => Ok(HttpResponse::Created().json("User registered successfully")),
        Err(_) => Ok(HttpResponse::InternalServerError().json("Failed to register user")),
    }
}

