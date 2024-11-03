extern "C" {
    fn add_numbers(a: i32, b: i32) -> i32;
    fn greet();
}

fn main() {
    unsafe { // Why "unsafe"?
        greet();
        println!("The sum from C is: {}", add_numbers(5, 7));
    }
}
