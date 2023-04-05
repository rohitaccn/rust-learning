use std::fmt::Display;
//Similar to trait bounds, an impl Trait syntax can be used in function arguments and return values:
//impl Trait allows you to work with types which you cannot name.
fn get_x(name: impl Display) -> impl Display {
    format!("Hello {name}")
}

fn main() {
    let x = get_x("foo");
    println!("{x}");
}