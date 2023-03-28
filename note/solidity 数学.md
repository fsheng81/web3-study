solidity 数学



主要是记录遇到的数学问题，因为solidity目前的运算都是基于整数，目前还没有完全支持浮点数。小数的计算大多是通过定点数(fixed point number)来实现，因此相应的数学操作和过去习惯的浮点数有一点区别。



## log计算

了解计算过程，同时也通过Python来辅助校验计算结果。

python路径：web3-study/code/solidityMath/python

从10进制数转换到 Q64.64的数：

```python
210 * (1 << 64)
3873816255479005839360
# 16进制表示
hex(3873816255479005839360)
0xd20000000000000000 # 0xd2 + 64个0 符合预期
```

把Q64.64的数转为10进制数

```python
3873816255479005839360 / (1 << 64)
```

计算log的整数部分：$2^{n} \leq x < 2^{n + 1}$，n 就是整数部分。采用位运算+二分法来找到最高位bit，也就是

的值。

```python
def log_2_q64_64(x):
    msb = 0
    xc = x
    if (xc >= 0x10000000000000000): # 16个0，一共16*4=64位
        xc >>= 64; msb += 64
    if (xc >= 0x100000000):
        xc >>= 32; msb += 32
    if (xc >= 0x10000):
        xc >>= 16; msb += 16
    if (xc >= 0x100):
        xc >>= 8; msb += 8
    if (xc >= 0x10):
        xc >>= 4; msb += 4
    if (xc >= 0x4):
        xc >>= 2; msb += 2
    if (xc >= 0x2):
        msb += 1

    result = msb - 64 << 64
    return result
```

对于执行结果转回十进制：

```python
input = 210
x = input * (1 << 64) # in Q64.64
# function definition body
y = log_2_q64_64(x)
z = y / (1 << 64)
print(y) # 129127208515966861312
print(z) # 7.0
```

计算小数部分：$u_x = \log_{2}{x} - n = \log_{2}{\frac{x}{2^n}}$，也就是另一个$log_2$计算，并且输入值在$[1, 2)$之间。

```python
# 首先是基本版本
# x待处理数，n迭代次数
def log2(x, n):
    assert 1 <= x < 2

    result = 0
    for i in range(0, n):
        if x >= 2:
            result += 1 / (2 ** i)
            x /= 2
        x *= x
    return result
```

然后是优化版本，用于计算Q64.64的64位小数位：

```python
def log_2_q64_64(x):
    #### .....

    result = msb - 64 << 64
    ux = x << (127 - msb)
    bit = 0x8000000000000000 # 1/2 in Q64.64
    while (bit > 0):
        ux *= ux
        b = ux >> 255
        ux >>= 127 + b
        result += bit * b
        bit >>= 1
    return result
```

执行结果：

```python
input = 210
x = input * (1 << 64) # in Q64.64

y = log_2_q64_64(x)
z = y / (1 << 64)
print(hex(y)) # 
print(z) # 
print(math.log(210, 2)) # 
# 结果
# 0x7b6d8cb53b0ca4ecb
# 7.714245517666122
# 7.714245517666122
```

总的函数实现在：web3-study/code/solidityMath/python/log_solidity.py

参考：https://paco0x.org/logarithm-in-solidity/

参考：[ABDK Library](https://github.com/abdk-consulting/abdk-libraries-solidity/blob/d8817cb600381319992d7caa038bf4faceb1097f/ABDKMath64x64.sol#L460-L500)

### uniswap的log计算



## 指数计算

目标：$y = 2^x$ ，其中$x$是Q64.64形式的小数

计算：将$x = n + f$分为整数部分与小数部分，把小数部分按照2为底拆分：

$f = f_i * 2^{-1} + f_2 * 2^{-2} + ..., f_i = {0, 1}$

由于都是2的负指数，所以都是开根号的形式：

$2^{f} = \prod{2^{f_i * 2^{-i}}}$ ，如果$f_i = 1, $ 则需要乘上一个 $2^{2^{-i}}$ 

```solidity
function exp_2 (int128 x) internal pure returns (int128) {
    unchecked {
      require (x < 0x400000000000000000); // Overflow
      if (x < -0x400000000000000000) return 0; // Underflow
	  
	  // 0.5 in Q128.128
      uint256 result = 0x80000000000000000000000000000000;

	  // 依次判断小数位是否为1，即 f_i == 1
      if (x & 0x8000000000000000 > 0)
        // 1.0 * 0x16A09E667F3BCC908B2FB1366EA957D3E / (1 << 128)
        // = 1.4142135623730951 = 2^(-1)
        result = result * 0x16A09E667F3BCC908B2FB1366EA957D3E >> 128;
      if (x & 0x4000000000000000 > 0)
        result = result * 0x1306FE0A31B7152DE8D5A46305C85EDEC >> 128;
	  // ......
      if (x & 0x1 > 0)
        result = result * 0x10000000000000000B17217F7D1CF79AB >> 128;

	  // 把128位小数转成64位小数
      result >>= uint256 (int256 (63 - (x >> 64)));
      require (result <= uint256 (int256 (MAX_64x64)));

      return int128 (int256 (result));
    }
  }

```

https://github.com/abdk-consulting/abdk-libraries-solidity/blob/master/ABDKMath64x64.sol

### uniswap的指数计算

uniswap中需要计算的指数计算是从tick计算sqrtprice：$sqrtPrice = 1.0001^{\frac{tick}{2}}$ 

可以通过换底的方式改成以2为指数底：$1.0001^{\frac{tick}{2}} = 2^{\frac{tick}{2}\log_{2}{1.0001}}$

代码注解：

```solidity
function getSqrtRatioAtTick(int24 tick) internal pure returns (uint160 sqrtPriceX96) {
	uint256 absTick = tick < 0 ? uint256(-int256(tick)) : uint256(int256(tick));
	require(absTick <= uint256(uint24(MAX_TICK)), 'T');

	// 1.0 in Q128.128
	uint256 ratio = 0x100000000000000000000000000000000;

	// 0xfffcb933bd6fad37aa2d162d1a594001 * 1.0 / (1 << 128)
	// = 0.9999500037496876 = pow(1.0001, -0.5)
	// 0xfff97272373d413259a46990580e213a * 1.0 / (1 << 128)
	// = 0.9999000099990001 = pow(1.0001, -1.0)
	if (absTick & 0x1 != 0) ratio = (ratio * 0xfffcb933bd6fad37aa2d162d1a594001) >> 128;
	if (absTick & 0x2 != 0) ratio = (ratio * 0xfff97272373d413259a46990580e213a) >> 128;
	if (absTick & 0x4 != 0) ratio = (ratio * 0xfff2e50f5f656932ef12357cf3c7fdcc) >> 128;
	// ........
	if (absTick & 0x80000 != 0) ratio = (ratio * 0x48a170391f7dc42444e8fa2) >> 128;
	// 由于tick的范围是int24 因此最多只到23位即可

	// 如果tick是正数，则需要取导数
	if (tick > 0) ratio = type(uint256).max / ratio;
	// 把 Q128.128 转换为 Q64.96 改变小数位
	sqrtPriceX96 = uint160((ratio >> 32) + (ratio % (1 << 32) == 0 ? 0 : 1));
}
```

