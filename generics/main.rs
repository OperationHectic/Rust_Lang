mod generic;
use generic::*;

fn main()
{
    //Initializing a variable using the Generic struct with an explicit type of u32
    let gen: Generic<u32> = Generic { data: 34};

    //Print the generic type data
    gen.print_data();

    //println!("Doubled value is {}", double_val(3))
}