use std::fmt::Display;
//use std::ops::Shl;

//Struct that uses the genric type T
pub struct Generic<T>
{
    pub data: T
}

//Implementation of the Generic struct print method which uses the std::fmt::Display trait to allow printing
impl<T: Display> Generic<T>
{
    pub fn print_data(&self)
    {
        println!("The data is {}", self.data);
    }
}

/*pub fn double_val<T: Shl<Output = T> + Integer>(x: T) -> T
{
    x << T::one()
}*/