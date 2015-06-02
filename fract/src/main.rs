//!An example of generating julia fractals.

extern crate num;
extern crate image;

use std::fs::File;
use std::path::Path;

use num::abs;
use num::complex::Complex;

fn julia(xres: u32, yres: u32, max_iterations: u16) {

    let scalex = 4.0 / xres as f32;
    let scaley = 4.0 / yres as f32;

    let mut imgbuf = image::ImageBuffer::new(xres, yres);

    // Iterate over the coordiantes and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let cy = y as f32 * scaley - 2.0;
        let cx = x as f32 * scalex - 2.0;
        
        let mut z = Complex::new(cx, cy);
        let c = Complex::new(-0.4, 0.6);

        let mut i = 0;

        for t in (0..max_iterations) {
            if z.norm() > 2.0 {
                break
            }
            z = z * z + c;
            i = t;
        }

        // Create an 8bit pixel of type Luma and value i
        // and assign in to the pixel at position (x, y)
        *pixel = image::Luma([i as u8]);

    }

    // Save the image as “fractal.png”
    let ref mut fout = File::create(&Path::new("julia.png")).unwrap();

    // We must indicate the image’s color type and what format to save as
    let _ = image::ImageLuma8(imgbuf).save(fout, image::PNG);
}
fn compute_r(n: usize, x: f32, y: f32, fract_type: &[u8; 5]) -> f32
{
    let sn = n % fract_type.len();
    let choice = fract_type[sn];
    if choice == 0 { x } else { y }
}

fn lyapunov(xres: u32, yres: u32, max_iterations: u16, fract_type: &[u8; 5]) {

    let mut imgbuf = image::ImageBuffer::new(xres, yres);

    // Iterate over the coordiantes and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let a = 2.0 + (x as f32 / xres as f32) * 2.0;
        let b = 2.0 + (y as f32 / yres as f32) * 2.0;
        
        let mut iter = vec![0.0; max_iterations as usize];
        iter[0] = 0.5;

        for i in (1..max_iterations) {
            let i = i as usize;
            iter[i] = compute_r(i - 1, a, b, fract_type) * iter[i - 1] * (1.0 - iter[i - 1]);
        }

        let mut ri = vec![0.0; max_iterations as usize];

        for i in (1..max_iterations) {
            let i = i as usize;
            ri[i] = num::abs(compute_r(i, a, b, fract_type));
        }

        let mut one_minus_2_iterations = vec![0.0; max_iterations as usize];
        for i in (1..max_iterations) {
            let i = i as usize;
            one_minus_2_iterations[i] = num::abs((1.0 - (2.0 * iter[i])));
        }

        let mut lyapunov_exp = 0.0;
        for i in (1..max_iterations) {
            let i = i as usize;
            lyapunov_exp += (ri[i] * one_minus_2_iterations[i]).ln();
        }

        lyapunov_exp = lyapunov_exp / max_iterations as f32;
        if lyapunov_exp < 0.0 {
            *pixel = image::Rgb([(255.0 * num::abs(lyapunov_exp)) as u8,
                                 (255.0 * num::abs(lyapunov_exp)) as u8,
                                 (0.0) as u8]);
        } else if lyapunov_exp == 0.0 {
             *pixel = image::Rgb([(255) as u8,
                                 (255) as u8,
                                 (0) as u8]);
        } else {
             *pixel = image::Rgb([(0) as u8,
                                 (lyapunov_exp * 0.420) as u8,
                                 (lyapunov_exp * 255.0) as u8]);
        }
    }

    // Save the image as “fractal.png”
    let ref mut fout = File::create(&Path::new("lyapunov.png")).unwrap();

    // We must indicate the image’s color type and what format to save as
    let _ = image::ImageRgb8(imgbuf).save(fout, image::PNG);
}

fn main() {
    let max_iterations = 128u16;

    let imgx = 500;
    let imgy = 500;
    println!("Starting Julia");
    julia(imgx, imgy, max_iterations);
    println!("Starting lyapunov");
    lyapunov(imgx, imgy, max_iterations, &[0u8, 1u8, 0u8, 1u8, 1u8]); 
}