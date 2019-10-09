pub mod types {
    pub type DatabaseConnection = diesel::PgConnection;
    pub type DatabaseManager = diesel::r2d2::ConnectionManager<DatabaseConnection>;
    pub type DatabasePool = r2d2::Pool<DatabaseManager>;
}