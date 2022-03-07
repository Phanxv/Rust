mod print; //import module called print (print.rs) 
mod var;
mod types;
mod string;
mod tuples;

fn main() {
    println!("");
    println!("---output from print.rs---");
    println!("");
    print::run(); //calling function "run" from module "print" using (::)
    println!("");
    println!("---output from var.rs---");
    println!("");
    var::run();
    println!("");
    println!("---output from types.rs---");
    println!("");
    types::run();
    println!("");
    println!("---output from string.rs---");
    println!("");
    string::run();
    println!("");
    println!("---output from tuples.rs---");
    println!("");
    tuples::run();
    println!("");
}
