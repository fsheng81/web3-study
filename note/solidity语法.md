solidity语法

基础的语法就不用复述，基本都是一些特别的语法点记录。

EVM中主要有内存单位：1word = 32bytes = 256bits。

## 笔记

### 编译

从contract.sol文件编译后，能够生成一个ABI.json文件和二进制文件bytecodes。

部署的时候，是把这个bytecodes作为一个交易的data，发送给某一个地址。

以remix的1_Storage.sol作为样例，编译后的bytecodes为：

```bash
608060405234801561001057600080fd5b506004361061003657
60003560e01c80632e64cec11461003b5780636057361d1461005957
5b600080fd5b610043610075565b60405161005091906100d9565b60405180910390f35b610073600480360381019061006e919061009d565b61007e565b005b60008054905090565b8060008190555050565b60008135905061009781610103565b92915050565b6000602082840312156100b3576100b26100fe565b5b60006100c184828501610088565b91505092915050565b6100d3816100f4565b82525050565b60006020820190506100ee60008301846100ca565b92915050565b6000819050919050565b600080fd5b61010c816100f4565b811461011757600080fd5b5056fea2646970667358221220404e37f487a89a932dca5e77faaf6ca2de3b991f93d230604b1b8daaef64766264736f6c63430008070033
```

在本地可以通过`solc --bin source.sol`来输出该字符串。这个bytecode包括了部署的操作 和 需要部署的code。可以通过`solc --bin-runtime source.sol`来输出一个更短的code函数体。

### import / 库合约 

import一般是某一个链上合约/或者本地合约的ABI。其实ABI的字符串等价于 import一个Interface文件。

```solidity
import '@openzeppelin/contracts/access/Ownable.sol';
```



### openzepplin

openzepplin是一个辅助开发库，能够提供较多的ERC标准的实现，新定义的合约类直接继承这些实现类的接口即可。

### 存储

存储分为 calldata \ memory \ storage三种，storage是上链的，因此gas消耗最大，一般也是作为 合约的全局变量而存在；memory一般是函数中的局部变量，读取修改的gas费较小；calldata区是存放调用合约时的传入的实参。

storage是按照slot存放，每一个slot实际存储时占用32bytes/256bits，按照全局变量声明的先后顺序存放，因此可以适当的优化存储方式，紧密排列。

storage的map和byte[]的存储方法：一般来说map更加贴近storage的存储方式，更加推荐。

memory再次读写的gas消耗都比较小，因此可以先把storage中的复杂变量读取到memory中，再进行操作。

每次执行一个合约，都会初始化一个上下文Context，其包括：stack, storage, memory, code, calldata, returnData

calldata 包括了 函数签名4bytes + 函数实参值，

```solidity
// 函数名
function store(uint256);
// 调用该函数，并传入10作为实参：
calldata = 0x6057361d000000000000000000000000000000000000000000000000000000000000000a
// 前4byte为：
keccak256(“store(uint256)”) -> first 4 bytes -> 0x6057361d
// 后32bytes为：
0x0a = (10)
// calldata 可以用于call()函数的参数
address(this).call(abi.encodeWithSignature(“store(uint256)”, 10));
```

#### 世界状态

区块链最重要的特性是能够保证世界状态的一致，在区块中都保存了当前的所有merkle tree root状态。

https://learnblockchain.cn/article/4283

block head:

​	transaction root

​	receipent root

​	state root: (each account)

​		nonce:

​		balance:

​		codehash: 就是代码区

​		storage root：所以说storage数据会上链。

### 交易ETH

ETH的交易属于以太坊设计时的基础功能，因此是直接调用一些特殊函数来实现，与智能合约写出来的逻辑不一致。

```solidity
// msg.sender 	消息发送者的地址
// msg.value	消息发送者传递的 ether
// msg.gas		消息发送者设定的gas
// msg.data		消息发送者传递的data
```

对应的函数有：

```solidity
// 接收 ether
// msg.data == NULL 会执行 receive()
// 如果一个合约既没有receive()，也没有fallback()，那么向其发送 ether 则会报错
fallback() external payable {
	// .... 
}
receive() external payable {
	// ....
}

// 发送 ether
// 三个函数中基本是使用 call() 和 transfer()
_to.transfer(amount);
(bool) = _to.send(amount);
(bool, bytes) = _to.call{value: amount}("data");
```

一般智能合约里面交易ether，都是指 weth合约中的token。

### 调用

调用包括几种调用方式：ethers.js调用、solidity代码调用，底层call()和delegatecall()。

ethers.js 前端调用

```js
import { ethers } from "ethers";

// 连接Rinkeby测试网
const provider = new ethers.providers.JsonRpcProvider(`https://rinkeby.infura.io/v3/${INFURA_ID}`);
const wallet = new ethers.Wallet(privateKey, provider);

// const abi = ["....."]; // abi.json文件的内容 或 import导入
const contractObj = new ethers.Contract(address, abi, wallet);
const res = await contractObj.function(params);
```

solidity代码调用

```solidity
import ..../ContractName.sol
// 或者 IContractName.sol也可以

