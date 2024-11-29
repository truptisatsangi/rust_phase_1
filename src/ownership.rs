pub fn ownership() {
    let string_slice: &str = "Hello, Rust!"; // this is string literal, immutable

    let mut s = String::from("hello"); // String, mutable also
    
    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{s}"); // This will print `hello, world!`

    // We can't copy the string to another one, if we do this then rust has to free up same memory twice which is a double
    // free error, due to this what rust do when you copy string to another one it consider first one as invalid.
    
    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{s1}, world!"); // error

    // But if we still want to copy the string we can use clone which is expensive
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");

}