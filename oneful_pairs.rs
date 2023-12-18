use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut itr = input.trim().split_whitespace();
    
    let a:i32 = itr.next().unwrap().parse().unwrap();
    let b:i32 = itr.next().unwrap().parse().unwrap();
    
    if a + b + (a * b) == 111 {
        println!("YES");
    } else {
        println!("NO");
    }
}
