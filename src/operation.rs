use crate::host;
use crate::interpreter::Context;
use crate::storage::spec::Database;
use crate::types::{Error, OpResult, OpStep};
use ethereum_types::U256;

pub fn stop() -> OpResult {
    Ok(OpStep::Return(Vec::new()))
}

pub fn evm_return<DB: Database>(ctx: &mut Context<DB>) -> OpResult {
    let start = ctx.stack.pop().as_usize();
    let len = ctx.stack.pop().as_usize();
    Ok(OpStep::Return(ctx.memory.mview(start, len)?.to_vec()))
}

pub fn add<DB: Database>(ctx: &mut Context<DB>) -> OpResult {
    let left = ctx.stack.pop();
    let right = ctx.stack.pop();
    ctx.stack.push_u256(left + right)?;
    ctx.pc += 1;
    Ok(OpStep::Continue)
}

pub fn mul<DB>(ctx: &mut Context<DB>) -> OpResult {
    let left = ctx.stack.pop();
    let right = ctx.stack.pop();
    ctx.stack.push_u256(left * right)?;
    ctx.pc += 1;
    Ok(OpStep::Continue)
}

pub fn mstore<DB: Database>(ctx: &mut Context<DB>) -> OpResult {
    let key = ctx.stack.pop().as_usize();
    let value = ctx.stack.pop();
    ctx.memory.mstore(key, value)?;
    ctx.pc += 1;
    Ok(OpStep::Continue)
}

pub fn sstore<DB: Database>(ctx: &mut Context<DB>) -> OpResult {
    let key = ctx.stack.pop();
    let value: U256 = ctx.stack.pop();
    ctx.state.store(key, value);
    ctx.pc += 1;
    Ok(OpStep::Continue)
}

pub fn pushn<DB>(ctx: &mut Context<DB>, n: usize) -> OpResult {
    if ctx.pc + n < ctx.code.len() {
        let slice = &ctx.code[ctx.pc + 1..ctx.pc + n + 1];
        let value = U256::from_big_endian(slice);
        ctx.stack.push_u256(value)?;
        ctx.pc += n + 1;
        Ok(OpStep::Continue)
    } else {
        Err(Error::StackOverflow)
    }
}

pub fn push<DB>(ctx: &mut Context<DB>) -> OpResult {
    if ctx.pc + 1 < ctx.code.len() {
        let slice = &ctx.code[ctx.pc + 1..ctx.pc + 2];
        let value = U256::from_big_endian(slice);
        ctx.stack.push_u256(value)?;
        ctx.pc += 2;
        Ok(OpStep::Continue)
    } else {
        Err(Error::StackOverflow)
    }
}

pub fn callvalue<DB>(ctx: &mut Context<DB>) -> OpResult {
    let value = &ctx.env.value;
    ctx.stack.push_u256(*value)?;
    ctx.pc += 1;
    Ok(OpStep::Continue)
}

pub fn calldata_load<DB>(ctx: &mut Context<DB>) -> OpResult {
    let starting = &ctx.stack.pop().as_usize();
    let calldata_part =
        &ctx.env.calldata[*starting..usize::min(32, &ctx.env.calldata.len() - starting)];
    let value = U256::from_big_endian(calldata_part);
    ctx.stack.push_u256(value)?;
    ctx.pc += 1;
    Ok(OpStep::Continue)
}

pub fn calldata_size<DB>(ctx: &mut Context<DB>) -> OpResult {
    let len = &ctx.env.calldata.len();
    ctx.stack.push_u256(U256::from(*len))?;
    ctx.pc += 1;
    Ok(OpStep::Continue)
}

pub fn calldata_copy<DB>(ctx: &mut Context<DB>) -> OpResult {
    let dest_offset = &ctx.stack.pop().as_usize();
    let starting = &ctx.stack.pop().as_usize();
    let size = &ctx.stack.pop().as_usize();

    let calldata = &ctx.env.calldata[*starting..(starting + size)];
    let _ = &ctx
        .memory
        .mstore(*dest_offset, U256::from_big_endian(calldata))?;
    ctx.pc += 1;
    Ok(OpStep::Continue)
}

pub fn block_num<DB>(ctx: &mut Context<DB>) -> OpResult {
    let num = host::get_blockNum();
    let value = U256::from_big_endian(&num.to_be_bytes());
    ctx.stack.push_u256(value)?;
    ctx.pc += 1;
    Ok(OpStep::Continue)
}
