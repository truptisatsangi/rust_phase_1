pub fn generate_fibonacci(mut num: u8) -> u32  {

    if num <= 2 {
        return num.into();
    }
    let mut temp = num;
    let result = loop {
        temp = temp * (num - 1); 
        num -= 1;
        if num == 0 || num == 1 {
            break temp;
        }
    };
    return result.into();
}