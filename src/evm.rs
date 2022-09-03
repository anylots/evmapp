use crate::db::spec::Database;
use crate::interpreter;
use crate::state::State;
use crate::types::{Env, Error, Log, RunResult};

pub struct EVM<'a, DB> {
    code: &'a [u8],
    state: State<DB>,
}

impl<'a, DB: Database> EVM<'a, DB> {
    pub fn new(db: DB, code: &'a [u8]) -> Self {
        Self {
            code: code,
            state: State::new(db),
        }
    }

    pub fn run(&mut self, env: &Env) -> RunResult {
        let res = interpreter::run(self.code, env, &self.state);
        return res;
    }
}
