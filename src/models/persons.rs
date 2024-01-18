use crate::modules::db::Store;

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

pub struct Person {
    pub id: i8,
    pub version: i8,
    pub data: Vec<u8>,
}

impl Person {
    async fn find(id: i8, store: &Store) -> Result<Self, Box<dyn std::error::Error>> {
        let result = store.actions.query("SELECT * FROM persons WHERE id = $1 ORDER BY versions DESC LIMIT 1", &[&id]).await;
        match result {
            Ok(rows) => {
                if rows.len() > 0 {
                    let id: i8 = rows[0].get("id");
                    let version: i8 = rows[0].get("version");
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_find() {
        let db_store = crate::modules::db::Store::new("host=localhost user=postgres sslmode=require dbname=mysec").await;
        let result = Person::find(1, &db_store).await;
        match result {
            Ok(person) => {
                assert_eq!(person.id, 1);
            },
            Err(e) => {
                // assert_eq!(ERR_PERSON_NOT_FOUND, e.code);
            },
        }
    }

}