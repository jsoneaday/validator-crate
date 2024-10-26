use actix_web::{post, web::Json, App, HttpResponse, HttpServer};
use serde::Deserialize;
use validator::{Validate, ValidationError};

#[allow(unused)]
#[derive(Deserialize, Debug, Validate)]
struct User {
    #[validate(length(min = 5, max = 10))]
    user_name: String,
    #[validate(length(min = 10, max = 20), custom(function = "is_valid_password"))]
    password: String,
    #[validate(email)]
    email: String,
    #[validate(range(min = 18))]
    age: i32
}

fn is_valid_password(password: &str) -> Result<(), ValidationError> {
    let has_numbers = regex::Regex::new(r"\d").unwrap();
    let has_symbols = regex::Regex::new(r#"[!@#\$%\^&\*\(\),\.\?\":{}|<>]"#).unwrap();

    match has_numbers.is_match(password) && has_symbols.is_match(password) {
        true => Ok(()),
        false => Err(ValidationError::new("Password is invalid"))
    }
}

#[post("/new_user")]
async fn new_user(user: Json<User>) -> HttpResponse {
    match user.0.validate() {
        Ok(_) => HttpResponse::Created().into(),
        Err(e) => HttpResponse::InternalServerError().json(e.errors())
    }    
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(new_user)
    })
    .bind(("0.0.0.0", 4000)).expect("Failed to bind")
    .run()
    .await
}