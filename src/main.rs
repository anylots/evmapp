use ethereum_types::{Address, U256};
use evmapp::evm;
use evmapp::storage::ramdb;
use evmapp::types::{Env, RunResult};

fn main() {
    let database = ramdb::RamDB::new();
    //6080 6040 52 6001 6001a 01 6080 52 6020 6080 f3
    let code = hex::decode("6080604052600160010160805260206080f3").unwrap();

    let env = Env {
        caller: Address::zero(),
        timestamp: U256::from(0),
        number: 0.into(),
        chainid: 1.into(),
        calldata: vec![],
    };

    let mut evm = evm::EVM::new(database, &code);

    let result: RunResult = evm.run(&env);

    let vec= result.unwrap().0;
    for i in vec.iter(){
        println!("value is {}",i);
    }
    // let vec_str: String = (&vec[..]).join('-');


    // println!("result is {}", result.is_ok());
    // println!("result is {}",vec_str);

}
