# A compact EVM implementation with Rust.

- 这是一个EVM运行时、解释器的简单实现，包括evm的stack、memory、pc、storage、以及主要指令的逻辑实现，用于对EVM的学习、合约执行过程的熟悉；通过洞察合约执行过程中内存、存储的处理逻辑，达到对solidity、内联汇编开发中gas节约、合约安全的深刻的理解；

- 不包含所有硬分叉的检查、余额检查、account state的校验.

![image](https://github.com/anylots/evmapp/blob/main/evm_rabbit.png)