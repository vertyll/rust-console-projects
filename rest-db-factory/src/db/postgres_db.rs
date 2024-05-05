// src/db/postgres_db.rs

use super::db_access_trait::DatabaseAccess;
use sqlx::postgres::PgPool;

pub struct PostgresDB {
    pool: PgPool,
}

impl PostgresDB {
    pub fn new(pool: PgPool) -> Self {
        PostgresDB { pool }
    }
}

impl DatabaseAccess for PostgresDB {
    async fn ping(&self) -> Result<(), sqlx::Error> {
        sqlx::query("SELECT 1")
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    // Tutaj dodaj implementacje innych metod
}
