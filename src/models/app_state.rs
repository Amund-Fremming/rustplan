use sqlx::{Pool, Postgres};

pub struct AppState {
    db_pool: Pool<Postgres>,
}

impl AppState {
    pub async fn connect_pool(connection_string: &str) -> Result<Self, sqlx::Error> {
        let db_pool = Pool::<Postgres>::connect(connection_string).await?;

        Ok(Self { db_pool })
    }

    pub fn get_pool(&self) -> &Pool<Postgres> {
        &self.db_pool
    }
}
