// Tuples group together values of different types
// Max 12 elements
pub fn run(){
    let person: (&str, &str, u8) = ("Phankawee","Chulakasian",18);
    println!("Name : {}\nSurname : {}\nAge : {}",person.0,person.1,person.2);
}