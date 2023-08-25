use std::fmt;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Sql {
    pub(crate) sql: String,
    pub(crate) params: Option<Vec<String>>,
}

impl Sql {
    fn create(sql: String, params: Option<Vec<String>>) -> Self {
        Sql { sql, params }
    }
}

impl fmt::Display for Sql {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.sql)
    }
}

#[cfg(test)]
mod tests {
    use crate::config;
    use rusqlite::{Connection, Result};
    use crate::mail::Email;

    #[test]
    fn sql() -> Result<()> {
        let mut path = String::from("C:\\Users\\su_ch\\AppData\\Roaming\\tauri-skeleton\\");
        path.push_str(config::DB_URL);
        let conn = Connection::open(path)?;

        println!("链接信息： {:?}", conn);

        let mut stmt = conn.prepare("SELECT id,address,password,type FROM email")?;
        let email_iter = stmt.query_map([], |row| {
            Ok(Email {
                id: row.get(0)?,
                address: row.get(1)?,
                password: row.get(2)?,
                kind: None,
                status: None,
            })
        })?;

        for email in email_iter {
            println!("邮箱： {:?}", email?);
        }

        Ok(())
    }

}
