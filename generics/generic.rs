use std::fmt::Display;
use std::ops::{Shl, Mul};

//Create a trait that will allow calling of a function that return 1
pub trait One
{
    fn one() -> Self;
}

//Implement the one function
impl One for u32
{
    fn one() -> Self { 1 }
}

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

//Returns a value shifted to the left by one 
//which is effectively multiple by 2
pub fn double_val<T: Shl<Output = T> + One>(x: T) -> T
{
    x << One::one()
}

//Returns the a value squared
pub fn square<T: Mul<Output = T> + Copy>(x: T) -> T
{
    x * x
}

//Returns the a value cubed
pub fn cube<T: Mul<Output = T> + Copy>(x: T) -> T
{
    x * x * x
}