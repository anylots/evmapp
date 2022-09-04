use std::time::SystemTime;

use ethereum_types::{Address, U256};
use evmapp::evm;
use evmapp::storage::ramdb;
use evmapp::types::{Env, RunResult};

fn main() {
    let database = ramdb::RamDB::new();
    let code = hex::decode("60806040").unwrap();

    let env = Env {
        caller: Address::zero(),
        timestamp: U256::from(0),
        number: 0.into(),
        chainid: 1.into(),
        calldata: vec![],
    };

    let mut evm = evm::EVM::new(database, &code);

    let result: RunResult = evm.run(&env);

    println!("result is {}", result.is_ok());
}