ContractName(_Address).functionName(params);
```

call()调用

````solidity
// 目标合约地址.call(二进制编码);
(bool success, bytes memory data) = _addr.call{value: msg.value}(
	abi.encodeWithSignature("functionName(uint256)", x)
);

// 如果有data返回的话，记得解码
uint256 res = abi.decode(data, (uint256));
````

call() 和 delegatecall() 区别在于使用哪个的msg.sender和storage

```
userA -> call -> B -> call -> C
in c:
	context: C
	msg.sender: B


userA -> call -> B -> delegatecall -> C
in c:
	context: B
	msg.sender: A
```



### 创建新合约

基本就是看create2()吧，更加可控

```solidity
// 一般方法：new
import ContractName.sol
ContractName x = new ContractName{value: _value}(params)

// create2()方法 能够控制具体生成的地址
// 新地址 = hash("0xFF",创建者地址, salt, bytecode)
// 还有不用new的方式
bytes32 salt = keccak256(abi.encodePacked(token0, token1));
Contract x = new Contract{salt: _salt, value: _value}(params);

predictedAddress = address(uint160(uint(keccak256(abi.encodePacked(
	bytes1(0xff),
	address(this),
	salt,
	keccak256(type(Pair).creationCode)
)))));
```

### 代理/升级

一般还是直接继承已经写好的可升级库就好。

uniswap的factory为了可升级的要求，而constructor()不带参数，通过initialize()来初始化

### 回调

这里的回调函数参考 uniswapV3 中的实现，能够保证对不同调用者执行不同的功能。只要最后校验就好。



## assembly

EVM playground：https://www.evm.codes/playground

在solidity的代码中，assembly{}代码块能够提供更加底层的操作逻辑，用于实现更多复杂的功能。主要是理解opcode的含义，以及EVM在执行过程中，以 stack 和 (storage, memory, calldata, code)等存储结构交互来实现。

### 函数选择

以remix的样例1_storage.sol文件为例，其编译后的bytecode如下：

```bash
608060405234801561001057600080fd5b506004361061003657
60003560e01c80632e64cec11461003b5780636057361d1461005957
5b600080fd5b610043610075565b60405161005091906100d9565b60405180910390f35b610073600480360381019061006e919061009d565b61007e565b005b60008054905090565b8060008190555050565b60008135905061009781610103565b92915050565b6000602082840312156100b3576100b26100fe565b5b60006100c184828501610088565b91505092915050565b6100d3816100f4565b82525050565b60006020820190506100ee60008301846100ca565b92915050565b6000819050919050565b600080fd5b61010c816100f4565b811461011757600080fd5b5056fea2646970667358221220404e37f487a89a932dca5e77faaf6ca2de3b991f93d230604b1b8daaef64766264736f6c63430008070033
```

以`60003560e01c80632e64cec11461003b5780636057361d1461005957`为例，记录函数选择时的动作：

此时`calldata`为：0x6057361d000000000000000000000000000000000000000000000000000000000000000a

```bash
PUSH1 0x00 		# 把 值0x00 推进 栈
CALLDATALOAD	# 弹出栈顶元素0x00，作为入参offset，从calldata区读取 1 word 入栈
PUSH1 0xe0		# 再推入 0xe0 = (224) = (256 - 32)bit
SHR				# 弹出栈顶元素 0xe0, 并对下一个元素右移 224bit，此时只有4 bytes 了
DUP1  			# 复制一遍栈顶元素 即 值0x6057361d
PUSH4 0x2e64cec1# 再推入一个 函数签名
EQ				# 弹出并判断栈顶两个元素是否相等，结果不相等，入栈 0
PUSH2 0x003b	# 推入59，这个数字时 0x2e64cec1 对应函数代码code所在位置
JUMPI			# jump if. 发现是false，函数名不对，因此继续找
DUP1 			# 复制
PUSH4 0x6057361d# 重新判断     
EQ				# 找到
PUSH2 0x0059	# store()函数代码所在位置 0x0059
JUMPI			# 确定跳转
```

### memory

内存的关键是了解到内存的布局，也就是有一个指针志向当前内存已经使用的最后地址。

```bash
PUSH1 0x80	# 把 0X80值入栈
PUSH1 0x40	# 把 0X40值入栈
MSTORE		# 把 值y 存入 地址x中，也就是 把 0x80值 存入 地址0x40 + 256bits

# 主要指令
mload x: 		# 从地址x处，读取32bytes到栈中
mstore x, y:	# 把y存到x地址处 + 32bytes
mstore8 x, y:	# 把y存到x地址处 + 8bytes
```

内存里面的基本单位是byte，也就是说，地址0x40是第64个byte位置

内存有一些保留的地址：

```
    0x00- 0x3f（64 字节）：	暂存空间
    0x40- 0x5f（32字节）：	空闲内存指针
    0x60- 0x7f（32 字节）：	零槽
```

执行函数前先给其局部变量分配空间时候，就是首先更改 0x40的值，首先留出空间来读写。

https://learnblockchain.cn/article/4258

### storage

storage本身就是一个k-v存储，所以其操作比较简单

```bash
sstore v, k # 把 v 存到 k 中，都是32bytes
sload k 	# 读取k的value到栈
```

针对slot packing，增加与或非的操作来完成值的存取。

## EIP

EIP-2535 Diamonds（钻石）

