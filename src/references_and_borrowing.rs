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

// ONE MORE PROBLEM..
// We also cannot have a mutable reference while we have an immutable one to the same value.
// Users of an immutable reference don’t expect the value to suddenly change out from under them!

pub fn mutable_reference() {
    let mut s = String::from("hello");  // :: = path seperator
    let r1 = &s;
    let r2 = &s;
    // let r3 = &mut s;
    
    println!("{r1} and {r2}");

    // println!("{r3}") // ERROR


// The scopes of the immutable references r1 and r2 end after the println! where they are last used, 
// which is before the mutable reference r3 is created. These scopes don’t overlap, so this code is allowed: the compiler can tell 
// that the reference is no longer being used at a point before the end of the scope.

    let mut s = String::from("hello");  // :: = path seperator
    let r1 = &s;
    let r2 = &s;

    println!("{r1} and {r2}");

    let r3 = &mut s;
    println!("{r3}") // No problem
}

pub fn dangling_references() {
//     dangling pointer—a pointer that references a location in memory that may have been given to someone else—by freeing some memory
//      while preserving a pointer to that memory. In Rust, by contrast, the compiler guarantees that references will never be dangling references:
//       if you have a reference to some data, the compiler will ensure that the data will not go out of scope before the reference to the data does.

    let reference_to_nothing = dangle();
}


fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s // solution => return s only;
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!


