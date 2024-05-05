// src/db/db_access_trait.rs

pub trait DatabaseAccess {
    async fn ping(&self) -> Result<(), sqlx::Error>;
    // Tutaj dodaj inne metody, np. do pobierania danych, ich wstawiania itp.
}
