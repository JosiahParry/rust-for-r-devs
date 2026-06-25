fn main() {
    // let i = 15; // FizzBuzz
    // let i = 3; // Fizz
    // let i = 5; // Buzz
    let i = 47; // Nothing
    if (i % 3 == 0) && (i % 5 == 0) {
        println!("FizzBuzz");
    } else if i % 3 == 0 {
        println!("Fizz");
    } else if i % 5 == 0 {
        println!("Buzz");
    }
}
