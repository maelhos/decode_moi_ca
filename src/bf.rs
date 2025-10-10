use crate::hamming_iter::HammingIter;
use rug::{Complete, Integer};

pub fn decode_bf(H: &Vec<Integer>, s: &Integer, k: usize, n: usize, w: usize) -> Option<Integer> {
    for tw in 1..=w {
        let hi = HammingIter::new(n, tw);

        for e in hi {
            let mut is_ok = true;

            for (index, h_row) in H.iter().enumerate() {
                let c_bit = (h_row & &e).complete().count_ones().unwrap() & 1;
                let t_bit = s.get_bit(index as u32) as u32;

                if c_bit != t_bit {
                    is_ok = false;
                    break;
                }
            }

            if is_ok {
                return Some(e);
            }
        }
    }
    // If no error vector of weight <= w produces the syndrome 's', return None.
    None
}
