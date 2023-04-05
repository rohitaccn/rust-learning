
fn main() {
    // Using String
    // A String is a wrapper over a Vec<u8>.
    let mut s = String::new();
    s.push_str("hello ");
    s.push('w');
    s += "orld";

    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world); // prints "hello world"

    // Using &str
    let s = "hello world";
    let s_len = s.len();
    println!("string is '{}' and its length is {}", s, s_len);

    // Using format!
    let name = "world";
    let s = format!("Hello, {}!", name);
    println!("{}", s);

    // Borrowing
    let s = String::from("hello");
    let len = calculate_length(&s);
    println!("The length of '{}' is {}.", s, len);

    //String and &str types to help handle these complex situations correctly. Be sure to check out the documentation for useful methods like contains for searching in a string and replace for substituting parts of a string with another string.

        // Concatenating strings
        let s1 = "Hello".to_string();
        let s2 = "world!".to_string();
        let s3 = s1.clone() + &s2;
        println!("{}", s3);
        // or use `format!` macro
        let s3 = format!("{} {}", s1, s2);
        println!("{}", s3);
    
        // Substring
        let s = "Hello, world!".to_string();
        let sub = &s[7..12];
        println!("{}", sub);
    
        // Finding a substring
        let s = "Hello, world!".to_string();
        let index = s.find("world").unwrap();
        println!("{}", index);
    
        // Replacing a substring
        let s = "Hello, world!".to_string();
        let new_s = s.replace("world", "Rust");
        println!("{}", new_s);
    
        // Iterating over the characters in a string
        let s = "Hello, world!".to_string();
        for c in s.chars() {
            println!("{}", c);
        }
    
        // Removing whitespaces
        let s = "   lots of    whitespaces  ".to_string();
        let new_s = s.trim();
        println!("{}", new_s);
    
        // Splitting a string
        let s = "a,b,c,d,e";
        let v: Vec<&str> = s.split(',').collect();
        println!("{:?}", v);
    
        // Reversing a String
        let s = "Hello, world!".to_string();
        let rev_s: String = s.chars().rev().collect();
        println!("{}", rev_s);
    
    
   
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
