use std::env;

use crate::bf::decode_bf;
mod bf;
mod parser;
mod hamming_iter;

fn main() {
    let path = env::args().nth(1).expect("No path given.");
    let (H, s, w, k) = parser::parse_challenge(path);

    println!("Succesfuly parsed, k = {:?}, w = {:?}, s = {:?}, H = {:?}", k, w, s, H);

    match decode_bf(H, s, k, w) {
        None => println!("No solution found..."),
        Some(s) => println!("Solution found : H*{:?} = s", s.to_string_radix(2))
    }
}
