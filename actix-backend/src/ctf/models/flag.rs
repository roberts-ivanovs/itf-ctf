use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::state::AppState;

type SqlID = u64;

#[derive(FromRow, Serialize, Deserialize, Clone, Debug)]
pub struct Flag {
    pub id: SqlID,
    pub name: String,
    pub answer: String,
    pub filepath: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewFlag {
    pub name: String,
    pub answer: String,
    pub filepath: String,
}

#[async_trait]
pub trait IFlag {
    async fn flag_add(&self, form: &NewFlag) -> sqlx::Result<SqlID>;
    async fn flag_query(&self, id: SqlID) -> sqlx::Result<Flag>;
    async fn flag_all(&self) -> sqlx::Result<Vec<Flag>>;
}

#[async_trait]
impl IFlag for AppState {
    async fn flag_add(&self, form: &NewFlag) -> sqlx::Result<SqlID> {
        let id = sqlx::query!(
            r#"
        INSERT INTO flag (name, answer, filepath)
        VALUES (?, ?, ?);
                "#,
            form.name,
            form.answer,
            form.filepath,
        )
        .execute(&self.sql)
        .await?
        .last_insert_id();
        Ok(id)
    }

    async fn flag_query(&self, id: SqlID) -> sqlx::Result<Flag> {
        sqlx::query_as!(
            Flag,
            r#"
        SELECT *
        FROM flag
        where id = ?
                "#,
            id
        )
        .fetch_one(&self.sql)
        .await
    }

    async fn flag_all(&self) -> sqlx::Result<Vec<Flag>> {
        sqlx::query_as!(
            Flag,
            r#"
        SELECT *
        FROM flag
        ORDER BY id
            "#,
        )
        .fetch_all(&self.sql)
        .await
    }
}
