use crate::routes::echo;
use crate::routes::hello;
use crate::routes::login;
use crate::routes::manual_hello;
use crate::routes::new_user;
//use actix_web::dev::ServiceRequest;
//use actix_web::service::ServiceRequest;
use actix_web::Error;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web_httpauth::extractors::basic::BasicAuth;

use actix_web_httpauth::middleware::HttpAuthentication;
use ccount_service::establish_pooled_connection;

mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server is running on port: 8080");
    let pool = establish_pooled_connection();
    HttpServer::new(move || {
        //let auth = HttpAuthentication::basic(basic_auth_validator);
        App::new()
            .service(hello)
            .service(echo)
            .service(new_user)
            .service(login)
            .data(pool.clone())
            // .route("/user/new", web::get().to(new_user))
            //.service(web::resource("/user/new").route(web::post().to_async(new_user)))
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

// async fn basic_auth_validator(
//     req: ServiceRequest,
//     _credentials: BasicAuth,
// ) -> Result<ServiceRequest, Error> {
//     Ok(req)
// }

// fn validate_credentials(user_id: &str, user_password: &str) -> Result<bool, std::io::Error> {
//     // Basic Auth (Username, Password)
//     if user_id.eq("abc") && user_password.eq("123") {
//         return Ok(true);
//     }
//     return Err(std::io::Error::new(
//         std::io::ErrorKind::Other,
//         "Authentication failed!",
//     ));
// }

// async fn basic_auth_validator(
//     req: ServiceRequest,
//     credentials: BasicAuth,
// ) -> Result<ServiceRequest, Error> {
//     let config = req
//         .app_data::<Config>()
//         .map(|data| data.get_ref().clone())
//         .unwrap_or_else(Default::default);
//     match validate_credentials(
//         credentials.user_id(),
//         credentials.password().unwrap().trim(),
//     ) {
//         Ok(res) => {
//             if res == true {
//                 Ok(req)
//             } else {
//                 Err(AuthenticationError::from(config).into())
//             }
//         }
//         Err(_) => Err(AuthenticationError::from(config).into()),
//     }
// }

// async fn basic_auth_validator(
//     req: ServiceRequest,
//     credentials: BasicAuth,
// ) -> Result<ServiceRequest, Error> {
//     let config = req
//         .app_data::<Config>()
//         .map(|data| data.get_ref().clone())
//         .unwrap_or_else(Default::default);
//     match validate_credentials(
//         credentials.user_id(),
//         credentials.password().unwrap().trim(),
//     ) {
//         Ok(res) => {
//             if res == true {
//                 Ok(req)
//             } else {
//                 Err(AuthenticationError::from(config).into())
//             }
//         }
//         Err(_) => Err(AuthenticationError::from(config).into()),
//     }
// }
