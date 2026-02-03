fn main() {

    // OWNERSHIP
   let x:String = String::from("Hello");
   procss_string(x);
//    println!("The value of x in main() is: {}", x);
}

fn procss_string(item:String) {
    println!("The value of x in process function is: {}", item);
}



