use crate::db::spec::Database;
use ethereum_types::U256;
use std::collections::HashMap;

pub struct State<DB> {
    db: DB,
    cache: HashMap<U256, U256>,
}

impl<DB: Database> State<DB> {
    pub fn new(db: DB) -> Self {
        Self {
            db: db,
            cache: HashMap::new(),
        }
    }
}
