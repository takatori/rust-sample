extern crate num;
extern crate image;

use num::Complex;
use image::ColorType;
use image::png::PNGEncoder;
use std::str::FromStr;
use std::fs::File;

#[allow(dead_code)]
fn complex_square_add_loop(c: Complex<f64>) {
    let mut z = Complex { re: 0.0, im: 0.0 };
    loop {
        z = z * z + c;
    }
}

/// Try to determin if `c` is in the Mandelbrot set, using at most `limit`
/// iterations to decide.
///
/// If `c` is not a member, return `Some(i)`, where `i` is the number of
/// iterations it took for `c` to leave the circle of radius two centered on the
/// origin. If `c` seems to be a member (more precisely, If we reached the
/// iteration limit without being able to prove that `c` is not a member),
/// return `None`
fn escapes(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }
    return None;
}


/// Parse the string `s` as a coordinate pair, like `"400x600"` or `"1.0, 0.5"`.
///
/// Specifically, `s` should have the form <left><sep><right>, where <sep> is
/// the character given by the `separator` argument, and <left> and <right> are both
/// strings that can be parsed by `T::from_str`.
///
/// If `s` has the proper form, return `Some<(x, y)>`. If it doesn't parse
/// correctly, return `None`.
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None,
            }
        }
    }
}


/// Return the point on the complex plane corresponding to a given pixel in the bitmap.
///
/// `bounds` is a pair giving the width and height of the bitmap.
/// `pixel` is a pair indicating a particular pixel in that bitmap.
/// The `upper_left` and `lower_right` parameters are pints on the complex plane designating the
/// area our bitmap covers.
fn pixel_to_point(bounds: (usize, usize),
                  pixel: (usize, usize),
                  upper_left: (f64, f64),
                  lower_right: (f64, f64))
                  -> (f64, f64) {
    // It might be nicer to find the position of the *middle* of the pixel,
    // instead of its upper left corner, but this is easier to write tests for.
    let (width, height) = (lower_right.0 - upper_left.0, upper_left.1 - lower_right.1);

    (upper_left.0 + pixel.0 as f64 * width / bounds.0 as f64,
     upper_left.1 - pixel.1 as f64 * height / bounds.1 as f64)
}

/// Render a rectangle of the Mandelbrot set into a buffer of pixels.
/// The `bounds` argument gives the width and height of the buffer `pixels`,
/// which holds one grayscale piexel per byte. The `upper_left` and `lower_right`
/// arguments specify points on the complex
fn render(pixels: &mut [u8],
          bounds: (usize, usize),
          upper_left: (f64, f64),
          lower_right: (f64, f64)) {

    assert!(pixel.len() == bounds.0 * bounds.1);

    for r in 0..bounds.1 {
        for c in 0..bounds.0 {

            let point = pixel_to_point(bounds, (c, r), upper_left, lower_right);

            pixels[r * bounds.0 + c] = match escapes(Complex { re: point.0, im: point.1 }, 255) {
                None => 0,
                Some(count) => 255 - count as u8,
            };
        }
    }
}

/// Write the buffer `pixels`, whose dimensions are given by `bounds`, to the
/// file name `filename`.
fn write_bitmap(filename: &str,
                pixels: &[u8],
                bounds: (usize, usize))
                -> Result<(), std::io::Error> {

    let output = File::create(filename)?;
    let encoder = PNGEncoder::new(output);
    encoder.encode(&pixels,
                bounds.0 as u32,
                bounds.1 as u32,
                ColorType::Gray(8))?;
    Ok(())
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(pixel_to_point((100, 100), (25, 75), (-1.0, 1.0), (1.0, -1.0)),
               (-0.5, -0.5));
}
