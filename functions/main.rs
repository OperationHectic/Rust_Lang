fn main()
{
     custom_function();
     
     println!("return value is {}\n", ret_function(5));

     let mut x : i32 = 21;
     modify_reference_function(&mut x);
     println!("x is {}\n", x);

     let r = modify_reference_ret_function(&mut x);
     println!("r is {}", r);
     println!("x is {}", x);
}

//Function without arguments that returns nothing
fn custom_function()
{
    println!("Call custom function.\n");
}

//Function that takes a variable by value and returns an integer multiplied by 2
fn ret_function(x: i32) -> i32
{
    x * 2
}

//Function that takes a mutable reference variable and modifies it 
fn modify_reference_function(x : &mut i32)
{
    *x *= 2;
}

//Function that takes a mutable reference variable and modifies it with a return type
fn modify_reference_ret_function(x : &mut i32) -> i32
{
    let r = *x * 2;
    r
}