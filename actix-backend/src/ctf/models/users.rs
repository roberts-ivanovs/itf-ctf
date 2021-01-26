use regex::Regex;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::{how::Error, state::AppState};

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
    async fn user_add(&self, form: &Register, name: String) -> Result<SqlID, Error>;
    async fn user_query(&self, id: SqlID) -> sqlx::Result<User>;
    async fn user_by_email(&self, email: String) -> sqlx::Result<User>;
    async fn user_all(&self) -> sqlx::Result<Vec<User>>;
}

struct Exists {
    count: SqlID
}

lazy_static! {
    static ref EMAIL_REGEX: Regex = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();
}

#[async_trait]
impl IUser for AppState {
    async fn user_add(&self, form: &Register, name: String) -> Result<SqlID, Error> {

        info!("Attempting to create a new user {:#?}", form);
        if EMAIL_REGEX.is_match(&form.email) {
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
        } else {
            Err(Error::EmailError)
        }
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


    async fn user_by_email(&self, email: String) -> sqlx::Result<User> {
        let res = sqlx::query_as!(
            User,
            r#"
        SELECT *
        FROM users
        where email = ?
                "#,
            email
        )
        .fetch_one(&self.sql)
        .await?;
        Ok(res)
    }
}


#[cfg(test)]
mod test{

    use super::*;

    #[test]
    fn email_regex_correct() {
        let email1 = "s8_ivanov_r@venta.lv";
        let res = EMAIL_REGEX.is_match(email1);
        assert!(res);
    }

    #[test]
    fn email_regex_incorrect() {
        let email1 = "s8_ivanov_r.venta.lv";
        let res = EMAIL_REGEX.is_match(email1);
        assert!(!res);
    }
}
