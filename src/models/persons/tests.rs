use super::model::Person;
use super::control::{
    PersonControl,
    convert_any_as_u8_slice,
    convert_slice_u8_to_ref_struct,
};

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

    let bytes: &[u8] = convert_any_as_u8_slice(&pc);
    let rpc: &PersonControl = convert_slice_u8_to_ref_struct(bytes);
    assert_eq!(rpc.nonce, 34);

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
    let _res = client.lock().unwrap().execute("DELETE FROM deletes_persons WHERE person_id = $1", &[&id]).await;
    let _res = client.lock().unwrap().execute("DELETE FROM persons WHERE id = $1", &[&id]).await;
}