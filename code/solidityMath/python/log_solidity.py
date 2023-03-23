# 计算 210 转 Q64.64
# 计算 log 2 of 210
import math;
input = 210
x = input * (1 << 64) # in Q64.64

def log_2_q64_64(x):
    # 整数部分
    msb = 0 # 最大bit位
    xc = x
    if (xc >= 0x10000000000000000):
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

    # 小数部分
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

y = log_2_q64_64(x)
z = y / (1 << 64)
print(hex(y))
print(z)
print(math.log(210, 2))