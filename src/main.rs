use actix_web::{post, web::Json, App, HttpResponse, HttpServer};
use serde::Deserialize;
use validator::Validate;

#[allow(unused)]
#[derive(Deserialize, Validate)]
struct User {
    #[validate(length(min = 5, max = 10))]
    #[serde(rename = "userName")]
    user_name: String,
    #[validate(email)]
    email: String,
    #[validate(range(min = 18))]
    age: i32
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
    .bind(("0.0.0.0", 4000)).expect("Failed to bind server")
    .run()
    .await
}

// use actix_web::{post, web::Json, App, HttpResponse, HttpServer};
// use serde::Deserialize;
// use validator::Validate;

// #[allow(unused)]
// #[derive(Deserialize, Debug, Validate)]
// struct User {
//     #[validate(length(min = 5, max = 15))]
//     #[serde(rename = "userName")]
//     user_name: String,
//     #[validate(length(min = 8, max = 15))]
//     #[serde(rename = "fullName")]
//     full_name: String,
//     #[validate(range(min = 18))]
//     age: i32,
//     #[validate(email)]
//     email: String
// }

// #[post("/new")]
// async fn new_user(user: Json<User>) -> HttpResponse {
//     match user.0.validate() {
//         Ok(_) => {
//             println!("user: {:?}", user);

//             HttpResponse::Created().into()
//         },
//         Err(e) => HttpResponse::InternalServerError().json(e.errors())
//     }    
// }

// #[tokio::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .service(new_user)
//     })
//     .bind(("0.0.0.0", 4000)).expect("Failed to bind to host and port")
//     .run()
//     .await
// }