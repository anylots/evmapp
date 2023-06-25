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
    // Basic parameters required to run evm.
    pub env: &'a Env,
    // Execution log
    pub logs: Vec<Log>,
}

impl<'a, DB: Database> Context<'a, DB> {
    fn new(code: &'a [u8], env: &'a Env, state: &'a mut State<DB>) -> Self {
        Self {
            code,
            stack: Stack::new(),
            memory: Memory::new(),
            state,
            pc: 0,
            env: env,
            logs: Vec::new(),
        }
    }
}

// Execute contract code using stack structure.
pub fn run<DB: Database>(code: &[u8], env: &Env, state: &mut State<DB>) -> RunResult {
    let mut ctx = Context::new(code, env, state);
    loop {
        match exec_operation(ctx.code[ctx.pc], &mut ctx) {
            Err(err) => return Err(err),
            Ok(OpStep::Return(v)) => return Ok((v, ctx.logs)),
            _ => (),
        }
        if ctx.pc >= ctx.code.len() {
            return Err(Error::CodeOutOfBound);
        }
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                       EVM OPERATION                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/
// execute by opcode
pub fn exec_operation<DB: Database>(opcode: u8, ctx: &mut Context<DB>) -> OpResult {
    match opcode {
        0x00 => operation::stop(),
        0x01 => operation::add(ctx),
        0x02 => operation::mul(ctx),
        0x34 => operation::callvalue(ctx),
        0x35 => operation::calldata_load(ctx),
        0x36 => operation::calldata_size(ctx),
        0x37 => operation::calldata_copy(ctx),
        0x43 => operation::block_num(ctx),
        0x52 => operation::mstore(ctx),
        0x55 => operation::sstore(ctx),
        0x5f => operation::push0(ctx),
        0x60 => operation::pushn(ctx, 1),
        0x61 => operation::pushn(ctx, 2),
        0x62 => operation::pushn(ctx, 3),
        0x63 => operation::pushn(ctx, 4),
        0x64 => operation::pushn(ctx, 5),
        0x65 => operation::pushn(ctx, 6),
        0x66 => operation::pushn(ctx, 7),
        0x67 => operation::pushn(ctx, 8),
        0x68 => operation::pushn(ctx, 9),
        0x69 => operation::pushn(ctx, 10),
        0x6a => operation::pushn(ctx, 11),
        0x6b => operation::pushn(ctx, 12),
        0x6c => operation::pushn(ctx, 13),
        0x6d => operation::pushn(ctx, 14),
        0x6e => operation::pushn(ctx, 15),
        0x6f => operation::pushn(ctx, 16),
        0x70 => operation::pushn(ctx, 17),
        0x71 => operation::pushn(ctx, 18),
        0x72 => operation::pushn(ctx, 19),
        0x73 => operation::pushn(ctx, 21),
        0x74 => operation::pushn(ctx, 22),
        0x75 => operation::pushn(ctx, 23),
        0x76 => operation::pushn(ctx, 23),
        0x77 => operation::pushn(ctx, 24),
        0x78 => operation::pushn(ctx, 25),
        0x79 => operation::pushn(ctx, 26),
        0x7a => operation::pushn(ctx, 27),
        0x7b => operation::pushn(ctx, 28),
        0x7c => operation::pushn(ctx, 29),
        0x7d => operation::pushn(ctx, 30),
        0x7e => operation::pushn(ctx, 31),
        0x7f => operation::pushn(ctx, 32),
        0xf3 => operation::evm_return(ctx),
        opcode => Err(Error::InvalidOpcode(opcode)),
    }
}
