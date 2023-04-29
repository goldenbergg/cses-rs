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

#[allow(unused_variables)]
fn main() {
    let n: i64 = input!().parse().unwrap();
    let mut nums: Vec<i64> = ls2vec!(input!());
    let mut x_prev: i64 = 0;
    let mut moves: i64 = 0;
    
    for (i, x) in nums.iter_mut().enumerate() {
        if i != 0 {
            if x_prev > *x {
                moves += x_prev - *x;
                *x = x_prev;
            }
        }

        x_prev = *x;
    }

    print!("{}", moves);
}