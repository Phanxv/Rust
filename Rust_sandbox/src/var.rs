//variables hold primitive data and reference of the data
//variable are immutable by default
//Rust is block-scoped language

//use let keyword to declare variables

pub fn run(){
    let name = "Yousei";
    let mut age = 18; //mut keyword to make ariable mutable
    println!("Hello my name is {} and I am {} years old", name, age);
    age+=1;
    println!("I will be {} next year",age);
    //define cinstant using const keyword and specify datatype of constant
    const STUDENT_ID: i32 = 2111310609;
    println!("My student ID is {}", STUDENT_ID);
    //define multiple variables using ()
    let (real_name, last_name, phone_number) = ("Phankawee", "Chulakasian", 634216162);
    println!("Name : {} {} Phone number : +66{}",real_name,last_name,phone_number);
}