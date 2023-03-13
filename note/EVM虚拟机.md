

EVM是一个基于栈的虚拟机，栈的宽度为256bits，深度为1024，主要是每一个操作数都会进行入栈/出栈操作。优点是无视寄存器的物理架构。但是会比较慢。



人类可读的合约代码sol （EVM code）

EVM执行的操作码 ==> 交给EVM来执行（入栈/出栈）

以太坊客户端 Geth，Parity  也是Runtime



存储的架构：

1. EVM code 区：部署的代码
2. machine state：PC、calldata、Gas、stack、memory
3. world state：Storage

外部账户信息列表 会存储在 Storage中吗？

部署了一个智能合约，会对这三个区产生哪些影响？

![img](https://picx.zhimg.com/v2-c3efcc2a5ecaffb2ca4bf94e40060a52_r.jpg?source=1940ef5c)



![img](https://picx.zhimg.com/80/v2-e616b8ccb8c7b4fb4690fe8e9e3c8b0f_1440w.webp?source=1940ef5c)

```
补充：calldata也在PC下面，与operation交互
```

当外部方法调用合约时候，实参会存储在calldata中，具体的被调用的EVM code会被推进 stack中执行，执行过程中，不断累积记录消耗的gas值。

stack不产生gas费。

storage是一块巨大的映射，2^256个slots，每个slot大小为32Bits。是每一个合约持久化存储数据的地方。（可以理解为进程的虚拟内存空间）

````solidity
// 托管 存储ETH到contract中，并支持拿回所有的ETH

pragma solidity 0.8.0
contract Escow {
	address agent;
	mapping(address => uint256) public deposits;
	modifier onlyAgent() {
		require(msg.sender == agent); // 只有通过部署的用户才能操作 
		_;
	}
	constructor () public {
		agent = msg.sender;
	}
	function deposit(address payee) public onlyAgent payable {
		uint256 amount = msg.value; // 以太坊交易时携带的ETH数目
		deposits[payee] = deposits[payee] + amount;
	}
	function withdraw(address payable payee) public onlyAgent {
		uint256 payment = deposits[payee];
		deposits[payee] = 0;
		payee.transfer(payment); // 转账函数？是属于特殊函数么
	}
}
````

solc 编译后，会有一个json格式，包含了 object 和 opcodes 的字符串。

ABI编码 可以通过特定解码后，用 json 描述。

deposits这些storage变量，都会记录到merkle tree中。

（一个合约账户的storage，和merkle tree的关系？）

map和vector的存储则不同。



函数选择器：

（这些meta数据，能够有什么专门的方法调用得到嘛）

跨合约调用：

​	call 和 delegatecall：看调用的是哪个的storage。