use rug::{Complete, Integer};
use crate::hamming_iter::HammingIter;

pub fn decode_bf(H: &Vec<Integer>, s: &Integer, n: usize, w: usize) -> Option<Integer> {
    for tw in 1..=w {
        let hi = HammingIter::new(n, tw);

        for e in hi {
            let mut is_ok = true;

            for (index, h_row) in H.iter().enumerate() {
                let c_bit = (h_row & &e).complete().count_ones().unwrap() & 1;
                let t_bit = s.get_bit(index as u32) as u32;

                if c_bit != t_bit {
                    is_ok = false;
                    break
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

// TODO implement Dummer algo : simple MITM
// we want to find e of low weight s.t H * e^T = s
// instead of bruteforce we can split H and e in two : H_a * e_a^T + H_b * e_b^T = s
// rearanging we can then meet in the middle with a hash table for H_a * e_a^T  = s - H_b * e_b^T
// by storing values of H_a * e_a^T  for random e_a of incresing weight and comparing to s - H_b * e_b^T for random e_b again