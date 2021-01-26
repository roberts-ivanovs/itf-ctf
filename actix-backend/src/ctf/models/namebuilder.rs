use regex::Regex;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::{how::Error, state::AppState};

type SqlID = u64;

#[derive(FromRow, Serialize, Deserialize, Clone, Debug)]
pub struct Namebuilder {
    pub id: SqlID,
    pub name: String,
}

#[async_trait]
pub trait INameBuilder {
    async fn get_unique_name(&self) -> sqlx::Result<String>;
}

#[derive(FromRow)]
struct MaxId {
    count: i64,
}

#[async_trait]
impl INameBuilder for AppState {
    async fn get_unique_name(&self) -> sqlx::Result<String> {
        // Get noun
        let user_count = sqlx::query_as!(
            MaxId,
            r#"
        SELECT COUNT(*) as count
        FROM users
        ORDER BY id
            "#,
        )
        .fetch_one(&self.sql)
        .await?;

        let noun_count = sqlx::query_as!(
            MaxId,
            r#"
        SELECT COUNT(*) as count
        FROM nouns
        ORDER BY id
            "#,
        )
        .fetch_one(&self.sql)
        .await?;
        let next_noun_id = user_count.count % noun_count.count;
        let noun = sqlx::query_as!(
            Namebuilder,
            r#"
        SELECT*
        FROM nouns
        WHERE id = ?
        ORDER BY id
            "#,
            next_noun_id
        )
        .fetch_one(&self.sql)
        .await
        .unwrap_or(Namebuilder {
            id: 0,
            name: "Anonynmous".to_owned(),
        })
        .name;

        // Generate adjactives
        let adjactive_count = sqlx::query_as!(
            MaxId,
            r#"
        SELECT COUNT(*) as count
        FROM adjactives
        ORDER BY id
            "#,
        )
        .fetch_one(&self.sql)
        .await?;
        let mut adjacives = String::new();
        let mut ucnt = user_count.count;
        let mut next_adjactive_id = ucnt % adjactive_count.count;
        while next_adjactive_id > 0 {
            let adjactive = sqlx::query_as!(
                Namebuilder,
                r#"
                SELECT*
                FROM adjactives
                WHERE id = ?
                ORDER BY id
                "#,
                next_adjactive_id
            )
            .fetch_one(&self.sql)
            .await
            .unwrap_or(Namebuilder {
                id: 0,
                name: "Anonynmous".to_owned(),
            });
            ucnt -= next_adjactive_id;
            next_adjactive_id = ucnt % adjactive_count.count;
            adjacives += " ";
            adjacives += &adjactive.name;
        }

        Ok(format!("{} {} |{}", adjacives, noun, user_count.count))
    }
}
