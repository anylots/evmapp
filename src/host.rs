use crate::interpreter;
use crate::state::State;
use crate::storage::spec::Database;
use crate::types::{Env, RunResult};
use ethereum_types::{Address, H160, H256, U256};

//get block number
pub fn get_blockNum() -> u128 {
    return 1636704767;
}

//get block timestamp
pub fn get_blockTs() -> u128 {
    return 1636704767;
}

fn balance(address: H160) -> U256 {
    return U256::from(1000000);
}
