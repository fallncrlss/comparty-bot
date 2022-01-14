use crate::lib::helpers::get_envvar;
use {sqlx, std};

pub async fn new_pg_pool() -> sqlx::PgPool {
    let options = sqlx::postgres::PgConnectOptions::new()
        .host(&*get_envvar("POSTGRES_HOST"))
        .port((&*get_envvar("POSTGRES_PORT")).parse().unwrap_or_else(|err| {
            panic!("{:?}", err);
        }))
        .username(&*get_envvar("POSTGRES_USER"))
        .password(&*get_envvar("POSTGRES_PASSWORD"))
        .database(&*get_envvar("POSTGRES_DB"))
        .to_owned();

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect_timeout(std::time::Duration::from_secs(1))
        .connect_with(options)
        .await
        .expect("Unable to connect to DB");

    return pool;
}

pub async fn run_pg_migrations() {
    let pool = new_pg_pool().await;
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Unable to run migrations");
}
