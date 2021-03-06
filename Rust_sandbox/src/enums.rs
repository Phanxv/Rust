//Enums are types which have a few definitive values
enum Movement {
    //Variants
    Up,
    Down,
    Left,
    Right
}

fn move_avatar(m: Movement){
    match m {
        Movement::Up => println!("Avatar moving up"),
        Movement::Down => println!("Avatar moving down"),
        Movement::Left => println!("Avatar moving left"),
        Movement::Right => println!("Avatar moving right") 
    }
}
pub fn run(){
    let avatar1 = Movement::Down;
    let avatar2 = Movement::Left;
    move_avatar(avatar1);
    move_avatar(avatar2);
}