
use axum::Router;
use crate::config::Config;
use crate::db::DbPool;

pub struct App {
    router: Router,
    db_pool: DbPool,
}

impl App {
    pub async fn new() -> Self {
        let config = Config::load();
        let db_pool = DbPool::connect(config.database_url).await;
        let router = Router::new()
            .merge(routes::auth::router())
            .merge(routes::repos::router)
            .with_state(db_pool);

        Self { router, db_pool }
    }
}