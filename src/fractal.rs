pub enum Mode {
    Mandelbrot,
    Julia { c_re: f64, c_im: f64 },
}

pub fn escape(mode: &Mode, x: f64, y: f64, max_iter: u32) -> f64 {
    let (mut zr, mut zi, cr, ci) = match mode {
        Mode::Mandelbrot => (0.0, 0.0, x, y),
        Mode::Julia { c_re, c_im } => (x, y, *c_re, *c_im),
    };

    let mut iter = 0u32;
    while zr * zr + zi * zi <= 4.0 && iter < max_iter {
        let new_zr = zr * zr - zi * zi + cr;
        let new_zi = 2.0 * zr * zi + ci;
        zr = new_zr;
        zi = new_zi;
        iter += 1;
    }

    if iter >= max_iter {
        return max_iter as f64;
    }

    let log_zn = (zr * zr + zi * zi).ln() / 2.0;
    let nu = (log_zn / std::f64::consts::LN_2).ln() / std::f64::consts::LN_2;
    (iter as f64 + 1.0 - nu).max(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn origin_never_escapes_mandelbrot() {
        let result = escape(&Mode::Mandelbrot, 0.0, 0.0, 200);
        assert_eq!(result, 200.0);
    }

    #[test]
    fn far_point_escapes_immediately() {
        let result = escape(&Mode::Mandelbrot, 10.0, 10.0, 200);
        assert!(result < 3.0, "expected near-instant escape, got {}", result);
    }

    #[test]
    fn known_boundary_point_escapes_partway() {
        let result = escape(&Mode::Mandelbrot, -0.75, 0.1, 200);
        assert!(result > 0.0 && result < 200.0, "got {}", result);
    }

    #[test]
    fn julia_mode_uses_fixed_c_not_pixel_as_c() {
        let a = escape(
            &Mode::Julia {
                c_re: -0.7,
                c_im: 0.27015,
            },
            0.1,
            0.1,
            200,
        );
        let b = escape(
            &Mode::Julia {
                c_re: 1.5,
                c_im: 0.0,
            },
            0.1,
            0.1,
            200,
        );
        assert_ne!(
            a, b,
            "different julia constants at the same point should differ"
        );
    }

    #[test]
    fn escape_never_returns_negative() {
        for i in 0..50 {
            let x = -2.0 + i as f64 * 0.08;
            let result = escape(&Mode::Mandelbrot, x, 0.0, 100);
            assert!(
                result >= 0.0,
                "negative escape value at x={}: {}",
                x,
                result
            );
        }
    }
}
