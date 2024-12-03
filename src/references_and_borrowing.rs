// Till now we have learnt that to use the string or any dynamic object, 
// if we are passing it to the functions then we have to return it.

// Solution : instead of moving the object in argument pass its reference in the argument

pub fn references() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);  // ampersands represent references its opposite is dereference operator, *.
    
    println!("Length is {len}");
}

fn calculate_length(s: &String) -> usize {
    s.len()  // Here, s goes out of scope. But because it does not have ownership of what
    // it refers to, it is not dropped.
}