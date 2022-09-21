use ethereum_types::{Address, U256};
use evmapp::evm;
use evmapp::storage::ramdb;
use evmapp::types::{Env, RunResult};

fn main() {
    let database = ramdb::RamDB::new();
    //6080 6040 52 6001 6001a 01 6080 52 6001 6002 55 6020 6080 f3
    let code = hex::decode("60806040526001600101608052600160025560206080f3").unwrap();

    let env = Env {
        caller: Address::zero(),
        timestamp: U256::from(0),
        number: 0.into(),
        chainid: 1.into(),
        calldata: vec![],
    };

    let mut evm = evm::EVM::new(database, &code);

    let result: RunResult = evm.run(&env);

    let vec = result.unwrap().0;
    for i in &vec[vec.len() - 5..vec.len()] {
        println!("value is {}", i);
    }
}
