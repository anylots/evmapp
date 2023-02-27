use crate::memory::Memory;
use crate::operation;
use crate::stack::Stack;
use crate::state::State;
use crate::storage::spec::Database;
use crate::types::{Env, Error, Log, OpResult, OpStep, RunResult};

// Evm context, For each contract call, a new evm context will be created.
pub struct Context<'a, DB> {
    // The byte code of smart contract, Each instruction use one byte, and can support 256 different instructions.
    pub code: &'a [u8],
    // Evm stack, with capacity of 1024, and 32bytes of each item.
    pub stack: Stack,
    // Evm memory
    pub memory: Memory,
    // Eth world state abstraction layer
    pub state: &'a mut State<DB>,
    // Program counter, Using for track the location of the next instruction
    pub pc: usize,
    // Execution log
    pub logs: Vec<Log>,
}


// Execute contract code using stack structure.
pub fn run<DB: Database>(code: &[u8], env: &Env, state: &mut State<DB>) -> RunResult {

    // create new evm context
    let mut ctx = Context {
        code,
        stack: Stack::new(),
        memory: Memory::new(),
        state,
        pc: 0,
        logs: Vec::new(),
    };

    // execute instruction stream of contract
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
        0x43 => operation::blockNum(ctx),
        0x52 => operation::mstore(ctx),
        0x55 => operation::sstore(ctx),
        0x60 => operation::pushn(ctx, 1),
        0x61 => operation::pushn(ctx, 2),
        0xf3 => operation::evm_return(ctx),
        opcode => Err(Error::InvalidOpcode(opcode)),
    }
}
