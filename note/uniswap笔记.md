uniswap学习主要分为几个部分：

1. AMM自动做市机制
2. 智能合约的实现
3. 前端实现
4. V2到V3的演进
5. 其他知识点相关：flashSwap

## AMM自动做市机制

自动做市机制简单来说就是uniswap交易所不采用常规的挂单方式进行交易，而是针对每一个交易对pair<tokenA, tokenB>，当用户想要用A购买B的时候，交易时保证池子中 $ amountA * amountB = constK$ 。这样保证了池子不会出现某种token被兑换完的情况。

而另一方面 流动性提供者LP 可以为池子注入流动性（tokenA，tokenB），按照当前池子的比例，并在每一次交易时收取0.3%作为LP的费用。为了证明流动性提供者的收益，pair会有相应的LP token发布。

交易过程：

假设 池子里面有(tokenA, tokenB)两种代币，其数目为reverseA，reverseB；addrU准备拿amountA的tokenA来换取tokenB，那么他能够获得的tokenB的amountB大致为：

$amountB = 0.997 * amountA * reverseB / (reverseA + amountA)$

注意到参与计算的是市场中的token，也就是池子+用户手中的token。也就是说，当次交易时，交易价格是考虑了用户手中token对池子比例影响下的价格。

## 智能合约V2

主要有两大类：core和periphery。前者是主要实现，后者是上层包装。

### ERC-20

```solidity
// UniswapV2ERC20.sol 是实现pair的 LP token功能。
// 同时该合约 还支持EIP-712 实现了信息的签名。

    bytes32 public DOMAIN_SEPARATOR;
    // keccak256("Permit(address owner,address spender,uint256 value,uint256 nonce,uint256 deadline)");
    bytes32 public constant PERMIT_TYPEHASH = 0x6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c9;
    
    constructor() public {
        uint chainId;
        assembly {
            chainId := chainid
        }
        DOMAIN_SEPARATOR = keccak256(
            abi.encode(
                keccak256('EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)'),
                keccak256(bytes(name)),
                keccak256(bytes('1')),
                chainId,
                address(this)
            )
        );
    }

    function permit(address owner, address spender, uint value, 
    	uint deadline, uint8 v, bytes32 r, bytes32 s) external {
        require(deadline >= block.timestamp, 'UniswapV2: EXPIRED');
        bytes32 digest = keccak256(
            abi.encodePacked(
                '\x19\x01',
                DOMAIN_SEPARATOR,
                keccak256(abi.encode(PERMIT_TYPEHASH, owner,
                	spender, value, nonces[owner]++, deadline))
            )
        );
        address recoveredAddress = ecrecover(digest, v, r, s);
        require(recoveredAddress != address(0) && recoveredAddress == owner, 'UniswapV2: INVALID_SIGNATURE');
        _approve(owner, spender, value);
    }
```

### Factory

Factory合约使用create2()方法创建不同的交易对pair，提供查询所有交易对，并设置收取手续费的地址

```solidity
    function createPair(address tokenA, address tokenB) external returns (address pair) {
        // 条件判断，两个token不为0地址，不可以相同地址
        require(tokenA != tokenB, 'UniswapV2: IDENTICAL_ADDRESSES');

        // tokenA/tokenB排序, 小的为token0, 另一为token1, address可以和uint160互转，所以可以排序
        (address token0, address token1) = tokenA < tokenB ? (tokenA, tokenB) : (tokenB, tokenA);

        // token0 不能为0地址, 且交易对需要存在
        require(token0 != address(0), 'UniswapV2: ZERO_ADDRESS');
        require(getPair[token0][token1] == address(0), 'UniswapV2: PAIR_EXISTS'); // single check is sufficient
        
        // 得到UniswapV2Pair的字节码(可在remix编译页面按钮下 选中pair合约，再点下面的Bytecode复制，文本中的object字段的值就是字节码)
        bytes memory bytecode = type(UniswapV2Pair).creationCode;

        // 使用create2创建合约
        bytes32 salt = keccak256(abi.encodePacked(token0, token1));
        assembly {
            pair := create2(0, add(bytecode, 32), mload(bytecode), salt)
        }

        // 调用pair的initialize对token0/token1进行初始化赋值
        IUniswapV2Pair(pair).initialize(token0, token1);

        // 将两个token正、反方向分别填充到mapping中，方便查找
        getPair[token0][token1] = pair;
        getPair[token1][token0] = pair; // populate mapping in the reverse direction

        // 添加到allPairs列表中
        allPairs.push(pair);

        // 合约创建事件
        emit PairCreated(token0, token1, pair, allPairs.length);
    }

    // 针对 init code hash 说明  这个hash是由pair的字节码 keccak256得来的
    // 每次使用remix部署的时候，可能会变，所以每次都取到bytecode/object中值进行keccak256,然后替换router中的pairFor中的code
    function pairFor(address factory, address tokenA, address tokenB) internal pure returns (address pair) {
        (address token0, address token1) = sortTokens(tokenA, tokenB);
        pair = address(uint(keccak256(abi.encodePacked(
                hex'ff',
                factory,
                keccak256(abi.encodePacked(token0, token1)),
                hex'96e8ac4277198ff8b6f785478aa9a39f403cb768dd02cbee326c3e7da348845f'
                // init code hash
            ))));
    }
```

