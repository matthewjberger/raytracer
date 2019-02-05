use std::fs::File;
use std::io::Write;

fn main() {
    let filename = "output.ppm";
    let mut output = File::create(filename).unwrap();

    let width = 200;
    let height = 100;
    write!(output, "P3\n{} {}\n255\n", width, height).unwrap();

    for y in (0..height).rev() {
        for x in 0..width {
            let red = (x as f64 / width as f64) * 255.99;
            let green = (y as f64 / height as f64) * 255.99;
            let blue = 0.2 * 255.99;
            write!(output, "{} {} {}\n", red as i32, green as i32, blue as i32).unwrap();
        }
    }
}
