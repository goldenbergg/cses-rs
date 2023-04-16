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

fn main() {
    let seq = input!();
    let mut max_len: i64 = 0;
    let mut len: i64 = 0;
    let mut c_prev: char = 'Z';

    for c in seq.chars() {
        if len == 0 {
            len += 1;
            c_prev = c;
        } else {
            if c == c_prev {
                len += 1;
            } else {
                if len > max_len {
                    max_len = len;
                }

                len = 1;
                c_prev = c;
            }
        }
    }

    if len > max_len {
        max_len = len;
    }

    print!("{}", max_len);
}