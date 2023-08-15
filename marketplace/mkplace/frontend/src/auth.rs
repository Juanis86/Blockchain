// En el archivo controllers/auth_controller.rs
use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use diesel::result::Error;
use crate::models::User;
use crate::schema::users;

pub async fn login(
    user_data: web::Json<User>,
    db: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let user = user_data.into_inner();
    let conn = db.get().expect("Error getting db connection");

    let result = users::table
        .filter(users::email.eq(&user.email))
        .filter(users::password.eq(&user.password))
        .first::<User>(&conn);

    match result {
        Ok(_) => Ok(HttpResponse::Ok().json("Login successful")),
        Err(_) => Ok(HttpResponse::Unauthorized().json("Invalid credentials")),
    }
}