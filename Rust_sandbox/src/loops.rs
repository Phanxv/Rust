pub fn run(){
    /* infinite loop */
    let mut count = 1;
    /* loop{
        count+=1;
        print!("{} , ",count);
        if count == 50 {break;}
    } */
    /* While loop */
    while count <= 50{
        if count < 10 {
            print!(" {} ", count);
        }else{
            print!("{} ",count);
        }
        
        if count%10==0 {
            println!("");
        }
        count+=1;
    }
    println!(" ");
    //For Range
    for x in 1..11 {
        for y in 1..11 {
            print!("{} x {} = {}\t",y,x,x*y);
        }
        print!("\n");
    }

}