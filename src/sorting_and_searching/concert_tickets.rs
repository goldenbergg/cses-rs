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

use std::cmp::Ordering;

fn main() {
    let nm: Vec<i64> = ls2vec!(input!());
    let _n: i64 = nm[0];
    let _m: i64 = nm[1];
    let mut h_n: Vec<i64> = ls2vec!(input!());
    let t_m: Vec<i64> = ls2vec!(input!());

    h_n.sort_unstable();

    for t in t_m {
        let mut j: usize = h_n.binary_search_by(|e| match e.cmp(&t) {
            Ordering::Equal => Ordering::Less,
            ord => ord,
        }).unwrap_err();
        if j == 0 {
            println!("-1");
        } else {
            j -= 1;
            println!("{}", h_n[j]);
            h_n.remove(j);
        }
    }
}
