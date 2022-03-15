//reference pointer - point to the resource in memory
pub fn run(){
    //primative array
    let arr1: [i32;5] = [1,2,3,4,5];
    let arr2 = arr1;
    println!("data in arr1 & arr2 : {:?}",(arr1,arr2));
    /* With non-primitives, if you assign another variable to a piece of data, 
    the first variable will no longer hold that value. 
    You'll need to use a reference (&) to point to the resource*/
    let vec1: Vec<i32> = vec![1,2,3,4,5];
    let vec2 = &vec1;
    println!("data in vec1 & vec2 : {:?}",(&vec1,&vec2));

}