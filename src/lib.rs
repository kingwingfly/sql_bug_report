#![allow(dead_code)]

use sqlx::postgres::{PgPool, PgPoolOptions};
use tokio::sync::OnceCell;

const DATABASE_URL: &str = "postgres://postgres:postgres@localhost:5432/postgres";
static INIT: OnceCell<ModelManager> = OnceCell::const_new();

/// Init ModelManager for test, so that all the test func use the same ModelManager
async fn init_test() -> ModelManager {
    let mm = INIT
        .get_or_init(|| async {
            println!("INIT ModelManager");
            ModelManager::new().await
        })
        .await;
    mm.clone()
}

#[derive(Clone)]
struct ModelManager {
    // postgres connection pool
    pool: PgPool,
    another: String,
}

impl ModelManager {
    async fn new() -> Self {
        let pool = PgPoolOptions::new()
            .acquire_timeout(std::time::Duration::from_secs(2))
            .max_connections(4)
            .connect(DATABASE_URL)
            .await
            .unwrap();
        Self {
            pool,
            another: "".to_string(),
        }
    }

    fn pool(&self) -> PgPool {
        self.pool.clone()
    }
    fn another(&self) -> String {
        self.another.clone()
    }
}

// acqurie a connection from pool
async fn pool_op(mm: &ModelManager) {
    mm.pool().acquire().await.unwrap();
}

// do something on another field in ModelManager
async fn another_op(mm: &ModelManager) {
    mm.another();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test1() {
        let mm = init_test().await;
        pool_op(&mm).await;
    }

    #[tokio::test]
    async fn test2() {
        let mm = init_test().await;
        another_op(&mm).await;
    }
}
