fn main() {

    // OWNERSHIP
    let outside_variable: u8 = 5;

    {
        let inside_variable: u8 = 15;
        println!("The inside variable: {}", inside_variable);
    }
    println!("The inside variable: {}", inside_variable);
    println!("The outside variable: {}", outside_variable);
}



