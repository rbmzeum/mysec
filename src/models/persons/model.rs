use crate::modules::db::Store;
use super::error::{
    PersonError,
    ERR_PERSON_NOT_FOUND,
};

pub struct Person {
    pub id: i64,
    pub version: i64,
    pub data: Vec<u8>,
}

impl Person {
    pub async fn find(id: i64, store: &Store) -> Result<Self, Box<dyn std::error::Error>> {
        let result = store.actions.query("SELECT * FROM persons WHERE id = $1 ORDER BY version DESC LIMIT 1", &[&id]).await;
        match result {
            Ok(rows) => {
                if rows.len() > 0 {
                    let id: i64 = rows[0].get("id");
                    let version: i64 = rows[0].get("version");
                    let data: Vec<u8> = rows[0].get("data");
                    return Ok(Self {
                        id: id,
                        version: version,
                        data: data,
                    });
                }
                let e = PersonError{
                    code: ERR_PERSON_NOT_FOUND,
                    message: "Person not found".to_string(),
                };
                Err(Box::new(e))
            },
            Err(e) => {
                Err(Box::new(e))
            },
        }
    }

    pub async fn save(&self, store: &Store) -> bool {
        let result = store.actions.execute("INSERT INTO persons (id, version, data) VALUES ($1, $2, $3)", &[
            &self.id,
            &self.version,
            &self.data, // TODO: make dynamically use trust_chain
        ]).await;
        match result {
            Ok(_v) => true,
            Err(_e) => false,
        }
    }

    pub async fn delete(&self, store: &Store) -> bool {
        let data: Vec<u8> = vec![]; // TODO: make valid data with nonce, parent hash, current and control sum use trust_chain
        let result = store.actions.execute("INSERT INTO deletes_persons (person_id, data) VALUES ($1, $2)", &[
            &self.id,
            &data,
        ]).await;
        match result {
            Ok(_v) => true,
            Err(_e) => false,
        }
    }
}