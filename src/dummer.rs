use crate::hamming_iter::HammingIter;
use rug::{Complete, Integer};
use std::collections::HashMap;

fn mul(M: &Vec<Integer>, v: &Integer) -> Integer {
    Integer::from_digits(
        M.iter()
            .map(|row| (row & v).complete().count_ones().unwrap() & 1 == 1)
            .collect::<Vec<bool>>()
            .as_ref(),
        rug::integer::Order::Lsf,
    )
}

fn mul_weight(M: &Vec<Integer>, v: &Integer) -> u32 {
    M.iter()
        .map(|row| (row & v).complete().count_ones().unwrap() & 1)
        .sum()
}

pub fn decode_dummer(
    H: &Vec<Integer>,
    s: &Integer,
    k: usize,
    n: usize,
    w: usize,
) -> Option<Integer> {
    // get first part of H (the rest is just identity)
    // H_a * e_a + H_b*e_b = s
    // H_a * e_a + e_b = s

    let H1 = H
        .iter()
        .map(|x| (x >> k).complete())
        .collect::<Vec<Integer>>();

    let w1 = ((w + 1) / 2);
    let w2 = w - w1;

    for tw in 1..=w1 {
        let hi = HammingIter::new(k, tw);

        for e in hi {
            let m = mul(&H1, &e);
            let x = m ^ s;
            if x.count_ones().unwrap() <= w2 as u32 {
                return Some(x | e << k)
            }
        }
    }
    // If no error vector of weight <= w produces the syndrome 's', return None.
    None
}
