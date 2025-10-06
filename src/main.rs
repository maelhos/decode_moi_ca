use std::env;
mod bf;
mod parser;
mod hamming_iter;

fn main() {
    let path = env::args().nth(1).expect("No path given.");
    let (H, s, k, w) = parser::parse_challenge(path);

    println!("Succesfuly parsed, k = {:?}, w = {:?}, s = {:?}, H = {:?}", k, w, s, H);
}
