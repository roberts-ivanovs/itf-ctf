use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::state::AppState;

type SqlID = u64;

#[derive(FromRow, Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub id: SqlID,
    pub email: String,
    pub name: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Register {
    pub email: String,
}

#[async_trait]
pub trait IUser {
    async fn user_add(&self, form: &Register, name: String) -> sqlx::Result<SqlID>;
    async fn user_query(&self, id: SqlID) -> sqlx::Result<User>;
    async fn user_email_exists(&self, email: String) -> sqlx::Result<bool>;
    async fn user_all(&self) -> sqlx::Result<Vec<User>>;
}

struct Exists {
    count: i64
}

#[async_trait]
impl IUser for AppState {
    async fn user_add(&self, form: &Register, name: String) -> sqlx::Result<SqlID> {
        let id = sqlx::query!(
            r#"
        INSERT INTO users (email, name)
        VALUES (?, ?);
                "#,
            form.email, name
        )
        .execute(&self.sql)
        .await?
        .last_insert_id();
        Ok(id)
    }

    async fn user_query(&self, id: SqlID) -> sqlx::Result<User> {
        sqlx::query_as!(
            User,
            r#"
        SELECT *
        FROM users
        where id = ?
                "#,
            id
        )
        .fetch_one(&self.sql)
        .await
    }

    async fn user_all(&self) -> sqlx::Result<Vec<User>> {
        sqlx::query_as!(
            User,
            r#"
        SELECT *
        FROM users
        ORDER BY id
            "#,
        )
        .fetch_all(&self.sql)
        .await
    }


    async fn user_email_exists(&self, email: String) -> sqlx::Result<bool> {
        let res = sqlx::query_as!(
            Exists,
            r#"
        SELECT COUNT(*) as count
        FROM users
        where email = ?
                "#,
            email
        )
        .fetch_one(&self.sql)
        .await?;
        Ok(res.count > 0)
    }
}
