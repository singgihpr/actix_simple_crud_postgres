use actix_web::{delete, get, patch, post, web, ResponseError, http::StatusCode, HttpResponse};
use sqlx::PgPool;
use sqlx::Error as SqlxError;
use std::fmt;
use crate::models::NewUser;
use crate::models::UpdateUser;
use crate::models::User; 

#[derive(Debug)]
pub enum AppError {
    DatabaseError(SqlxError),
    NotFound, // Contoh error lain
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::DatabaseError(err) => write!(f, "Database error: {}", err),
            AppError::NotFound => write!(f, "Resource not found"),
            // ... error lainnya
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::DatabaseError(err) => {
                // Log error di sini (sangat disarankan)
                eprintln!("Database error: {}", err);  // Contoh logging sederhana

                HttpResponse::InternalServerError().json(format!("Database error")) // Pesan ke client bisa disesuaikan
            }
            AppError::NotFound => HttpResponse::NotFound().body("Resource not found"),
            // ... error lainnya
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound => StatusCode::NOT_FOUND,
            // ...
        }
    }
}

#[post("/users")]
async fn create_user(
    pool: web::Data<PgPool>,
    user: web::Json<NewUser>,
) -> Result<HttpResponse, AppError> {
    let result: User = sqlx::query_as::<_, User>(
        "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id, name, email",
    )
    .bind(&user.name)
    .bind(&user.email)
    .fetch_one(&**pool)
    .await
    .map_err(AppError::DatabaseError)?; // Konversi error sqlx::Error ke AppError

    Ok(HttpResponse::Ok().json(result))
}

#[get("/users")]
async fn get_users(pool: web::Data<PgPool>) -> Result<HttpResponse, AppError> {
    let result = sqlx::query_as::<_, User>("SELECT id, name, email FROM users")
        .fetch_all(&**pool)
        .await
        .map_err(AppError::DatabaseError)?;

    Ok(HttpResponse::Ok().json(result))
}

#[get("/users/{id}")]
async fn get_user_by_id(
    pool: web::Data<PgPool>,
    id: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let result = sqlx::query_as::<_, User>("SELECT id, name, email FROM users WHERE id = $1")
        .bind(*id)
        .fetch_optional(&**pool)
        .await
        .map_err(AppError::DatabaseError)?;

    match result {
        Some(user) => Ok(HttpResponse::Ok().json(user)),
        None => Err(AppError::NotFound), // Sekarang NotFound digunakan
    }
}

#[patch("/users/{id}")]
async fn update_user(
    pool: web::Data<PgPool>,
    id: web::Path<i32>,
    user: web::Json<UpdateUser>,
) -> Result<HttpResponse, AppError> {
    let user = sqlx::query_as::<_, User>(
        "UPDATE users SET name = $1, email = $2 WHERE id = $3 RETURNING id, name, email",
    )
    .bind(&user.name)
    .bind(&user.email)
    .bind(*id)
    .fetch_one(&**pool)
    .await
    .map_err(AppError::DatabaseError)?;

    Ok(HttpResponse::Ok().json(user))
}

#[delete("/users/{id}")]
async fn delete_user(pool: web::Data<PgPool>, id: web::Path<i32>) -> Result<HttpResponse, AppError> {
    sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(*id)
        .execute(&**pool)
        .await
        .map_err(AppError::DatabaseError)?;

    Ok(HttpResponse::Ok().finish())
}

// FOR TESTING ONLY
// Input 10000 data to database
#[post("/insert")]
async fn insert_10k_user(
    pool: web::Data<PgPool>
) -> Result<HttpResponse, AppError> {
    for _n in 0..10000 {
        let _result: User = sqlx::query_as::<_, User>(
            "INSERT INTO users (name, email) VALUES ('userGG', 'gg@mail.com') RETURNING id, name, email",
        )
        .fetch_one(&**pool)
        .await
        .map_err(AppError::DatabaseError)?;
    }

    Ok(HttpResponse::Ok().json("ok"))
}
