use actix_web::{web, HttpResponse, Error};
use crate::dao;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTodo {
    pub title: String,
    pub description: Option<String>,
    pub image_name: Option<String>,
    pub image_data: Option<Vec<u8>>,
    pub extra: Option<serde_json::Value>,
}

pub async fn create_todo(
    db: web::Data<sea_orm::DatabaseConnection>,
    form: web::Json<CreateTodo>,
) -> Result<HttpResponse, Error> {
    let todo = dao::create_todo(
        &db, 
        form.title.clone(),
        form.description.clone(),
        form.image_name.clone(),
        form.image_data.clone(),
        form.extra.clone(),
    )
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(todo))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationParams {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: u64,
    pub page: u64,
    pub page_size: u64,
    pub total_pages: u64,
}

pub async fn get_todos(
    db: web::Data<sea_orm::DatabaseConnection>,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, Error> {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(10);
    
    let (todos, total) = dao::get_todos(&db, page, page_size)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(PaginatedResponse {
        data: todos,
        total,
        page,
        page_size,
        total_pages: (total as f64 / page_size as f64).ceil() as u64,
    }))
}

pub async fn get_todo_by_id(
    db: web::Data<sea_orm::DatabaseConnection>,
    id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let todo = dao::get_todo_by_id(&db, *id)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    match todo {
        Some(todo) => Ok(HttpResponse::Ok().json(todo)),
        None => Ok(HttpResponse::NotFound().finish()),
    }
}

pub async fn delete_todo(
    db: web::Data<sea_orm::DatabaseConnection>,
    id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    dao::delete_todo(&db, *id)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::NoContent().finish())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTodoCompleted {
    pub id: i32,
    pub completed: bool,
}

pub async fn update_todo_completed(
    db: web::Data<sea_orm::DatabaseConnection>,
    form: web::Json<UpdateTodoCompleted>,
) -> Result<HttpResponse, Error> {
    let updated_todo = dao::update_todo_completed(&db, form.id.clone(), form.completed)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(updated_todo))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCreateTodo {
    pub todos: Vec<CreateTodo>,
}

pub async fn batch_create_todos(
    db: web::Data<sea_orm::DatabaseConnection>,
    form: web::Json<BatchCreateTodo>,
) -> Result<HttpResponse, Error> {
    dao::batch_create_todos(&db, form.todos.clone())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
        
    Ok(HttpResponse::NoContent().finish())
}