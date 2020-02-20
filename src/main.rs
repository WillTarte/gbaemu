use gbaemulib::instructions;
use gbaemulib::registers;

mod gbaemulib;

fn main() {
    print!("{}", "Hello World");
    let (r, ov) = 5u8.overflowing_add(2);
    println!("{}{}", r, ov);
}