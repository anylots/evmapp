use crate::interpreter;
use crate::state::State;
use crate::storage::spec::Database;
use crate::types::{Env, RunResult};

pub struct EVM<'a, DB> {
    code: &'a [u8],
    state: State<DB>,
}

impl<'a, DB: Database> EVM<'a, DB> {
    //create evm instance
    pub fn new(db: DB, code: &'a [u8]) -> Self {
        Self {
            code,
            state: State::new(db),
        }
    }

    //execution of contract call
    pub fn run(&mut self, env: &Env) -> RunResult {
        //run with interpreter
        let rt = interpreter::run(self.code, env, &mut self.state);
        match rt {
            Ok(_) => self.state.commit(),
            Err(_) => self.state.rollback(),
        }
        println!("evm run finish");
        return rt;
    }
}
