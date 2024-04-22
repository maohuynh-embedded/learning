fn main() {
    let x = 5;
    let y : i32 = 6;
    let c : char = 'c'; 
    println!("The value of x is: {}", x);
    println!("The value of y is: {} {}", y, std::any::type_name::<i32>());
    println!("The value of c is: {} {}", c, std::any::type_name::<char>());
}
