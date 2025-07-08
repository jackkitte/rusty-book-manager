use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use registry::AppRegistry;
use thiserror::Error;
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
        .map_err(AppError::from)
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
        .map_err(AppError::from)
}

pub async fn show_book(
    State(registry): State<AppRegistry>,
    Path(book_id): Path<Uuid>,
) -> Result<Json<BookResponse>, AppError> {
    registry
        .book_repository()
        .find_by_id(book_id)
        .await
        .and_then(|book| match book {
            Some(book) => Ok(Json(book.into())),
            None => Err(anyhow::anyhow!("The specific book was not found.")),
        })
        .map_err(AppError::from)
}
