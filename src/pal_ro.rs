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

use std::collections::HashMap;
use std::iter::repeat;

fn main() {
    let s: String = input!();

    let mut h: HashMap<char, usize> = HashMap::new();
        
    for c in s.chars() {
        *h.entry(c.to_owned()).or_default() += 1;
    }

    let mut p = String::new();
    let mut pivot: (char, usize) = ('$', 0);

    for k in h.keys() {
        if h[k] % 2 == 0 {
            p.push_str( 
                &repeat(k)
                .take(h[k] / 2)
                .collect::<String>()
            );
        } else {
            if s.len() % 2 == 0 {
                println!("NO SOLUTION");
                return
            } else {
                if pivot.0 == '$' {
                    pivot = (*k, h[k]);
                } else {
                    println!("NO SOLUTION");
                    return
                }
            }
        }
    }

    if s.len() % 2 == 0 {
        println!("{}{}", p, p.chars().rev().collect::<String>());
    } else {
        println!(
            "{}{}{}", 
            p, 
            repeat(pivot.0)
            .take(pivot.1)
            .collect::<String>(),
            p.chars().rev().collect::<String>()
        );
    }
}
