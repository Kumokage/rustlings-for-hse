use core::f32;

fn main() {
    // TODO: Fix the Clippy lint in this line.
    let pi = f32::consts::PI;
    let radius: f32 = 5.0;

    let area = pi * radius.powi(2);

    println!("The area of a circle with radius {radius:.2} is {area:.5}");
}
