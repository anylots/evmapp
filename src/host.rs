use ethereum_types::{H160, U256};

//get block number
pub fn get_block_num() -> u128 {
    return 1636704767;
}

//get block timestamp
pub fn get_block_ts() -> u128 {
    return 1636704767;
}

fn balance(address: H160) -> U256 {
    return U256::from(1000000);
}
