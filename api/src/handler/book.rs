use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use registry::AppRegistry;
use shared::error::AppError;
use uuid::Uuid;

use crate::model::book::{BookResponse, CreateBookRequest};

pub async fn register_book(
    State(registry): State<AppRegistry>,
    Json(request): Json<CreateBookRequest>,
) -> Result<StatusCode, AppError> {
    registry
        .book_repository()
        .create(request.into())
        .await
        .map(|_| StatusCode::CREATED)
}

pub async fn show_book_list(
    State(registry): State<AppRegistry>,
) -> Result<Json<Vec<BookResponse>>, AppError> {
    registry
        .book_repository()
        .find_all()
        .await
        .map(|books| {
            books
                .into_iter()
                .map(BookResponse::from)
                .collect::<Vec<_>>()
        })
        .map(Json)
}

pub async fn show_book(
    State(registry): State<AppRegistry>,
    Path(book_id): Path<Uuid>,
) -> Result<Json<BookResponse>, AppError> {
    registry
        .book_repository()
        .find_by_id(book_id.into())
        .await
        .and_then(|book| match book {
            Some(book) => Ok(Json(book.into())),
            None => Err(AppError::EntityNotFound(
                "The specific book was not found.".into(),
            )),
        })
}
