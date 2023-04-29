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
    let nmk: Vec<i64> = ls2vec!(input!());
    let _n: i64 = nmk[0];
    let _m: i64 = nmk[1];
    let k: i64 = nmk[2];
    let mut a_n: Vec<i64> = ls2vec!(input!());
    let mut b_m: Vec<i64> = ls2vec!(input!());
    let mut s: i64 = 0;

    a_n.sort();
    b_m.sort();

    let mut a_n_it = a_n.iter();
    let mut b_m_it = b_m.iter();
 
    let mut a_curr = a_n_it.next();
    let mut b_curr = b_m_it.next();

    while a_curr != None && b_curr != None {
        match a_curr {
            Some(a) => match b_curr {
                Some(b) => {
                    if a.abs_diff(*b) <= k as u64 {
                        a_curr = a_n_it.next();
                        b_curr = b_m_it.next();
                        s += 1;
                        continue;
                    }

                    if a - b > k {
                        b_curr = b_m_it.next();
                    } else {
                        a_curr = a_n_it.next();
                    }
                } None => panic!("no more apartments")
            } None => panic!("no more applicants")
        }
    }

    println!("{}", s);
}