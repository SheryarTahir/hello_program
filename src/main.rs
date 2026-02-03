fn main() {

    let num1: u8 = 5;
    let num2: u8 = 10;
    let result: u8 = add(num1, num2);
    println!("The sum of num1 and num2 is: {}", result);
}

fn add(num1: u8, num2: u8) -> u8{
    return num1 + num2;
}