提供查询功能：

```solidity
mapping(address => mapping(address, address)) public getPair;
// 通过 getPair[tokenA][tokenB] = pairAddr 来获取 
address[] public allPairs; // 动态数组 所有的pair对

```

手续费收取地址：

```solidity
address feeTo; // 收取手续费地址
address feeToSetter; // 可以设置的角色
```



### Pair

Pair是uniswap的主体功能：

1. LP的增加/减少流动性
2. 交换token，检验
3. 闪电贷

```solidity
// 精度考虑，采用二进制的112位作为精度，这样对于256位内存，还有256-2*112 = 32位用来记录 时间戳

// 存储累计价格，预言机使用，用来读取的。为了防止结合闪电贷的套现漏洞。
uint public price0CumulativeLast;
uint public price1CumulativeLast;
uint public kLast; // reserve0 * reserve1, as of immediately after the most recent liquidity event

// 每次池子变化后，调用该方法刷新，用当前合约的两个币种余额替换储备量。
// 再每个区块首次进来的时候，更新存储的价格，可用于预言机，具体了解参考时间加权平均价格交易（TWAP）
function _update();

// 计算 LP的手续费
function _mintFee();

// LP 调用
function mint();
function burn();

// 交易
function swap();

// 强行同步 合约中token的量 == reverse_token
function sync();
```



### Route

route是uniswap的外部包装，提供面向用户的操作方法

Library

```solidity
function sortTokens(); // 把两个token的地址按照从小到大的顺序排列

function pairFor(address factory, address tokenA, address tokenB) internal pure returns (address pair) {
	(address token0, address token1) = sortTokens(tokenA, tokenB);
	pair = address(uint(keccak256(abi.encodePacked(
				hex'ff',
				factory,
				keccak256(abi.encodePacked(token0, token1)),
				hex'96e8ac4277198ff8b6f785478aa9a39f403cb768dd02cbee326c3e7da348845f'
				// init code hash
				))));
} // 直接通过ABI计算pair的地址，节省gas

function getReserves(); // 获取池子里面token数量

function quote(); // 添加流动性时，给定 amountA ，求解 amountB

function getAmountOut(); // 场景为交易时，
// out = in * 997 * rOut / (rIn*1000 + in * 997)

function getAmountIn();

// 下面两个带有path
function getAmountsOut();
function getAmountsIn();
```

route

```solidity
function addLiquidity();
	// 先计算能够交换的量，再进行交换，最后给LP token
function _addLiquidity();
function removeLiquidity();
function _removeLiquidity();

function  removeLiquidityETH();
function  removeLiquidityWithPermit();
function  removeLiquidityETHWithPermit();
// 以上3个remove内部还是调用removeLiquidity
// 带ETH的区别是，调用接收币是router，然后由路由将weth转换成eth，后将两笔币转发给用户
// 带 Permit是EIP712 带签名信息，验证签名后，Premit里面会授权，将授权和移除在一个交易内完成

function removeLiquidityETHSupportingFeeOnTransferTokens();
function removeLiquidityETHWithPermitSupportingFeeOnTransferTokens();
// 这两个方法名带ETH ，调用removeLiquidity的时候，接收者是router，由路由再转给用户
// 这两个和removeLiquidityETH的区别是：


function swapExactTokensForTokens();
function swapTokensForExactTokens();
// 输入值精确，参数中会有个最小输出，作为交易限制，没有达到amountOutMin, 交易失败
// 输出值精确，参数中会有一个amountInMax，购买精确输出时最大允许支付这个值，否则交易失败


```



## V3

主要是集中流动性这个概念，而且代码已经非常复杂了。

流动性概念非常不好理解，资产的范围。

tick的概念相当于是把price给划分为一系列的等比数列。而 tick_space 则是间隔，将间隔内的tick视为一个来计算





不理解 LP 注入流动性时候，期望的价格范围为[Pa, Pb]，选择的池子比如说 Loop(token0, token1, fee, tickSpace)，此时该池子的价格为 Pc，这样怎么理解？LP在注入之后，应该不会导致池子价格波动。

但是池子当前价格`Pc`这个值怎么确定呢？



[Uniswap v3 详解（三）：交易过程 - Fantasy (paco0x.org)](https://paco0x.org/uniswap-v3-3/)



## 参考

Dapp-learning-dao的 V2 V3的笔记。











