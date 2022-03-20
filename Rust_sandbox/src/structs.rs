/*used to create custom datatype  */

/* Traditional struct */
struct Color
{
    red: u8,
    green: u8,
    blue: u8,
}
/* Tuple struct */
struct Colors(u8,u8,u8);

/* Struct with function associated with it */
/* Using impl (implement keyword) */
struct Person
{
    first_name: String,
    last_name: String
}

impl Person
{
    /* Construct a person */
    fn new(first: &str, last: &str) -> Person
    {
        Person{first_name: first.to_string(), last_name: last.to_string()}
    }
    /* return full name */
    fn full_name(&self) -> String
    {
        format!("{} {}", self.first_name,self.last_name)
    }
    fn set_last_name(&mut self,last: &str)
    {
        self.last_name = last.to_string();
    }
}

pub fn run(){
    let mut c1 = Color{red:255,green:0,blue:0};
    let mut c2 = Colors(255,0,0);
    c1.red = 250;
    c2.1 = 69;
    println!("Color c1 : R:{} G:{} B:{}",c1.red,c1.green,c1.blue);
    println!("Color c2 : R:{} G:{} B:{}",c2.0,c2.1,c2.2);
    let mut p1 = Person::new("Rimuru","");
    //p1.last_name = "Tempest".to_string();
    p1.set_last_name("Tempest");
    println!("Person p1 \n\tFirst name : {}\n\tLast name : {}",p1.first_name,p1.last_name);
    println!("Person p1 full name : {}",p1.full_name());
}
