pub fn branches_demo() {
    let condition = true;
    let number = if condition { 3 } else { 9 };
    if number < 5 {
        println!("Lesser");
    } else {
        println!("Greater")
    }
}
// NOTE: The error indicates that Rust expected a bool but got an integer. Unlike languages such as Ruby and JavaScript, 
// Rust will not automatically try to convert non-Boolean types to a Boolean. You must be explicit and always provide 
// if with a Boolean as its condition. 

pub fn loop_demo() {
    loop {
        println!("hey!");
        break;
    }

    let mut counter = 0 ;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result {}", result);
}

pub fn naming_the_loop() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop{
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}


