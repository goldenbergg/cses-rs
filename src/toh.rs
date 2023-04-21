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

fn toh(n: u64, l: u64, m: u64, r: u64) {
    if n == 1 {
        println!("{} {}", l, r);
        return
    }

    toh(n - 1, l, r, m);
    println!("{} {}", l, r);
    toh(n - 1, m, l, r);
}

fn main() {
    let n: u64 = input!().parse().unwrap();

    println!("{}", (1 << n) - 1);
    toh(n, 1, 2, 3);
}