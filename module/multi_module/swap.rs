//SImple swao function that takes mutable references to update variable values
pub fn swap(a: &mut u32, b: &mut u32)
{
    let temp = *a;
    *a = *b;
    *b = temp;
}