// @ Arithmetic and Type Casting
fn main() {
    println!("----------------------");
    // let x: u8 = 255; // 0 - 255
    // let y: u8 = 10; // 0 - 255

    // let z = x / y;

    let x = 50;
    let y = 50;
    let add = x + y;
    let sub = x - y;
    let multi = x * y;
    let div = x / y;

    let arr = [1, 2, 3, 4, 5];
    struct Person {
        name: String,
        age: String,
    };

    let person_1 = Person { name: (), age: () };

    println!("{}", add);
    println!("{}", sub);
    println!("{}", multi);
    println!("{}", div);
    println!("{:?}", arr);
}
