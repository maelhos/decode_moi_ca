use m4ri_rust::friendly::*;
use vob::Vob;

pub struct HammingIter {
    k: usize,
    w: usize,
    one: BinVector,
    curr: BinVector,
}

impl HammingIter {
    fn new(k: usize, w: usize) -> HammingIter {
        let bv = BinVector::with_capacity(k);
        let mut one = BinVector::with_capacity(k);
        one.set(0, true);
        HammingIter { k: k, w: w, one: one, curr: bv }
    }
}

impl Iterator for HammingIter {
    type Item = BinVector;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: implement some sort of iterator by hamming weight
        None
    }
}
