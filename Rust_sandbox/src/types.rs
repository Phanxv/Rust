/*
Primitive Types--
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays
*/

pub fn run(){
    //interger default i32
    let x = 32;
    //float default f64
    let y = 64.0;
    //explicit type
    let z: i64 = 69420;
    //find max size 
    println!("max size for i32 : {}", std::i32::MAX);
    println!("max size for i64 : {}", std::i64::MAX);
    //println!("max size for f64 : {}", std::f64::MAX);
    //declare boolean
    let is_active: bool = true;
    //get boolean from expression
    let is_greater: bool = 10<5;
    //char (unicode)
    let alphabet: char = '僕';
    let face = '\u{1F600}';
    println!("{:?}",(x,y,z,is_active,is_greater,alphabet,face));
}