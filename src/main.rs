fn main() {
    println!("Function Here");
    variable()
}


fn variable(){
    let num1 = 10;
    let num2 = 15;

    let str1 = "string";
    let str2 = "here";

    let char1 = 'R';
    let char2 = 'S';

    let bool1 = true;
    let bool2 = false;

    let mut my_name = "Sameer";

    let num_arr1 = [1,2,3,4];
    let num_arr2 = [5,6,7,8];

    let str_arr1 = ["I'm","array","one","here"];
    let str_arr2 = ["I'm","array","two","here"];

    let tuple_1 = (num1,str1,bool1,my_name,num_arr1,str1);

    let add_num = num1 + num2;
    let add_str = str1.to_owned() + str2;


    println!("num1: {}", num1);
    println!("num2: {}", num2);
    println!("str1: {}", str1);
    println!("str2: {}", str2);
    println!("char1: {}", char1);
    println!("char2: {}", char2);
    println!("bool1: {}", bool1);
    println!("bool2: {}", bool2);
    println!("my_name: {}", my_name);
    println!("num_arr1: {:?}", num_arr1);
    println!("num_arr2: {:?}", num_arr2);
    println!("str_arr1: {:?}", str_arr1);
    println!("str_arr2: {:?}", str_arr2);
    println!("tuple_1: {:?}", tuple_1);
    println!("add_num: {}", add_num);
    println!("add_str: {}", add_str);


}
