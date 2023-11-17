use sqlx::postgres::{PgPool, PgPoolOptions};
use std::sync::Arc;
use tokio::sync::OnceCell;
use tokio::sync::RwLock;

const DATABASE_URL: &str = "postgres://postgres:postgres@localhost:5432/postgres";
static INIT: OnceCell<ModelManager> = OnceCell::const_new();

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
    pool: PgPool,
    another: Arc<RwLock<String>>,
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
            another: Arc::new(RwLock::new("".to_string())),
        }
    }

    fn pool(&self) -> &PgPool {
        &self.pool
    }
    fn another(&self) -> Arc<RwLock<String>> {
        self.another.clone()
    }
}

async fn pool_op(mm: &ModelManager) {
    mm.pool().acquire().await.unwrap();
}

async fn another_op(mm: &ModelManager) {
    mm.another().write().await.push_str("hello");
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
