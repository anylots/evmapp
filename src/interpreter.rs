use crate::db::spec::Database;
use crate::stack::Stack;
use crate::state::State;
use crate::types::{Env, Error, Log, OpResult, OpStep, RunResult};
use ethereum_types::U256;

pub struct Context<'a> {
    pub code: &'a [u8],
    pub stack: Stack,
    pub pc: usize,
    logs: Vec<Log>,
}

pub fn run<DB: Database>(code: &[u8], env: &Env, state: &State<DB>) -> RunResult {
    let mut ctx = Context {
        code: code,
        stack: Stack::new(),
        pc: 0,
        logs: Vec::new(),
    };

    loop {
        match exec_operation(ctx.code[ctx.pc], &mut ctx) {
            Err(err) => return Err(err),
            Ok(OpStep::Continue) => (),
            Ok(OpStep::Return(v)) => return Ok((v, ctx.logs)),
        }
    }
}

pub fn exec_operation(opcode: &u8, ctx: &mut Context) -> OpResult {
    match opcode {
        0x60 => push1(ctx),
        0x61 => push1(ctx),
    }
}

pub fn push1(ctx: &mut Context) -> OpResult {
    if ctx.pc + 1 < ctx.code.len() {
        let mut value = &ctx.code[ctx.pc + 1..ctx.pc + 2];
        ctx.stack.push_u256(U256::default());
        ctx.pc += 2;
        Ok(OpStep::Continue)
    } else {
        Err(Error::StackOverflow)
    }
}
