use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::state::AppState;

type SqlID = u64;

#[derive(FromRow, Serialize, Deserialize, Clone, Debug)]
pub struct Flag {
    pub id: SqlID,
    pub name: String,
    pub answer: String,
    pub description: Option<String>,
    pub filepath: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewFlag {
    pub name: String,
    pub answer: String,
    pub description: Option<String>,
    pub filepath: Option<String>,
}

impl NewFlag {
    pub fn from_flag(flag: Flag) -> Self {
        Self {
            name: flag.name,
            answer: flag.answer,
            description: flag.description,
            filepath: flag.filepath,
        }
    }
}

#[derive(FromRow, Clone, Serialize, Deserialize, Debug)]
pub struct AnswerlessFlag {
    pub id: SqlID,
    pub name: String,
    pub description: Option<String>,
    pub filepath: Option<String>,
}

#[async_trait]
pub trait IFlag {
    async fn flag_add(&self, form: &NewFlag) -> sqlx::Result<SqlID>;
    async fn flag_update(&self, form: &NewFlag, old_id: &u64) -> sqlx::Result<()>;
    async fn flag_query(&self, id: SqlID) -> sqlx::Result<Flag>;
    async fn flag_query_without_answer(&self, id: SqlID) -> sqlx::Result<AnswerlessFlag>;
    async fn flag_all_without_answer(&self) -> sqlx::Result<Vec<AnswerlessFlag>>;
    async fn flag_all(&self) -> sqlx::Result<Vec<Flag>>;
}

#[async_trait]
impl IFlag for AppState {
    async fn flag_add(&self, form: &NewFlag) -> sqlx::Result<SqlID> {
        let id = sqlx::query!(
            r#"
        INSERT INTO flag (name, answer, filepath, description)
        VALUES (?, ?, ?, ?);
                "#,
            form.name,
            form.answer,
            form.filepath,
            form.description,
        )
        .execute(&self.sql)
        .await?
        .last_insert_id();
        Ok(id)
    }

    async fn flag_update(&self, form: &NewFlag, old_id: &u64) -> sqlx::Result<()> {
        sqlx::query!(
            r#"
        UPDATE flag SET name = ?, answer = ?, filepath = ?, description = ? WHERE id = ?;
                "#,
            form.name,
            form.answer,
            form.filepath,
            form.description,
            old_id
        )
        .execute(&self.sql)
        .await?;
        Ok(())
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

    async fn flag_query_without_answer(&self, id: SqlID) -> sqlx::Result<AnswerlessFlag> {
        sqlx::query_as!(
            AnswerlessFlag,
            r#"
        SELECT id, name, filepath, description
        FROM flag
        where id = ?
                "#,
            id
        )
        .fetch_one(&self.sql)
        .await
    }

    async fn flag_all_without_answer(&self) -> sqlx::Result<Vec<AnswerlessFlag>> {
        sqlx::query_as!(
            AnswerlessFlag,
            r#"
        SELECT id, name, filepath, description
        FROM flag
        ORDER BY id
            "#,
        )
        .fetch_all(&self.sql)
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
