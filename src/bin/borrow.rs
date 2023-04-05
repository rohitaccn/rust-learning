fn main() {
    let s = String::from("hello"); // s comes into scope

    let len = calculate_length(&s); // borrow s, no ownership is transferred

    println!("The length of '{}' is {}.", s, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}