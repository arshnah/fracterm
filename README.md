# fracterm

A Mandelbrot/Julia set explorer that renders in your terminal using
truecolor half-block characters, two vertical pixels per cell.

## Install

```
git clone https://github.com/arshnah/fracterm
cd fracterm
cargo build --release
./target/release/fracterm
```

## Usage

```
fracterm
```

Controls:

- arrow keys: pan
- `+`/`-`: zoom in/out
- `j`: toggle Mandelbrot / Julia (Julia uses the point you're centered on
  as its constant)
- `[`/`]`: decrease/increase max iterations
- `r`: reset view
- `q`/`esc`: quit

Escape-time is computed with plain `f64`, no arbitrary-precision arithmetic,
so useful zoom depth tops out around 10^13x before floating point error
dominates the calculation and the image degenerates into noise. That's a
real, known limit of the technique, not a bug, going deeper needs
perturbation theory against a high-precision reference orbit, which isn't
implemented here.

Per-pixel escape-time is parallelized across rows with `rayon`.

## License

See [LICENSE](LICENSE).
