pub fn ownership() {
    let string_slice: &str = "Hello, Rust!"; // this is string literal, immutable

    let mut s = String::from("hello"); // String, mutable also
    
    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{s}"); // This will print `hello, world!`

}