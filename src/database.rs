use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use tinydb::Database;

#[derive(Debug, Eq, PartialEq, Hash, Serialize, Deserialize, Clone)]
pub struct WhiteList {
    code: String,
}

pub struct StockDB {}

impl StockDB {
    pub fn new() {
        let _: Database<WhiteList> = Database::new("stock", None, false);
    }

    pub fn add_code(code: String) {
        let db_from_path = PathBuf::from("stock.tinydb");
        let mut db: Database<WhiteList> = Database::auto_from(db_from_path, false).unwrap();
        let item = WhiteList { code: code.clone() };

        let _ = db.add_item(item);
        let _ =db.dump_db();
    }
    pub fn get_codes() -> Vec<String> {
        let db_from_path = PathBuf::from("stock.tinydb");
        let db: Database<WhiteList> = Database::auto_from(db_from_path, false).unwrap();
        let data = db.items.into_iter().collect::<Vec<_>>();
        let mut items: Vec<String> = vec![];
        for i in data {
            items.push(i.code)
        }
        items
    }
}
