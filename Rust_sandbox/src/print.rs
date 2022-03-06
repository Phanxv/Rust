pub fn run(){
    //output to console
    println!("Hello from print.rs");
    //basic formatting using place holder ({})
    println!("Hello from print.rs using {}", "place holder");
    //positional argument {index}
    println!("Hello from {0} using {1}", "print.rs", "positional argument");
    //named argument {name}
    println!("Hello from {file_name} using {way}", file_name = "print.rs", way = "named argument");
    //placeholder trait {index or name : trait}
    println!("Number {number} in binary : {number:b} | hexadecimal : {number:x} | Octadecimal : {number:o}", number = 10);
    //placecholder for debug trait {:?}, (argument)
    println!("Placeholder for debuggin traits : {:?}", (69, 420.69, true, "Hello"));
    //basic math +-*/
    println!("Basic math : {num1} + {num2} = {result}", num1 = 69, num2 = 420, result = 69 + 420);
}