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
 
fn main() {
    let nx: Vec<i64> = ls2vec!(input!());
    let _n: i64 = nx[0];
    let x: i64 = nx[1];
    let mut p_n: Vec<i64> = ls2vec!(input!());

    let mut g: i64 = 0;
    let mut seen = vec![false; p_n.len() as usize];
    
    p_n.sort();

    let mut fwd_idx: usize = 0;
    let mut bwd_idx: usize = p_n.len() - 1;

    while fwd_idx < bwd_idx {
        if p_n[fwd_idx] + p_n[bwd_idx] <= x {
            g += 1;
            seen[fwd_idx] = true;
            seen[bwd_idx] = true;
            fwd_idx += 1;
        }

        bwd_idx -= 1;
    }

    for c in seen {
        if !c {
            g += 1;
        }
    }

    println!("{}", g);
}