//Arrays - fixed list where element are the same datatype

pub fn run(){
    let mut numbers : [i32;5] = [1,2,3,4,5];
    //print data in array
    print!("Array numbers : ");
    println!("{:?}", numbers);
    print!("Data at index 0 of array is ");
    println!("{}", numbers[0]);
    //mutate data in array
    println!("Change element index 0 of array to number 6");
    numbers[0] = 6;
    println!("{:?}", numbers);
    //get array length
    println!("The length of the array : {} ", numbers.len());
    //get the size of array 
    //arrays are stack allocated
    println!("Arrays occupied {} bytes on memory", std::mem::size_of_val(&numbers));
    //get slice of array
    let slice: &[i32] = &numbers[0..3];
    print!("Array slice : ");
    println!("{:?}", slice);    
    
}