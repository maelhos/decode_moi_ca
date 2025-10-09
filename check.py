from sage.all import *
import sys

if len(sys.argv) < 2:
    print(f"{sys.argv[0]} expects challenge's file name as argument ")
    exit(1)

F2 = GF(2)

def parsevec(v, n):
    l = list(map(int, list(v)))
    assert len(l) == n
    return vector(F2, l)


with open(sys.argv[1], "r") as f:
    f.readline()
    n = int(f.readline())
    print(f"{n = }")
    f.readline(); f.readline(); f.readline()
    w = int(f.readline())
    print(f"{w = }")
    f.readline()

    k = n // 2
    print(f"{k = }")
    H = []

    for i in range(k):
        H.append(parsevec(f.readline().strip(), k))

    f.readline()
    s = parsevec(f.readline().strip(), k)
    print(f"{s = }")

    #H = matrix.identity(k).augment(matrix(F2, H))
    H = matrix.identity(k).augment(matrix(F2, H).transpose())
    print(H)

    print()
    for v in H:
        print(str(v[::-1])[1:-1].replace(", ",""))
    print()
    e = input("e > ")
    e = parsevec(e, n)

    print(H * e == s and list(e).count(1) <= w)
