// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run(){
    let hello1 = "Hello1"; //Immutable fixed length
    let mut hello2 = String::from("Hello "); //Growable, heap-allocated data structure

    println!("{} is Immutable fixed length\n{} is Growable, heap-allocated data structure",hello1,hello2);

    //get length of string 
    println!("The length of hello is {}", hello1.len());

    //push and push_str
    hello2.push('W');
    hello2.push_str("orld");
    println!("{}",hello2);

    //capacity in byte
    println!("Capacity of hello2 : {} bytes",hello2.capacity());
    
    //check if the string is empty or not
    println!("Is hello2 empty : {}",hello2.is_empty());

    //check if contain substring
    println!("Does hello2 contains 'World' : {} ",hello2.contains("World"));

    //replace substring
    println!("Replace 'World' with 'Sekai : {}", hello2.replace("World", "Sekai"));

    //create string with capacity
    let mut s = String::with_capacity(12);
    s.push_str("Hello");
    s.push_str(" World");
    //assertion testing
    assert_eq!(11, s.len());
    assert_eq!(12, s.capacity());
    println!("{}",s);

}