/*  
// Borrowing reference rules with read and write operation
fn main() {
    
    let mut s1:String = String::from("Hello");

    let r1 = &s1;
    println!("r1 = {}", r1);

    let w1 = &mut s1;
    w1.push_str(" World");
    println!("w1 = {}", w1);
}
*/