use actix_web::web;
use crate::services;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/todos")
            .route("", web::post().to(services::create_todo))
            .route("", web::get().to(services::get_todos))
            .route("/{id}", web::get().to(services::get_todo_by_id))
            .route("/{id}", web::delete().to(services::delete_todo))
            .route("", web::put().to(services::update_todo_completed))
            .route("/batch", web::post().to(services::batch_create_todos)),
    );
}