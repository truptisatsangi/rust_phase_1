pub fn overflow_handlers() {
    let a: u8 = 255;
    let b: u8 = 1;

    // 1. WRAPPING METHOD
    let result = a.wrapping_add(b);
    println!("Wrapping Add: {} + {} = {}", a, b, result); // Wraps to 0

    // 2. CHECKED METHOD
    match a.checked_add(b){
        Some(result) => println!("Checked Add: {} + {} = {}", a, b, result),
        None => println!("Checked Add: Overflow occurred!"),
    }

    // 3. OVERFLOWING METHOD
    let (result, overflowed) = a.overflowing_add(b);
    println!("Overflowing Add: {} + {} = {}, Overflow occurred: {}",
    a, b, result, overflowed);

    // 4. SATURATING METHODS
    let result = a.saturating_add(b);
    println!(
        "Saturating Add: {} + {} = {} (saturates at max value)",
        a, b, result
    );
}