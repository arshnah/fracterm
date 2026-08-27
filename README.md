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
implemented here. `+` now clamps `scale` at that floor instead of letting
it shrink past it: below roughly `1e-13`, `center + pixel_offset * scale`
stops resolving to distinct `f64` values for a center around magnitude 1,
so every pixel was sampling the same point and the screen just went
uniformly black, silently, with no indication why. The status line now
says `(max zoom, f64 precision limit)` once you hit it.

A separate, more common way to end up staring at a solid black screen:
`(-0.5, 0)`, the default center, sits inside the Mandelbrot set's main
cardioid. That orbit never escapes, so zooming straight in from there
without panning toward the boundary just shows more interior, which is
black by definition at any zoom depth. Not a bug either, just point that
looks like one until you pan.

Per-pixel escape-time is parallelized across rows with `rayon`.

## License

See [LICENSE](LICENSE).
