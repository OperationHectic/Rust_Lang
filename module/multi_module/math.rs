//Calculates the distance for a 32 bit float value
pub fn dist(x0: f32, y0: f32, x1: f32, y1: f32) -> f32
{
    let sx = x1 - x0;
    let sy = y1 - y0;
    (sx*sx + sy*sy).sqrt()
}