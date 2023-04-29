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

#[allow(dead_code)]
fn next_permutation(v: &mut Vec<u8>) -> bool {
    if let Some((i, k)) = v
        .windows(2)
        .enumerate()
        .rfind(|(_, w)| w[0] < w[1])
        .map(|(j, w)| {
            let k = v[j + 1..].iter().rposition(|x| w[0] < *x).unwrap_or(0);
            (j, k)
        })
    {
        v.swap(i, k + i + 1);
        v[i + 1..].reverse();
        return true;
    }

    false
}

use std::cmp::min;

fn find_min(v: &mut Vec<i64>, l: usize, s: i64, t: i64) -> i64 {
    let r: i64;
    
    if l != 0 {
        r = min(
            find_min(v, l - 1, s + v[l - 1], t), 
            find_min(v, l - 1, s, t)
        );
    } else {
        r = (t - 2 * s).abs()
    }

    return r
}

fn main() {
    let _n: i64 = input!().parse().unwrap();
    let mut w: Vec<i64> = ls2vec!(input!());
    
    let s: i64 = w.iter().sum();
    let n: usize = w.len();
    
    let m: i64 = find_min(&mut w, n, 0, s);
    println!("{}", m);
}
    