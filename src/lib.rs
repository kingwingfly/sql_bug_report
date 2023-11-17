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

    async fn pool(&self) {
        self.pool.acquire().await.unwrap();
    }
    async fn another(&self) {
        println!("{}", self.another);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test1() {
        let mm = init_test().await;
        mm.pool().await;
    }

    #[tokio::test]
    async fn test2() {
        let mm = init_test().await;
        mm.another().await;
    }
}
