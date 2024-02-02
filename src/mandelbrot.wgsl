struct Complex { real: f32, imaginary: f32 }

const MAX_ITER: u32 = 250;
const PRECISION: f32 = 16.0;

@group(0) @binding(0) var<uniform> size: array<u32>;

var iter_n: u32 = 0;
var c = Complex(0.0, 0.0);
var z = Complex(0.0, 0.0);

fn clexabs(a: Complex) -> f32 { // Complex Abs
    return sqrt(a.real * a.real + a.imaginary * a.imaginary);
}

fn clexmul(a: Complex, b: Complex) -> Complex { // Complex Multiply
    return Complex(
        a.real * b.real - a.imaginary * b.imaginary,
        a.real * b.imaginary + a.imaginary * b.real
    );
}

fn clexsq(a: Complex) -> Complex { // Complex Square
    return clexmul(a, a);
}

fn clexadd(a: Complex, b: Complex) -> Complex { // Complex Add
    return Complex(a.real + b.real, a.imaginary + b.imaginary);
}

@compute @workgroup_size(1) fn mandelbrot() -> u32 {
    c.real = coords[0];
    c.imaginary = coords[1];

    while(clexabs(z) <= PRECISION && iter_n < MAX_ITER) {
        z = clexadd(clexsq(z), c);
        iter_n += 1;
    }

    return iter_n;
}

@compute @workgroup_size(1) fn generate_for_2d_range() -> @location(0) array<u8> {
    var output: array<u8> = array();


    for(var x = 0; x <= size[0]; x++) {
        for(var y = 0; y <= size[1]; y++) {

        }
    }
}
