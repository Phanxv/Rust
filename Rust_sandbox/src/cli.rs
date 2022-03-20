pub fn run(){
    let args: Vec<String> = std::env::args().collect();
    let command = args[1].clone();
    println!("Command : {}",command);
    if command == "hello" {
        println!("Hello user");
    }
    else if command == "hi" {
        println!("Hi user");
    }
    else {
        println!("Command not found");
    }
}