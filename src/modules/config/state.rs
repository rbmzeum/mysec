use config::*;
use std::collections::BTreeMap;

pub struct State {
    pub tree_map: BTreeMap<String, String>,
}

impl State {
    pub fn new(name: &str) -> State {
        let settings = Config::builder()
            .add_source(config::File::new(("configs/".to_string() + name).as_str(), FileFormat::Json))
            .build()
            .unwrap();

        State {
            tree_map: settings
            .try_deserialize::<BTreeMap<String, String>>()
            .unwrap(),
        }
    }
}