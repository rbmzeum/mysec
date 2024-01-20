use crate::modules::db::Store;
use crate::secure_invite::{
    keypair::KeyPair,
    // crypt::Crypt,
};

const ERR_PERSON_NOT_FOUND: u8 = 1;

#[derive(Debug)]
struct PersonError {
    code: u8,
    message: String,
}

impl std::fmt::Display for PersonError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Code: {}, Message: {}", self.code, self.message)
    }
}

impl std::error::Error for PersonError {}

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::core::slice::from_raw_parts(
        (p as *const T) as *const u8,
        ::core::mem::size_of::<T>(),
    )
}

pub struct PersonControl {
    pub nonce: u64,
    // pub control_sum: [u8; 56], // TODO: SHA-3 (Keccak) size
    // pub parent_control_sum: [u8; 56],
    // pub key_pair: KeyPair, // serde не подходит, надо другим способом
}

pub struct Person {
    pub id: i64,
    pub version: i64,
    pub data: Vec<u8>,
}

impl Person {
    async fn find(id: i64, store: &Store) -> Result<Self, Box<dyn std::error::Error>> {
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

    async fn save(&self, store: &Store) -> bool {
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

    async fn delete(&self, store: &Store) -> bool {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_save_find_delete() {
        let db_store = crate::modules::db::Store::new("host=localhost user=postgres sslmode=require dbname=mysec").await;

        let mut p = Person{
            id: -1,
            version: 1,
            data: vec![],
        };

        let pc = PersonControl {
            nonce: 34,
        };

        let bytes: &[u8] = unsafe { any_as_u8_slice(&pc) };
        p.data = Vec::from(bytes); // TODO: добавлять через метод Person: p.make_control_data(parent_control_sum); // и спроектировать связь с trust_chain

        let res = p.save(&db_store).await;
        assert_eq!(res, true);

        let result = Person::find(-1, &db_store).await;
        match result {
            Ok(person) => {
                assert_eq!(person.id, -1);
            },
            Err(e) => {
                // assert_eq!(ERR_PERSON_NOT_FOUND, e.code);
                dbg!(e);
                assert!(false);
            },
        }

        let res = p.delete(&db_store).await;
        assert!(res);

        // почистить от тестовых данных
        let client = db_store.getters.get_client();
        let id: i64 = -1;
        let _res = client.lock().unwrap().execute("DELETE FROM deletes_persons WHERE id = $1", &[&id]).await;
        let _res = client.lock().unwrap().execute("DELETE FROM persons WHERE id = $1", &[&id]).await;
    }
}