use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::state::AppState;

type SqlID = u64;

#[derive(FromRow, Serialize, Deserialize, Clone, Debug)]
pub struct Score {
    pub id: SqlID,
    pub flag_id: SqlID,
    pub user_id: SqlID,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewScore {
    pub flag_id: SqlID,
    pub user_id: SqlID,
}

#[async_trait]
pub trait IScore {
    async fn score_add(&self, form: &NewScore) -> sqlx::Result<SqlID>;
    async fn score_all(&self) -> sqlx::Result<Vec<Score>>;
}

#[async_trait]
impl IScore for AppState {
    async fn score_add(&self, form: &NewScore) -> sqlx::Result<SqlID> {
        let id = sqlx::query!(
            r#"
        INSERT INTO score (flag_id, user_id)
        VALUES (?, ?);
                "#,
            form.flag_id,
            form.user_id,
        )
        .execute(&self.sql)
        .await?
        .last_insert_id();
        Ok(id)
    }

    async fn score_all(&self) -> sqlx::Result<Vec<Score>> {
        sqlx::query_as!(
            Score,
            r#"
        SELECT *
        FROM score
        ORDER BY id
            "#,
        )
        .fetch_all(&self.sql)
        .await
    }
}
