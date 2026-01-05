trait Speak
{
    fn speak(&self);
}
//“Anything that implements Speak must have a speak function.”

struct Dog;
struct Cat;

impl Speak for Cat
{
    fn speak(&self)
    {
        println!("Neko-Neko!");
    }
}

impl Speak for Dog
{
    fn speak(&self)
    {
        println("Ko-inu!");
    }
}

fn make_it_speak(animal: &impl Speak)
{
    animal.speak();
}

fn make_it_speak<T: Speak>(animal: &T)
{
    animal.speak();
}