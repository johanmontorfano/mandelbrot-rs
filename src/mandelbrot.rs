use num::Complex;

#[derive(Copy, Clone)]
pub struct Mandelbrot {
    pub width: f64,
    pub height: f64,
    pub offset_x: f64,
    pub offset_y: f64,
    pub scale: f64,
    pub max_iter: u16,
    pub precision: f64
}

impl Mandelbrot {
    pub fn init_with_offset_and_scale_for_coords(
        size: (u32, u32),
        scale: f64,
        precision: f64,
        max_iter: u16
    ) -> Self {
        let width = size.0 as f64;
        let height = size.1 as f64;

        Self {
            width, height, scale, precision, max_iter,
            offset_x: width * 0.75,
            offset_y: height / 2.0
        }
    }

    // Computes a point color using Mandelbrot proper computations and the final result at allowed
    // MAX_ITER.
    #[inline(always)]
    pub fn get_point_color_at_coords(&self, x: u32, y: u32) -> [u8; 3] {
        let x = (f64::from(x) - self.offset_x) / self.scale;
        let y = (f64::from(y) - self.offset_y) / self.scale;

        let c = Complex::new(x / self.width, y / self.height);
        let mut z = Complex::new(0.0, 0.0);
        let mut n = 0_u16;
        while z.norm_sqr() < self.precision && n < self.max_iter {
            z = z * z + c;
            n += 1;
        }
        if n == self.max_iter { [0, 0, 0] }
        else {
            let n = n as f64;
            let r = (n / 25.0 * 255.0) as u8;
            let g = (n / 30.0 * 255.0) as u8;
            let b = (n / 35.0 * 255.0) as u8;
            [r, g, b]
        }
    }
}