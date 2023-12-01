


// ! Rust Types
// @Integer
let mut a: u32 = 8;
let b: u64 = 877;
let c: i64 = 8999;
let d = -90;

// @Float
let mut sixty_bit_float: f64 = 89.90;
let thirty_two_bit_float: f32 = 7.90;
let just_a_float = 69.69;

// @Boolean
let true_val: bool = true;
let false_val: bool = false;
let just_a_bool = true;
let is_true = 8 < 5;  // => false

// @Character
let first_letter_of_alphabet = 'a';
let explicit_char: char = 'F';
let implicit_char = '8';
let emoji = "\u{1f600}";   // => 😀

// @String Literal
fn string_literal (){
   let community_name = "AXIAL";
   let no_of_members: &str = "ten"; 
   println!("The name of the community is {community_name} and it has {no_of_members} members");
   
}

// @Arrays

fn array_literal (){
   let array: [i64; 6] = [92,97,98,99,98,94];
   println!("{:?}", array);
}


// @Multi-Dimensional Array

fn multi_array_literal (){

   let array: [[i64; 6] ;2] = [
      [1,2,3,4,5,6],
      [6,5,4,3,2,1]];
      
   println!("{:?}",array)
}

// @Mutable Array
fn mut_array () {
   let mut array: [i32 ; 3] = [2,6,10];
   array[1] = 4;
   array[2] = 6;

   println!("{:?}", array);
};

// @Slices

fn string_literal(){
   let mut array: [ i64; 4] = [1,2,3,4];
let mut slices: &[i64] = &array[0..3]

println!("The elements of the slices are : {slices:?}");

}

// @Vector
fn vector_literal(){
   let some_vector = vec![1,2,3,4,5]; 
   println!("{:?}",some_vector);
}
// @tuple
fn vector_literal(){
   let tuple = (1, 'A' , "Cool", 78, true);

   println!("{:?}",tuple);
}



fn main() {
   println!("Hellow")
}