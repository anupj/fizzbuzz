fn main() {
    // call fizzbuzz implemented using match
    fizzbuzz_match(20);
}

// From https://chrismorgan.info/blog/rust-fizzbuzz/
fn fizzbuzz_match(num: u32) {
    for i in 1..num + 1 {
        match (i % 3, i % 5) {
            (0, 0) => println!("FizzBuzz"),
            (_, 0) => println!("Buzz"),
            (0, _) => println!("Fizz"),
            (_, _) => println!("{}", i),
        }
    }
}
