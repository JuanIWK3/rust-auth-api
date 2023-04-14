pub mod models {
    use serde::{Deserialize, Serialize};
    use tokio_pg_mapper_derive::PostgresMapper;

    #[derive(Deserialize, Serialize, PostgresMapper)]
    #[pg_mapper(table = "users")]
    pub struct User {
        pub email: String,
    }
}
