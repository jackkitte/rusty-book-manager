use kernel::model::book::{Book, event::CreateBookEvent};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBookRequest {
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
}

impl From<CreateBookRequest> for CreateBookEvent {
    fn from(request: CreateBookRequest) -> Self {
        let CreateBookRequest {
            title,
            author,
            isbn,
            description,
        } = request;
        CreateBookEvent {
            title,
            author,
            isbn,
            description,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BookResponse {
    pub id: Uuid,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
}

impl From<Book> for BookResponse {
    fn from(book: Book) -> Self {
        let Book {
            id,
            title,
            author,
            isbn,
            description,
        } = book;
        BookResponse {
            id,
            title,
            author,
            isbn,
            description,
        }
    }
}
