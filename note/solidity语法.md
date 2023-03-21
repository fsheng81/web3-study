solidity语法

基础的语法就不用复述，基本都是一些特别的语法点记录

## 笔记

### 编译

从contract.sol文件编译后，能够生成一个ABI.json文件和二进制文件bytecodes。

部署的时候，是把这个bytecodes作为一个交易的data，发送给某一个地址。

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

momery再次读写的gas消耗都比较小，因此可以先把storage中的复杂变量读取到memory中，再进行操作。

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



### 代理

一般还是直接继承已经写好的代理库就好。



## assembly

