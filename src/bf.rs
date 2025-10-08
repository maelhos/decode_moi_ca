use rug::{Complete, Integer};

use crate::hamming_iter::HammingIter;
// TODO: decode by just trying random haming weight vectors of incresing weight

pub fn decode_bf(H: Vec<Integer>, s: Integer, k: usize, w: usize) -> Option<Integer>
{   
    for tw in 2..w {
        let hi = HammingIter::new(k, tw);

        for v in hi {
            println!("Trying : {:?}", v.to_string_radix(2));
            let mut good = true;

            for (index, hm) in H.iter().enumerate() {
                if ((hm ^ &v).complete().count_ones().unwrap().is_multiple_of(2)) ^ s.get_bit(index as u32) {
                    good = false;
                    break
                }
            }
            if good { return Some(v); }
        }
    }
    None
}

// TODO implement Dummer algo : simple MITM
// we want to find e of low weight s.t H * e^T = s
// instead of bruteforce we can split H and e in two : H_a * e_a^T + H_b * e_b^T = s
// rearanging we can then meet in the middle with a hash table for H_a * e_a^T  = s - H_b * e_b^T
// by storing values of H_a * e_a^T  for random e_a of incresing weight and comparing to s - H_b * e_b^T for random e_b again