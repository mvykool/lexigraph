use sqlx::postgres::PgPool;

pub async fn init_db(db_url: &str) -> PgPool {
    PgPool::connect(db_url)
         .await
         expect("failed to connect to the database")
}
