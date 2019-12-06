mod four;
mod one;
mod three;
mod two;

fn main() {
    println!("Hello, world!");

    println!("one-a: {}", one::one_a());
    println!("one-b: {}", one::one_b());

    println!("two-a: {}", two::two_a());
    println!("two-b: {}", two::two_b());

    println!("three-a: {}", three::three_a());
    println!("three-b: {}", three::three_b());

    println!("four-a: {}", four::four_a());
    println!("four-b: {}", four::four_b());
}
