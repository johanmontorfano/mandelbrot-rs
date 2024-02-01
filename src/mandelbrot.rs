use num::Complex;

const MAX_ITER: u16 = 25;
const PRECISION: f64 = 16.0;

#[derive(Copy, Clone)]
pub struct Mandelbrot {
    width: f64,
    height: f64,
    pub offset_x: f64,
    pub offset_y: f64,
    scale: f64
}

impl Mandelbrot {
    pub fn init_with_offset_and_scale_for_coords(size: (u32, u32), scale: f64) -> Self {
        let width = size.0 as f64;
        let height = size.1 as f64;

        Self {
            width, height, scale,
            offset_x: width * 0.75,
            offset_y: height / 2.0
        }
    }

    // Computes a point color using Mandelbrot proper computations and the final result at allowed
    // MAX_ITER.
    pub fn get_point_color_at_coords(&self, x: i32, y: i32) -> [u8; 3] {
        let x = (x as f64 - self.offset_x) / self.scale;
        let y = (y as f64 - self.offset_y) / self.scale;

        let c = Complex::new(x / self.width, y / self.height);
        let mut z = Complex::new(0.0, 0.0);
        let mut n = 0_u16;

        while z.norm() < PRECISION && n < MAX_ITER {
            z = z * z + c;
            n += 1;
        }

        if n == MAX_ITER { [0, 0, 0] }
        else {
            let n = n as f64;

            let r = (n / 25.0 * 255.0) as u8;
            let g = (n / 30.0 * 255.0) as u8;
            let b = (n / 35.0 * 255.0) as u8;

            [r, g, b]
        }
    }
}