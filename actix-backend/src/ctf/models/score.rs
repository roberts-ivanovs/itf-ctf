use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow};

use crate::state::AppState;

type SqlID = u64;

#[derive(FromRow, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Score {
    pub id: SqlID,
    pub flag_id: SqlID,
    pub user_id: SqlID,
    pub created_at: chrono::DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewScore {
    pub flag_id: SqlID,
    pub user_id: SqlID,
}

#[async_trait]
pub trait IScore {
    async fn score_add(&self, form: &NewScore) -> sqlx::Result<SqlID>;
    async fn score_all(&self) -> sqlx::Result<Vec<Score>>;
    async fn score_for_user(&self, user_id: SqlID) -> sqlx::Result<Vec<Score>>;
    async fn score_for_flag(&self, falg_id: SqlID) -> sqlx::Result<Vec<Score>>;
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

    async fn score_for_user(&self, user_id: SqlID) -> sqlx::Result<Vec<Score>> {
        sqlx::query_as!(
            Score,
            r#"
        SELECT *
        FROM score
        WHERE user_id = ?
        ORDER BY id
            "#,
            user_id,
        )
        .fetch_all(&self.sql)
        .await
    }

    async fn score_for_flag(&self, user_id: SqlID) -> sqlx::Result<Vec<Score>> {
        sqlx::query_as!(
            Score,
            r#"
        SELECT *
        FROM score
        WHERE flag_id = ?
        ORDER BY id
            "#,
            user_id,
        )
        .fetch_all(&self.sql)
        .await
    }
}
