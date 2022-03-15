//Conditionals logic
pub fn run(){
    let age:u8 = 21;
    let check_id:bool = false;
    /* if else*/
    if age >= 21 && check_id{
        println!("You may enter");
    }else if age >= 21 && !check_id{
        println!("Let me see your ID");
    }else{
        println!("You can't enter");
    }
    /* shorthand */
    let is_of_age: bool = if age >= 21 {true} else {false};
    if is_of_age {
        println!("The person is of age");
    }else {
        println!("The person is not of age");
    }
}