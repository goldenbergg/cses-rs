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
 
#[allow(dead_code)]
fn nck(n: u64, k: u64) -> u64 {
    if k == 0 {
        return 1
    } else if k > n / 2 {
        return nck(n, n - k)
    } else {
        return n * nck(n - 1, k - 1) / k
    }
}

use std::cmp::{max, min};

fn main() {
    let t: i64 = input!().parse().unwrap();

    for _ in 1..=t {
        let v: Vec<i64> = ls2vec!(input!());
        let (a, b) = (v[0], v[1]);

        if (a == 0 || b == 0) && a != b {
            println!("NO");
        } else if (a + b) % 3 != 0 {
            println!("NO");
        } else if 2 * min(a, b) >= max(a, b) {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
