mod single_module;
mod multi_module;

fn main()
{
    //Code to run the single module file single_module.rs
    single_module::display_msg("Alert Message", "This is a custom alert message");

    //Code to run the multi_module folder functions in math.rs and swap.rs
    let dist = multi_module::math::dist(3.4, 4.5, 5.6, 8.7);
    println!("Distance is {}", dist);
    let mut x: u32 = 12;
    let mut y: u32 = 46;
    println!("x is {} and y is {}", x, y);
    multi_module::swap::swap(&mut x, &mut y);
    println!("x is {} and y is {}", x, y);
}