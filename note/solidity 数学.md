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

