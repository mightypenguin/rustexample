extern crate simple_error;

use regex::Regex;
use base64::{Engine as _, engine::general_purpose};

mod example_traits;
mod simple_error_example;
use crate::example_traits::HasArea;

fn main() {
    // Simple String printing
    let s = "Hello, world";
    println!("{}, len={}\n",s, s.len());


    // Regex example
    let re = Regex::new(r"Hello").unwrap();
    let matched = re.is_match(s);
    println!("Did we match? {}\n", matched);


    // Get program argv
    let arg = std::env::args()
        .next().expect("Need at least 1 arg");
    println!("Argv: {}\n",arg);


    // Borrow checker annoying us by "consuming" things
    // Examples: https://doc.rust-lang.org/rust-by-example/fn/closures/capture.html
    let dat = "some data I want to split";
    let dat_array = dat.split_whitespace();
    println!("Original string: '{}'", dat);

    // Notice the ".clone()"
    for (i, el) in dat_array.clone().enumerate() { 
        println!("Array index:{} Items: {:?}", i, el);
    }
    println!("Size:{}\n",dat_array.count());


    // String comparisons
    let s1 = "something";
    let s2 = "something";
    println!("Are raw &str ==? {}", (s1==s2));

    let c1: String = String::from("something");
    let c2: String = String::from("something");
    println!("Are String == ? {}", (c1==c2));
    println!("Are String.eq == ? {}", c1.eq(&c2));
    println!("Are String == &str? {}\n", (c1==s2));
    // Rust does not want you comparing references!

    let b64 = general_purpose::STANDARD.encode(s1);
    println!("Base64 version of '{}'='{}'\n",s1, b64);


    // Error handling
    println!("{:?}", simple_error_example::run("23"));
    println!("{:?}", simple_error_example::run("2x"));
    println!("{:?}", simple_error_example::run(""));

    let ret = match simple_error_example::run("2x") {
        Ok(num) => num,
        Err(e) => {
            println!("Handle that error! Err:{}",e);
            // panic!();
            -1 //return value without ";"
        }
    };
    println!("Error handling ret={}",ret);
    println!("");

    
    // Traits
    let c = example_traits::Circle {
        x: 0.0f64,
        y: 0.0f64,
        radius: 4.2f64
    };
    println!("Circle's area is:{}", c.area());


    // Async / await
    // Need to use an Async Runtime
    // https://rust-lang.github.io/async-book/08_ecosystem/00_chapter.html

    // Discussion vs Go coroutines:
    // https://www.reddit.com/r/rust/comments/u8uw3z/is_tokio_slow_or_is_it_just_being_compared/

}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4,"We expect 4 == 4");
    }

    #[test]
    fn it_fails() {
        let result = 2 + 2;
        assert_eq!(result, 5, "We do NOT expect 4 == 5");
    }
}

