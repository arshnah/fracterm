use crate::fractal::{escape, Mode};
use rayon::prelude::*;

pub struct View {
    pub center_re: f64,
    pub center_im: f64,
    pub scale: f64,
    pub max_iter: u32,
    pub mode: Mode,
}

const ASPECT: f64 = 0.5;

pub fn palette(t: f64) -> (u8, u8, u8) {
    let phase = t * 0.12;
    let r = 0.5 + 0.5 * (phase).sin();
    let g = 0.5 + 0.5 * (phase + 2.0).sin();
    let b = 0.5 + 0.5 * (phase + 4.0).sin();
    ((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
}

pub fn pixel_color(view: &View, cols: usize, rows_px: usize, px: usize, py: usize) -> (u8, u8, u8) {
    let w = cols as f64;
    let h = rows_px as f64;
    let x = view.center_re + (px as f64 - w / 2.0) * view.scale * ASPECT;
    let y = view.center_im + (py as f64 - h / 2.0) * view.scale;

    let smoothed = escape(&view.mode, x, y, view.max_iter);
    if smoothed >= view.max_iter as f64 {
        return (0, 0, 0);
    }
    palette(smoothed)
}

pub fn render_frame(view: &View, cols: usize, rows: usize) -> String {
    let rows_px = rows * 2;

    let pixels: Vec<Vec<(u8, u8, u8)>> = (0..rows_px)
        .into_par_iter()
        .map(|py| {
            (0..cols)
                .map(|px| pixel_color(view, cols, rows_px, px, py))
                .collect()
        })
        .collect();

    let mut out = String::with_capacity(cols * rows * 20);
    for row in 0..rows {
        let upper = &pixels[row * 2];
        let lower = &pixels[row * 2 + 1];
        for col in 0..cols {
            let (ur, ug, ub) = upper[col];
            let (lr, lg, lb) = lower[col];
            out.push_str(&format!(
                "\x1b[38;2;{};{};{}m\x1b[48;2;{};{};{}m\u{2580}",
                ur, ug, ub, lr, lg, lb
            ));
        }
        out.push_str("\x1b[0m\r\n");
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_view() -> View {
        View {
            center_re: -0.5,
            center_im: 0.0,
            scale: 0.01,
            max_iter: 100,
            mode: Mode::Mandelbrot,
        }
    }

    #[test]
    fn points_inside_the_set_render_black() {
        let view = View {
            center_re: 0.0,
            center_im: 0.0,
            scale: 0.001,
            max_iter: 200,
            mode: Mode::Mandelbrot,
        };
        let color = pixel_color(&view, 10, 10, 5, 5);
        assert_eq!(color, (0, 0, 0));
    }

    #[test]
    fn palette_never_panics_across_full_cycle() {
        let mut t = 0.0;
        while t < 500.0 {
            let _ = palette(t);
            t += 1.3;
        }
    }

    #[test]
    fn render_frame_produces_one_line_per_row() {
        let view = test_view();
        let frame = render_frame(&view, 20, 5);
        let lines = frame.matches("\r\n").count();
        assert_eq!(lines, 5);
    }

    #[test]
    fn render_frame_is_deterministic_for_the_same_view() {
        let view = test_view();
        let a = render_frame(&view, 15, 4);
        let b = render_frame(&view, 15, 4);
        assert_eq!(a, b);
    }
}
