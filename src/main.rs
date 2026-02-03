// ARRAYS

fn main () {

    let mut arr;
    arr = [1,2,3,4,5];

    println!("arr = {}", arr[0]);
    println!("arr = {:?}", arr);

    arr[2] = 30;
    println!("arr = {:?}", arr);


    println!("Array length is: {}", arr.len());
}
 
 
 
 
 
 
 
 
 
 /* // References 

fn main() {

    let mut x = 5;
    x = x + 1;
    let y = &mut x;
    *y = *y + 1;
    println!("{}", y);


} */






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