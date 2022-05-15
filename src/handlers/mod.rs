use actix_web::{HttpResponse, web};
use actix_web::web::ServiceConfig;

pub fn app_config(config: &mut ServiceConfig){
    let health_resource = web::resource("/")
        .route(web::get().to(health_handler));
    config.service(health_resource);
}

pub async fn health_handler() -> HttpResponse{
    HttpResponse::Ok().finish()
}