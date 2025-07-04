use anyhow::Result;
use async_trait::async_trait;
use derive_new::new;
use kernel::model::book::{Book, event::CreateBookEvent};
use kernel::repository::book::BookRepository;
use uuid::Uuid;

use crate::database::ConnectionPool;
use crate::database::model::book::BookRow;

#[derive(new)]
pub struct BookRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl BookRepository for BookRepositoryImpl {
    async fn create(&self, event: CreateBookEvent) -> Result<()> {
        sqlx::query!(
            r#"
                INSERT INTO books (title, author, isbn, description)
                VALUES ($1, $2, $3, $4)
            "#,
            event.title,
            event.author,
            event.isbn,
            event.description
        )
        .execute(self.db.inner_ref())
        .await?;
        Ok(())
    }
    async fn find_all(&self) -> Result<Vec<Book>> {
        let rows: Vec<BookRow> = sqlx::query_as!(
            BookRow,
            r#"
                SELECT book_id, title, author, isbn, description
                FROM books
                ORDER BY created_at DESC
            "#
        )
        .fetch_all(self.db.inner_ref())
        .await?;

        let books: Vec<Book> = rows.into_iter().map(Book::from).collect();
        Ok(books)
    }
    async fn find_by_id(&self, book_id: Uuid) -> Result<Option<Book>> {
        let row: Option<BookRow> = sqlx::query_as!(
            BookRow,
            r#"
                SELECT book_id, title, author, isbn, description
                FROM books
                WHERE book_id = $1
            "#,
            book_id
        )
        .fetch_optional(self.db.inner_ref())
        .await?;
        Ok(row.map(Book::from))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test]
    async fn test_register_book(pool: sqlx::PgPool) -> anyhow::Result<()> {
        let repository = BookRepositoryImpl::new(ConnectionPool::new(pool));
        let book = CreateBookEvent {
            title: "Test Title".into(),
            author: "Test Author".into(),
            isbn: "Test ISBN".into(),
            description: "Test Description".into(),
        };
        repository.create(book).await?;
        let books = repository.find_all().await?;
        assert_eq!(books.len(), 1);

        let book_id = books[0].id;
        let found_book = repository.find_by_id(book_id).await?;
        assert!(found_book.is_some());

        let Book {
            id,
            title,
            author,
            isbn,
            description,
        } = found_book.unwrap();
        assert_eq!(id, book_id);
        assert_eq!(title, "Test Title");
        assert_eq!(author, "Test Author");
        assert_eq!(isbn, "Test ISBN");
        assert_eq!(description, "Test Description");

        Ok(())
    }
}
