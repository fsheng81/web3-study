本篇记录solidity语法与实战的一些程序

来源于 wtf-solidity、dapp-learning-dao



## solidity 语法

只记录一些自己觉得重要的：

一般的场景分类：

1. 合约创建合约：工厂模式
2. 合约调用合约

### 关键元信息

msg.sender 发送者地址

msg.value 

### 关键词



### ABI 接口 selector

ABI就是一个二维码

### 调用

当地址A 通过合约B来`call`合约C的函数，语境（context：变量）是合约C的，此时msg.sender是合约B，如果造成storage状态变量的变化，产生的效果会作用到合约C的变量上。

delegatecall：语境是合约B，msg.sender是地址A，状态变量的变化是作用到合约B

理解：富商把资产交给VC打理，执行的是VC的函数，改变的是富商的状态

### 库合约

常见库合约：String、Address、Create2、Arrays

### import

引入其他合约：

1. 源代码的相对位置
2. 网上URL链接
3. npm导入
4. 全局符号

### dive into Evm



## 案例-wtf

### ERC-20

维护一个映射（用户，额度）作为账本，同时增加一个授权功能

```javascript
Contract ERC20 {
    map(address, uint256)； /* */
    
	查询账户余额
    转账
    授权另一个账户一定的额度
}
```

#### 水龙头faucet

让用户能够通过调用函数，领到免费代币，每一个地址只能领取一次

相当于 `ICO合约`给`faucet合约`一定的授权，需要首先部署ICO合约，再部署faucet合约，同时前端调用ICO合约的transfer()函数，from `faucet合约账户` to `用户账户`

发放的结果，就是在ICO合约的map中，修改了状态变量。

```
```

#### 空投 airDrop

 空投合约需要ICO合约的授权，经过检查后，挨个trasfrom()转账

**转 ICO代币 和交易时候的 msg.value 需要区分**

最大的思维转变：转账不是说把一个金钱实体从一个账户实体转到另一个账户实体，而是在一个账本上记录这个额度值。

### ERC-721

接下来是NFT，最主要的还是理解NFT代表独一无二的账本(map)里的tokenId。其对应的实体可以是同一个IFPS的URL文件。比如一个URL，可以有0-100的tokenId。

ERC-165 提供一个检查，检查该合约是否支持了其他合约interface

```
```

checkOnERC721Received() 如果发送的to合约，不能够支持接收，则会发生整个一次交易的回滚。

public\private\external\internal

safeTransferfrom()

函数选择器：调用一个合约里面的某一个函数，发送一段calldata作为input，calldata的组成就是selector+param组成calldata

先有一个模板的ERC-721合约，然后再加上自己的定制合约（继承自ERC-721）

#### 荷兰拍卖

价格从高到低，每个一段时间，价格降低一次。

用户在任意时刻购买，合约都能够根据时间点算出价格，促成交易。避免了抢拍卖时候的复杂的信息交互。

用户不止提供 代币，还有msg.value

```
这个 拍卖合约 与 本身的NFT合约 怎么交互？
```

#### 发放白名单

难点在于白名单的地址信息如果全部都链上存储，会比较消耗gas费。

**merkle tree**

根据所有白名单地址，计算出merkle tree的root，设置到合约中

前端根据 用户地址 计算 merkle proof

交易携带 merkle proof，在EVM中验证是否是白名单。

**ECDSA数字签名**

还是相当于一个信息压缩吧。

#### NFT交易所

行为：挂单、撤单、修改、购买

部署NFT合约，

部署swap合约，

操作NFT合约，把NFT转到Swap合约（变更wtf合约中的一个所属权）

操作swap合约，上架NFT

（这种交易时候扣的钱，是msg.value的eth，还是一些其他代币？）

#### 随机数

链下chainlink预言机提供VRF服务，链上开发者提供LINK代币，调用VRF合约，并等待返回随机数

随机数合约 继承 VRFConsumerBase，并转入 LINK 代币。

### ERC-1155

一个合约里面包含多个 ICO NFT代币。区别就是用(id, account)来标记

account==1的都是NFT，其他的是ICO

与上面的区别是，NFT的tokenId就直接换成了id，尽管可以仍旧只用IFPS的一个URL

#### WETH

主链上一直运行一个WETH

#### 线性释放

相当于锁住ERC-20，在过来取钱的时候，判断当前时间点，能有多少可以取的。

用算法明确一个规则就行。

疑问：线性释放、代币锁、时间锁，这些区别呢？

### 代理

主要是考虑到升级的问题

多签钱包

### 后续

Degen、PR模板

graph

## 案例-Dapp



## 风险

