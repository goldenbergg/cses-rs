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

use std::collections::HashMap;

fn main() {
    let nx: Vec<i64> = ls2vec!(input!());
    let _n: i64 = nx[0];
    let x: i64 = nx[1];

    let a_n: Vec<i64> = ls2vec!(input!());
    let mut map: HashMap<i64, i64> = HashMap::new();

    for (idx, a) in a_n.iter().enumerate() {
        if map.contains_key(&(x - a)) {
            println!("{} {}", map.get(&(x - a)).unwrap(), idx + 1);
            return;
        }

        *map.entry(*a).or_default() = idx as i64 + 1;
    }

    println!("IMPOSSIBLE");
}