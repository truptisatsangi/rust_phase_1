mod overflow_handlers;
mod branches;
mod fibonacci;

fn main() {
    // overflow_handlers:: overflow_handlers();
    // println!("-------------------------------------------------------------");
    // branches:: branches_demo();
    // branches:: loop_demo();
    // branches:: naming_the_loop()
   let res = fibonacci:: generate_fibonacci(5);
    println!("{res}")
}
