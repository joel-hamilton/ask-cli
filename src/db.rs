use sqlx::SqlitePool;

use crate::chat::{ChatSession, Message};

pub struct DB {
    pool: SqlitePool,
}

impl DB {
    pub async fn new() -> Self {
        let pool = SqlitePool::connect(&std::env::var("DATABASE_URL").unwrap())
            .await
            .unwrap();
        DB { pool }
    }

    pub async fn migrate(&self) -> anyhow::Result<()> {
        sqlx::migrate!("./migrations").run(&self.pool).await?;
        Ok(())
    }

    pub async fn add_chat(
        &self,
        role: &str,
        content: &str,
        mut session_id: Option<i64>,
    ) -> anyhow::Result<i64> {
        if let Some(id) = session_id {
            sqlx::query!(
                r#"
UPDATE chat_sessions
SET updated_at = CURRENT_TIMESTAMP
WHERE id = ?1
        "#,
                id
            )
            .execute(&self.pool)
            .await?;
        } else {
            let name = "Untitled Session";
            let id = sqlx::query!(
                r#"
INSERT INTO chat_sessions
(name, updated_at, created_at)
VALUES
(?1, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
        "#,
                name
            )
            .execute(&self.pool)
            .await?
            .last_insert_rowid();

            session_id = Some(id);
        }

        sqlx::query!(
            r#"
INSERT INTO chat_messages
(session_id, role, content, created_at)
VALUES
(?1, ?2, ?3, CURRENT_TIMESTAMP)
        "#,
            session_id,
            role,
            content
        )
        .execute(&self.pool)
        .await?;

        Ok(session_id.unwrap())
    }

    pub async fn get_session(&self, session_id: i64) -> Result<Vec<Message>, sqlx::Error> {
        sqlx::query_as!(
            Message,
            r#"
SELECT role, content
FROM chat_messages
WHERE session_id = ?1
ORDER BY created_at
        "#,
        session_id
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn get_sessions(&self) -> Result<Vec<ChatSession>, sqlx::Error> {
        sqlx::query_as!(
            ChatSession,
            r#"
SELECT id, name
FROM chat_sessions
ORDER BY created_at
        "#
        )
        .fetch_all(&self.pool)
        .await
    }
}
