mod print; //import module called print (print.rs) 
mod var;
mod types;
mod string;
mod tuples;
mod arrays;
mod vectors;
mod conditionnals;
mod functions;
mod loops;
mod pointer_ref;

fn main() {
    println!("\n---output from print.rs---\n");
    print::run(); //calling function "run" from module "print" using (::)
    println!("\n---output from var.rs---\n");
    var::run();
    println!("\n---output from types.rs---\n");
    types::run();
    println!("\n---output from string.rs---\n");
    string::run();
    println!("\n---output from tuples.rs---\n");
    tuples::run();
    println!("\n---output from arrays.rs---\n");
    arrays::run();
    println!("\n---output from vectors.rs---\n");
    vectors::run();
    println!("\n---output from conditionals.rs---\n");
    conditionnals::run();
    println!("\n---output from loops.rs---\n");
    loops::run();
    println!("\n---output from functions.rs---\n");
    functions::run();
    println!("\n---output from pointer_ref.rs---\n");
    pointer_ref::run();
}
