use std::io;


fn main() {
    let x: i32 = 59;
    println!("x is : {:?}", x);

    input_output()
}


fn input_output(){
    let mut input = String::new();
    io::stdin().read_line(&mut input);
}