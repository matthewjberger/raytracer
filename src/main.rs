use std::fs::File;
use std::io::Write;

mod vec3;

fn main() {
    let filename = "output.ppm";
    let mut output = File::create(filename).unwrap();

    let width = 200;
    let height = 100;
    writeln!(output, "P3\n{} {}\n255", width, height).unwrap();

    for y in (0..height).rev() {
        for x in 0..width {
            let red = (f64::from(x) / f64::from(width)) * 255.99;
            let green = (f64::from(y) / f64::from(height)) * 255.99;
            let blue = 0.2 * 255.99;
            writeln!(output, "{} {} {}", red as i32, green as i32, blue as i32).unwrap();
        }
    }
}
