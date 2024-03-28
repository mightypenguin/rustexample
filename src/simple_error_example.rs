use std::error::Error;

use simple_error::bail;

//Source: https://stevedonovan.github.io/rust-gentle-intro/6-error-handling.html

type BoxResult<T> = Result<T,Box<dyn Error>>;

pub fn run(s: &str) -> BoxResult<i32> {
    if s.len() == 0 {
        bail!("empty string");
    }
    Ok(s.trim().parse()?)
}

// fn main() {
//     println!("{:?}", run("23"));
//     println!("{:?}", run("2x"));
//     println!("{:?}", run(""));
// }
// Ok(23)
// Err(ParseIntError { kind: InvalidDigit })
// Err(StringError("empty string"))
