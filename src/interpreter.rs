use crate::stack::Stack;
use crate::state::State;
use crate::storage::spec::Database;
use crate::types::{Env, Error, Log, OpResult, OpStep, RunResult};
use ethereum_types::U256;

pub struct Context<'a, DB> {
    pub code: &'a [u8],
    pub stack: Stack,
    pub state: &'a State<DB>,
    pub pc: usize,
    logs: Vec<Log>,
}

pub fn run<DB: Database>(code: &[u8], env: &Env, state: &State<DB>) -> RunResult {
    let mut ctx = Context {
        code: code,
        stack: Stack::new(),
        state: state,
        pc: 0,
        logs: Vec::new(),
    };

    loop {
        if ctx.pc >= ctx.code.len() {
            return Err(Error::CodeOutOfBound);
        }
        match exec_operation(ctx.code[ctx.pc], &mut ctx) {
            Err(err) => return Err(err),
            Ok(OpStep::Continue) => (),
            Ok(OpStep::Return(v)) => return Ok((v, ctx.logs)),
        }
    }
}

pub fn exec_operation<DB: Database>(opcode: u8, ctx: &mut Context<DB>) -> OpResult {
    match opcode {
        0x00 => stop(),
        0x52 => mstore(ctx),
        0x60 => push1(ctx),
        // 0x61 => push1(ctx),
        opcode => Err(Error::InvalidOpcode(opcode)),
    }
}

pub fn stop() -> OpResult {
    Ok(OpStep::Return(Vec::new()))
}

pub fn mstore<DB>(ctx: &mut Context<DB>) -> OpResult {
    let value = ctx.stack.pop();
    let key=ctx.stack.pop();
    // ctx.state.cache
    Ok(OpStep::Continue)
}

pub fn push1<DB>(ctx: &mut Context<DB>) -> OpResult {
    if ctx.pc + 1 < ctx.code.len() {
        let mut value = &ctx.code[ctx.pc + 1..ctx.pc + 2];
        ctx.stack.push_u256(U256::default());
        ctx.pc += 2;
        Ok(OpStep::Continue)
    } else {
        Err(Error::StackOverflow)
    }
}
