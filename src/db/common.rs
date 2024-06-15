use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use rocket::{Build, Rocket};
use rocket_sync_db_pools::{database, diesel};

#[database("rocket_template_db")]
pub struct ConnectionPool(diesel::SqliteConnection);

pub async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("src/migrations");

    ConnectionPool::get_one(&rocket)
        .await
        .expect("Failed to get a database connection")
        .run(|c| {
            let migration_result = MigrationHarness::run_pending_migrations(c, MIGRATIONS);
            match migration_result {
                Ok(applied_migrations) => {
                    for migration in applied_migrations {
                        println!("Applied migration: {}", migration);
                    }
                }
                Err(err) => {
                    eprintln!("Error applying migrations: {}", err);
                }
            }
        })
        .await;

    rocket
}
