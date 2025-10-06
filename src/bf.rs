use m4ri_rust::friendly::*;

// TODO: decode by just trying random haming weight vectors of incresing weight
fn decode_bf(H: BinMatrix, s: BinVector, w: usize) -> Option<BinVector>
{
    None
}

// TODO implement Dummer algo : simple MITM
// we want to find e of low weight s.t H * e^T = s
// instead of bruteforce we can split H and e in two : H_a * e_a^T + H_b * e_b^T = s
// rearanging we can then meet in the middle with a hash table for H_a * e_a^T  = s - H_b * e_b^T
// by storing values of H_a * e_a^T  for random e_a of incresing weight and comparing to s - H_b * e_b^T for random e_b again