
# Décode moi ça

This projet aims at implementing state of the art decoding algorithms for **fun** in *Rust*.

## Installing `Décode moi ça`

To install `Décode moi ça`, you only need a working *rust* / *cargo* install.
Then just run the *build.sh* script.

```bash
chmod +x build.sh && ./build.sh
```

This will create the **decode_moi_ca** binary which just takes a challenge file as input

```bash
./decode_moi_ca challs/chall5
```

## Getting the challenge files

The challenge files from [Decoding Challenge](https://decodingchallenge.org/syndrome) can be downloaded by just running the *get_challs.sh* script.

```bash
chmod +x get_challs.sh && ./get_challs.sh
```

## Testing the output

Testing that the recoverd error vector found by `décode moi ça` is correct can be done by running the `check.py` script provided you have a working [Sagemath](https://github.com/sagemath/sage) install.

Just run

```bash
python check.py challs/chall5
```

And input the error vector you want to verify, you'll then get *True* or *False* printed to the screen depending on the validity of the error vector.

## TODO List

For now only syndrome decoding is implemented, we might also want to do LWC.

- [x] Parser
- [x] Hamming weight iterator
- [x] Basic bruteforce search
- [ ] Randomized search
- [ ] Dummer algorithm
- [ ] Better datastructure / memory management / custom storage types
- [ ] MMT
- [ ] Prange
- [ ] Benchmark
- [ ] Multi-thread

## Records

Current record is `chall7` corresponding to **n = 70** and **w = 11** with *Iterative search*

```text
$ time ./decode_moi_ca challs/chall6
Succesfuly parsed, k = 30, w = 10
Solution found : H*"010100100001000000001001000000000000000000000100000000000000" = s

real    0m16,611s
user    0m16,589s
sys     0m0,004s
```

## Contributors

- Fernando Leal
- Maël Hostettler
