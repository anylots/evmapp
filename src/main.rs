use ethereum_types::{Address, U256};
use evmapp::evm;
use evmapp::storage::ramdb;
use evmapp::types::{Env, RunResult};

//Use evm to run a small piece of contract code.
fn main() {


    //Step0. Prepare evm bytecode

    /*
     * en. This is a piece of code that calculates the addition of two numbers in the contract.
     * We try to pass in 16 + 1 (0x10 + 0x01), use evm to calculate, and expect the result = 17.
     * 
     * zh. 这是一段计算两个数相加的代码, 我们尝试传入16 + 1（0x10 + 0x01），用evm计算，期望结果=17。
     * 
     * ===code===
     * 6080:  PUSH1 0x80  /* 0x40 - 0x5f (32 字节): 空闲内存指针具体的值 = 0x80 = 4 x 32, 运行solidity的时候，需要预留 4 个 32 字节的插槽 */
     * 6040:  PUSH1 0x40  /* 0x40 - 0x5f (32 字节): 存储空闲内存指针值的地方 = 0x40 = 2 x 32, 也就是第三个插槽 */
     * 52:    MSTORE      /* 将栈顶的数据保存到内存中（key = 0x40, value =0x80） */
     * 6001:  PUSH1 0x0a  /* 参数0x16 放入栈 */
     * 6001:  PUSH1 0x01  /* 参数0x1 放入栈 */
     * 01:    ADD         /* 将栈顶的两个数弹出并相加，结果add_result放入栈顶 */
     * 6080:  PUSH1 0x80  /* 将80放入栈 */
     * 52:    MSTORE      /* 保存数据到内存，key =0x80, value = add_result*/
     * 6001:  PUSH1 0x01  /* PUSH1 0x01 */
     * 6002:  PUSH1 0x02  /* PUSH1 0x02 */
     * 55:    SSTORE      /* 将数据保存到storage */
     * 6020:  PUSH1 0x20  /* PUSH1 0x20*/
     * 6080:  PUSH1 0x80  /* PUSH1 0x80*/
     * f3:    RETURN      /* 将内存中从80开始，长度为0x20 = 32bytes的数据返回 */
     * ===code===
     */
    let code =
        hex::decode("6080 6040 52 6010 6001 01 6080 52 6001 6002 55 6020 6080 f3".replace(" ", ""))
            .unwrap();


    // Step1. Prepare basic parameters required to run evm.
    let env = Env {
        caller: Address::zero(),
        timestamp: U256::from(0),
        number: 0.into(),
        chainid: 1.into(),
        calldata: vec![],
    };

    // Step2. Create evm instance.
    let database = ramdb::RamDB::new();
    let mut evm = evm::EVM::new(database, &code);

    // Step3. Run evm to handle call.
    let result: RunResult = evm.run(&env);

    let vec_cal = result.unwrap().0;
    for i in &vec_cal[vec_cal.len() - 1..vec_cal.len()] {
        //This is not a test case, print the result value directly
        println!("value is {}", i);
    }
}
