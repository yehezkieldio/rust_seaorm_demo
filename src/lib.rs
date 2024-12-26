use dotenv::dotenv;
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DbConn, DbErr};

pub async fn connect() -> Result<DbConn, DbErr> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").unwrap();
    let db = Database::connect(&database_url)
        .await
        .expect("Failed to connect to the database");
    Migrator::up(&db, None)
        .await
        .expect("Failed to run migrations for tests");

    Ok(db)
}
