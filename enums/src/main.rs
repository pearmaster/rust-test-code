
#![allow(dead_code)]

enum Fruit {
    Bananas,
    Apples = 2,
    Pears = 3
}

fn main() {
    println!("Hello {}", Fruit::Bananas as u32);
    println!("Hello {}", Fruit::Apples as u32);
}
