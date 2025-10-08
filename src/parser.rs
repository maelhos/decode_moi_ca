use rug::{Complete, Integer};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn parse_challenge(path: String) -> (Vec<Integer>, Integer, usize, usize) {
    // Parse the file
    let mut lines = read_lines(&path).expect("Wrong file format.");

    // First line is always "# n"
    if let Some(Ok(l)) = lines.next() {
        assert_eq!(l, "# n", "Wrong file format.")
    }

    // Second line is always the value of n
    let n = lines.next()
    .and_then(|r| r.ok())
    .and_then(|line| line.parse::<usize>().ok())
    .unwrap_or_else(|| {
        println!("Wrong file format.");
        std::process::exit(1); // or return a default/error value
    });

    // First line is always "# seed"
    if let Some(Ok(l)) = lines.next() {
        assert_eq!(l, "# seed", "Wrong file format.")
    }

    // Fourth line is always the value of the (for now unused) seed
    let _seed = lines.next()
    .and_then(|r| r.ok())
    .and_then(|line| line.parse::<usize>().ok())
    .unwrap_or_else(|| {
        println!("Wrong file format.");
        std::process::exit(1); // or return a default/error value
    });

    // Fith line is always "# w"
    if let Some(Ok(l)) = lines.next() {
        assert_eq!(l, "# w", "Wrong file format.")
    }

    // Sixth line is always the value of the weight
    let w = lines.next()
    .and_then(|r| r.ok())
    .and_then(|line| line.parse::<usize>().ok())
    .unwrap_or_else(|| {
        println!("Wrong file format.");
        std::process::exit(1); // or return a default/error value
    });

    // Fith line is always "# w"
    if let Some(Ok(l)) = lines.next() {
        assert_eq!(l, "# H^transpose (each line corresponds to column of H, the identity part is omitted)", "Wrong file format.")
    }

    // for now we assume regime R=0.5
    let k = n / 2;

    // Following lines are the parity check matrix
    let H = (0..k)
    .map(|_| {
        match lines.next() {
            Some(Ok(line)) if line.len() == k => {
                let a = Integer::parse_radix(line, 2);
                match a {
                    Err (_) => None,
                    Ok (val) => Some(val.complete())
                }
            }
            _ => None,
        }
    })
    .collect::<Option<Vec<Integer>>>()
    .expect(
        "Wrong file format."
    );

    // Just before last line
    if let Some(Ok(l)) = lines.next() {
        assert_eq!(l, "# s^transpose")
    }

    let s = match lines.next() {
        Some(Ok(line)) if line.len() == k => {
            let a = Integer::parse_radix(line, 2);
            match a {
                Err (_) => None,
                Ok (val) => Some(val.complete())
            }
        }
        _ => None,
    }.expect(
        "Wrong file format."
    );
    (H, s, w, k)
}