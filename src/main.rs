const GLOBAL_VARIABLE: u8 = 100;
fn main() {

    // OWNERSHIP
    let outside_variable: u8 = 5;

    {
        let inside_variable: u8 = 15;
        println!("The inside variable: {}", inside_variable);
        println!("Outside variable: {}", outside_variable);
    }
    println!("The outside variable: {}", outside_variable);
    print_value();
}
fn print_value() {
    println!("Global valriable: {}", GLOBAL_VARIABLE);
}


