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
    let q: i64 = input!().parse().unwrap();

    for _ in 0..q {
        let mut k: i64 = input!().parse().unwrap();
        let mut len: i64 = 1;

        while k > len * 9 * 10i64.pow(len as u32 - 1) {
            k -= len * 9 * 10i64.pow(len as u32 - 1);
            len += 1;
        }

        let region: i64 = (k - 1) / len + 10i64.pow(len as u32 - 1);
        let idx: usize = ((k - 1) % len) as usize;
        let region_str = region.to_string();

        println!("{}", region_str.as_bytes()[idx] as char);
    }
}
    