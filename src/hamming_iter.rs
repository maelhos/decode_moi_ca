use std::mem;

use rug::{Complete, Integer};

pub struct HammingIter {
    curr: Integer,
    max: Integer,
}

impl HammingIter {
    pub fn new(k: usize, w: usize) -> HammingIter {
        HammingIter {
            curr: (Integer::from(1) << w) - 1,
            max: Integer::from(1) << k,
        }
    }
}

impl Iterator for HammingIter {
    type Item = Integer;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr >= self.max {
            return None;
        }

        let old = mem::take(&mut self.curr);

        let c = &old & (-&old).complete();
        let r = (&old + &c).complete();

        self.curr = (((&r ^ &old).complete() >> 2) / &c) | &r;

        Some(old)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rug::{Integer, integer::IntegerExt64};

    #[test]
    fn test_hamming_iter_properties() {
        let k = 10;
        let w = 4;

        let iter = HammingIter::new(k, w);
        let mut previous = Integer::from(-1);

        for current in iter {
            println!("Iter : {:?}", current.to_string_radix(2));
            assert_eq!(
                current.count_ones_64().unwrap() as usize,
                w,
                "Value {} does not have weight {}",
                current,
                w
            );
            assert!(
                current > previous,
                "Sequence is not increasing: {} is not > {}",
                current,
                previous
            );
            previous = current;
        }
    }
}
