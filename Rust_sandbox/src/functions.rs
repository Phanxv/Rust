//Function stored in a box using fn keyword
pub fn run(){
    greeting("Konnichiha", "Yousei");
    println!("Sum : {}",add(69,420));
    //Closure local function
    let num3:i32 = 12;
    let add_num = |num1:i32,num2:i32|num1+num2+num3;
    println!("Closure sum : {}",add_num(69,69));
}
//fn NAME_OF_FUNCTION(PARAMETER_NAME: type){}
fn greeting(greet: &str, name: &str){
    println!("{} {} , Nice to meet you", greet, name);
}
//fn NAME_OF_FUNCTION(PARAMETER_NAME: type)->return_type{}
fn add(n1:i32,n2:i32) -> i32{
    n1+n2 //No semicolon when return//
}