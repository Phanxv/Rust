mod print; //import module called print (print.rs) 
mod var;
mod types;

fn main() {
    println!("---output from print.rs---");
    print::run(); //calling function "run" from module "print" using (::)
    println!("---output from var.rs---");
    var::run();
    println!("---output from types.rs---");
    types::run();
}
