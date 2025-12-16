
struct Student
{
    name: String,
    age: u8,
}

impl Student 
{
    fn greet(&self) 
    {
        println!("Hi! I'm {}", self.name);
    }
}

trait Speak
{
    fn speak(&self);
}
//this trait doesn't do anything yet!

//but once we implement it to our struct:
impl Speak for Student
{
    fn speak(&self)
    {
        println!("Hello from speak, I'm {}", self.name);
    }
}

fn main()
{
    let student = Student{
        name: "Semire".to_string(),
        age: 16,
    };

    student.greet();
    student.speak();
}