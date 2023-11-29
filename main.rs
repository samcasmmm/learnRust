

let mut a: u32 = 8;
let b: u64 = 877;
let c: i64 = 8999;
let d = -90;
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

// @array
let array: [i64; 6] = [92,97,98,99,98,94];


// @2D array
let array: [[i64; 6] ;2] = [
            [1,2,3,4,5,6],
            [6,5,4,3,2,1]];



fn main() {
   println!("Hellow")
}