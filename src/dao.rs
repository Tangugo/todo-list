use crate::model::{self, Entity as Todo};
use sea_orm::{DatabaseConnection, EntityTrait, Set};
use sea_orm::ActiveValue;
use sea_orm::ActiveModelTrait;
use sea_orm::PaginatorTrait;
use crate::services::CreateTodo;

pub async fn create_todo(
    db: &DatabaseConnection,
    title: String,
    description: Option<String>,
    image_name: Option<String>,
    image_data: Option<Vec<u8>>,
    extra: Option<serde_json::Value>,
) -> Result<model::Model, sea_orm::DbErr> {
    let new_todo = model::ActiveModel {
        title: Set(title),
        description: Set(description),
        completed: Set(false),
        image_name: Set(image_name.unwrap_or_default()),
        image_data: Set(image_data.unwrap_or_default()),
        extra: Set(extra.unwrap_or_default()),
        ..Default::default()
    };
    let result = Todo::insert(new_todo).exec_with_returning(db).await?;
    Ok(result.into())
}

pub async fn get_todos(
    db: &DatabaseConnection,
    page: u64,
    page_size: u64,
) -> Result<(Vec<model::Model>, u64), sea_orm::DbErr> {
    let paginator = Todo::find()
        .paginate(db, page_size);
    let total = paginator.num_items().await?;
    let todos = paginator
        .fetch_page(page - 1) // SeaORM的页码从0开始
        .await?;
    
    Ok((todos, total))
}

pub async fn get_todo_by_id(
    db: &DatabaseConnection,
    id: i32,
) -> Result<Option<model::Model>, sea_orm::DbErr> {
    Todo::find_by_id(id).one(db).await
}

pub async fn delete_todo(db: &DatabaseConnection, id: i32) -> Result<(), sea_orm::DbErr> {
    let todo = Todo::find_by_id(id).one(db).await?;
    if let Some(todo) = todo {
        Todo::delete_by_id(todo.id).exec(db).await?;
    }
    Ok(())
}

pub async fn update_todo_completed(
    db: &DatabaseConnection,
    id: i32,
    completed: bool,
) -> Result<model::Model, sea_orm::DbErr> {
    let todo = Todo::find_by_id(id).one(db).await?;
    if let Some(todo) = todo {
        let mut active_model: model::ActiveModel = todo.into();
        active_model.completed = ActiveValue::Set(completed);
        Ok(active_model.update(db).await?)
    } else {
        Err(sea_orm::DbErr::RecordNotFound(format!("Todo with id {} not found", id)))
    }
}

pub async fn batch_create_todos(
    db: &DatabaseConnection,
    todos: Vec<CreateTodo>,
) -> Result<(), sea_orm::DbErr> {
    let active_models = todos.into_iter().map(|todo| {
        model::ActiveModel {
            title: Set(todo.title),
            description: Set(todo.description),
            completed: Set(false),
            image_name: Set(todo.image_name.unwrap_or_default()),
            image_data: Set(todo.image_data.unwrap_or_default()),
            extra: Set(todo.extra.unwrap_or_default()),
            ..Default::default()
        }
    }).collect::<Vec<_>>();

    Todo::insert_many(active_models)
        .exec(db)
        .await?;
    
    Ok(())
}