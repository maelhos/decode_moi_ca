from sage.all import *

F2 = GF(2)

def parsevec(v, n):
    l = ZZ(v).bits()
    return vector(F2, (l + [0] * (n - len(l)))[::-1])

H = [
0b1101111110,
0b0100101001,
0b1011101110,
0b0010110100,
0b0001001101,
0b1010110110,
0b1000011000,
0b0001100111,
0b1101011000,
0b1001011000
     ]
# s^transpose
s = 0b0000000101
e = 0b00000001010000000000
n = 20
k = n // 2

s = parsevec(s, k)
H = matrix.identity(k).augment(matrix(F2, map(lambda x: parsevec(x, k), H)))
print(H)

print()
e = parsevec(e, n)

print(H * e == s)