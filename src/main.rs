// References 

fn main() {

    let mut x = 5;
    x = x + 1;
    let y = &mut x;
    *y = *y + 1;
    println!("{}", y);


}






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