use std::{env};

use crate::bf::decode_bf;
mod bf;
mod parser;
mod hamming_iter;

fn main() {
    let path = env::args().nth(1).expect("No path given.");
    let (H, s, w, k, n) = parser::parse_challenge(path);

    println!("Succesfuly parsed, k = {:?}, w = {:?}", k, w);
    
    match decode_bf(&H, &s, n, w) {
        None => println!("No solution found..."),
        Some(e) => {
            let e_str = e.to_string_radix(2);
            let mut ret: String = (0..n - e_str.len()).map(|_| "0").collect::<String>();
            ret.push_str(&e_str);
            println!("Solution found : H*{:?} = s", ret.chars().rev().collect::<String>())}
    }
}
