use serde::Serialize;
use tokio_postgres::{Error, GenericClient, Row};

#[derive(Debug, Serialize)]
pub struct User {
    pub id: i32,
    pub counter: i32,
}

impl From<Row> for User {
    fn from(row: Row) -> Self {
        Self {
            id: row.get(0),
            counter: row.get(1),
        }
    }
}


impl User {
    pub async fn all<C: GenericClient>(client: &C) -> Result<Vec<User>, Error> {
        let stmt = client.prepare("SELECT id, counter FROM users").await?;
        let rows = client.query(&stmt, &[]).await?;

        Ok(rows.into_iter().map(User::from).collect())
    }

    pub async fn incrementCounter<C: GenericClient>(client: &C) -> Result<Vec<User>, Error> {
        let stmt = client.prepare("UPDATE users SET counter = counter + 1 WHERE id = 1").await?;
        let rows = client.query(&stmt, &[]).await?;

        Ok(rows.into_iter().map(User::from).collect())
    }
}
