#[macro_export]
macro_rules! input {
    () => {
        {   
            use std::io;
            let mut buf = String::new();
            let stdin = io::stdin();
            stdin.read_line(&mut buf).expect("unable to read line");
            buf.strip_suffix("\r\n")
                .or(buf.strip_suffix("\n"))
                .unwrap_or(&buf)
                .to_string()
        }   
    };
}

#[macro_export]
macro_rules! ls2vec {
    ($x:expr) => {
        {
            let ls = $x.to_string();
            ls.split(" ")
                .map(|x| x.parse().unwrap())
                .collect()
        }
    };
}

use std::cmp;
fn main() {
    let t: i64 = input!().parse().unwrap();
    for _ in 0..t {
        let coord: Vec<i64> = ls2vec!(input!());
        let y = coord[0];
        let x = coord[1];
        let max = cmp::max(y, x);
        let max_sq = (max - 1).pow(2);
        
        if max % 2 == 0 {
            if max != x {
                println!("{}", (2 * max) + max_sq - x);
            } else {
                println!("{}", max_sq + y);
            }
        } else {
            if max != y {
                println!("{}", (2 * max) + max_sq - y);
            } else {
                println!("{}", max_sq + x);
            }
        }
    }
}
