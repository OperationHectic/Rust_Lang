//Creates a struct with a lifetime definition using the 'a 
pub struct Person<'a>
{
    name: &'a str,
    age: u8
}

//Creates an implementation for the struct using the lifetime 'a
impl<'a> Person<'a>
{
    fn greeting(&self)
    {
        println!("Hey my name is {}, and I am {} years old", self.name, self.age);
    }
}

//Wrap a struct in a module to keep its members private
mod graphics
{
    pub struct Point
    {
        //x and y are private members
        x: u32,
        y: u32
    }

    impl Point
    {
        pub fn new(x: u32, y: u32) -> Point
        {
            Point {x: x, y: y}
        }

        pub fn get_x(&self) -> u32 { self.x }

        pub fn get_y(&self) -> u32 { self.y }
    }
}

fn main()
{
    use graphics::*;

    let point: Point = Point::new(32, 45);
    let person: Person = Person { name: "Jim", age: 34 };
    println!("X is {} and Y is {}", point.get_x(), point.get_y());
    person.greeting();
}