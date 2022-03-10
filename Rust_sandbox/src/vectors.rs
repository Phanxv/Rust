//Vectors are reizable arrays

pub fn run(){
    let mut numbers : Vec<i32> = vec![1,2,3,4,5];
    //print data in array
    print!("Vector 'numbers' : ");
    println!("{:?}", numbers);
    print!("Data at index 0 of vector is ");
    println!("{}", numbers[0]);
    //mutate data in array
    println!("Change element index 0 of vector to number 6");
    numbers[0] = 6;
    println!("{:?}", numbers);
    //get array length
    println!("The length of the vector : {} ", numbers.len());
    //get the size of array 
    //arrays are stack allocated
    println!("vector occupied {} bytes on memory", std::mem::size_of_val(&numbers));
    //get slice of array
    let slice: &[i32] = &numbers[0..3];
    print!("Vector 'slice' : ");
    println!("{:?}", slice);
    //appending vectors
    numbers.push(69);
    print!("Vector 'number' after append 69 : ");
    println!("{:?}",numbers);
    numbers.pop();
    print!("Vector 'number' after remove the last index : ");
    println!("{:?}",numbers);
    //loop through vector
    for x in numbers.iter(){
        println!("Number : {}",x);
    }
    for x in numbers.iter_mut(){
        *x *= 2;
    }
    println!("Vector 'number' after time two : {:?}",numbers);
}