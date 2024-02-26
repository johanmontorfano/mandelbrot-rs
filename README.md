# mandelbrot-rs

This implementation of the mandelbrot set is meant to train my skills on efficient code, concurrency and GUI with Rust.

To move into the set, use the following keys:
- `o` - Zoom Out
- `i` - Zoom In
- `l` - Left
- `r` - Right
- `t` - Top
- `b` - Bottom
- `&` - Lower resolution
- `é` - Higher resolution
- `(` - Lower precision
- `-` - Higher precision

### Fast Rendering
If the source code is compiled without optimizations, the resulting mandelbrot set viewer will not be efficient at all.
Hence, make sure that optimizations are enabled in the compiler (Enabled by default in the Cargo file.)

### Missing features/Improvements
- Auto zooming animation
- Better zoom handling
- Better color palette