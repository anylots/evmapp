use crate::memory::Memory;
use crate::operation;
use crate::stack::Stack;
use crate::state::State;
use crate::storage::spec::Database;
use crate::types::{Env, Error, Log, OpResult, OpStep, RunResult};

pub struct Context<'a, DB> {
    pub code: &'a [u8],
    pub stack: Stack,
    pub memory: Memory,
    pub state: &'a mut State<DB>,
    pub pc: usize,
    pub logs: Vec<Log>,
}

pub fn run<DB: Database>(code: &[u8], env: &Env, state: &mut State<DB>) -> RunResult {
    let mut ctx = Context {
        code,
        stack: Stack::new(),
        memory: Memory::new(),
        state,
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
        0x00 => operation::stop(),
        0x01 => operation::add(ctx),
        0x02 => operation::mul(ctx),
        0x52 => operation::mstore(ctx),
        0x55 => operation::sstore(ctx),
        0x60 => operation::push1(ctx),
        0xf3 => operation::evm_return(ctx),
        // 0x61 => push1(ctx),
        opcode => Err(Error::InvalidOpcode(opcode)),
    }
}
