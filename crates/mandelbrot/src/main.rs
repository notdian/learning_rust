use std::{
    env,
    fs::File,
    ops::{Add, Mul, Sub},
    str::FromStr,
};

use image::{png::PNGEncoder, ColorType};
use num::Float;
#[derive(Copy, Clone, Debug, Default)]
struct Complex<T: Float = f32> {
    re: T,
    im: T,
}

impl<T: Float> Complex<T> {
    #[inline]
    fn norm_sqr(self) -> T {
        self.im.clone() * self.im + self.re * self.re.clone()
    }
}

impl<T: Float> Add for Complex<T> {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}
impl<T: Float> Sub for Complex<T> {
    type Output = Self;
    #[inline]

    fn sub(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl<T: Float> Mul for Complex<T> {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re.clone() * rhs.re.clone() - self.im.clone() * rhs.im.clone(),
            im: self.re * rhs.im + rhs.re * self.im,
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        eprintln!("Usage: {} file pixels upper_left lower_right", args[0]);
        eprint!(
            "Example: {} mandelbrot.png 1000x750 -1.20,0.35 -1,0.20",
            args[0]
        );
        std::process::exit(1);
    }

    let bounds = parse_tuple(&args[2], "x").expect("error parsing dimensions");
    let upper_left = parse_complex(&args[3]).expect("error parsing upper left corner");
    let lower_right = parse_complex(&args[4]).expect("error parsing lower right corner");

    let mut pixels = vec![0; bounds.0 * bounds.1];

    let threads = num_cpus::get_physical();
    let rows_per_band = bounds.1 / threads + 1; //height / threads + 1

    let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();

    crossbeam::scope(|spawner| {
        for (i, band) in bands.into_iter().enumerate() {
            let top = rows_per_band * i;
            let height = band.len() / bounds.0;
            let band_bounds = (bounds.0, height);
            let band_upper_left = pixel_to_point::<f32>(bounds, (0, top), upper_left, lower_right);
            let band_lower_right =
                pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);

            spawner.spawn(move |_| {
                render(band, band_bounds, band_upper_left, band_lower_right);
            });
        }
    })
    .unwrap();

    write_image(&args[1], &pixels, bounds).expect("error writing to file");
}

fn write_image(
    filename: &str,
    pixels: &[u8],
    bounds: (usize, usize),
) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;
    let encoder = PNGEncoder::new(output);

    encoder.encode(
        &pixels,
        bounds.0 as u32,
        bounds.1 as u32,
        ColorType::Gray(8),
    )?;

    Ok(())
}
fn parse_complex(s: &str) -> Option<Complex> {
    match parse_tuple(s, ",") {
        Some((re, im)) => Some(Complex { re, im }),
        _ => None,
    }
}

fn parse_tuple<T: FromStr>(input: &str, del: &str) -> Option<(T, T)> {
    let parts: Vec<&str> = input.split(del).collect();

    if parts.len() != 2 {
        return None;
    }

    let x = parts[0];
    let y = parts[1];

    match (x.parse::<T>(), y.parse::<T>()) {
        (Ok(x1), Ok(y1)) => Some((x1, y1)),
        _ => None,
    }
}

fn pixel_to_point<T: Float>(
    (bound_x, bound_y): (usize, usize),
    (pixel_x, pixel_y): (usize, usize),
    upper_left: Complex<T>,
    lower_right: Complex<T>,
) -> Complex<T> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );

    Complex {
        re: upper_left.re
            + num_traits::cast::<usize, T>(pixel_x).unwrap() * width
                / num_traits::cast::<usize, T>(bound_x).unwrap(),
        im: upper_left.im
            - num_traits::cast::<usize, T>(pixel_y).unwrap() * height
                / num_traits::cast::<usize, T>(bound_y).unwrap(),
    }
}
fn escape_time(c: Complex, limit: u8) -> Option<u8> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c
    }
    None
}

fn render(pixels: &mut [u8], bounds: (usize, usize), upper_left: Complex, lower_right: Complex) {
    assert!(pixels.len() == bounds.0 * bounds.1);
    for row in 0..bounds.1 {
        for col in 0..bounds.0 {
            let point = pixel_to_point::<f32>(bounds, (col, row), upper_left, lower_right);
            pixels[row * bounds.0 as usize + col] = match escape_time(point, 255) {
                Some(l) => 255 - l,
                None => 0,
            }
        }
    }
}
