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

// We call the action of creating a reference borrowing.

pub fn modify_references() {
    let mut s1 = String::from("hello");

    change(&mut s1);
    
// Mutable references have one big restriction: if you have a mutable reference to a value, 
// you can have no other references to that value. This code that attempts to create two mutable references to s will fail:

    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    // println!("{}, {}", r1, r2); // ERROR

    // SOLUTION
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;

    // We also cannot have a mutable reference while we have an immutable one to the same value.
}
fn change(some_string: &mut String) {
    some_string.push_str(", world");
    println!("{some_string}");
}

// The restriction preventing multiple mutable references to the same data at the same time allows for mutation but in a very controlled fashion. It’s something that new Rustaceans struggle with because most languages let you mutate whenever you’d like. The benefit of having this restriction is that Rust can prevent data races at compile time. A data race is similar to a race condition and happens when these three behaviors occur:

// Two or more pointers access the same data at the same time.
// At least one of the pointers is being used to write to the data.
// There’s no mechanism being used to synchronize access to the data.


