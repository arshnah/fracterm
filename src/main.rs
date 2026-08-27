mod fractal;
mod render;

// Below this, `center + pixel_offset * scale` stops resolving to distinct
// f64 values near a center around magnitude 1 (f64 has ~2.22e-16 relative
// precision), so every pixel samples the same point and the screen goes
// uniformly black instead of showing anything. Zooming further than this
// needs arbitrary-precision or perturbation-based rendering, neither of
// which this renders with plain f64 math.
const MIN_SCALE: f64 = 1e-13;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, size, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::{cursor, execute};
use fractal::Mode;
use render::{render_frame, View};
use std::io::{stdout, Write};
use std::time::Duration;

fn default_view() -> View {
    View {
        center_re: -0.5,
        center_im: 0.0,
        scale: 0.005,
        max_iter: 200,
        mode: Mode::Mandelbrot,
    }
}

fn main() -> std::io::Result<()> {
    let mut stdout = stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, cursor::Hide)?;

    let mut view = default_view();
    let mut dirty = true;

    loop {
        let (cols, rows) = size()?;
        let cols = cols as usize;
        let rows = (rows as usize).saturating_sub(1).max(1);

        if dirty {
            let frame = render_frame(&view, cols, rows);
            execute!(stdout, cursor::MoveTo(0, 0))?;
            stdout.write_all(frame.as_bytes())?;
            let status = status_line(&view);
            stdout.write_all(status.as_bytes())?;
            stdout.flush()?;
            dirty = false;
        }

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Left => {
                        view.center_re -= view.scale * cols as f64 * 0.1;
                        dirty = true;
                    }
                    KeyCode::Right => {
                        view.center_re += view.scale * cols as f64 * 0.1;
                        dirty = true;
                    }
                    KeyCode::Up => {
                        view.center_im -= view.scale * rows as f64 * 0.1;
                        dirty = true;
                    }
                    KeyCode::Down => {
                        view.center_im += view.scale * rows as f64 * 0.1;
                        dirty = true;
                    }
                    KeyCode::Char('+') | KeyCode::Char('=') => {
                        view.scale = (view.scale * 0.7).max(MIN_SCALE);
                        dirty = true;
                    }
                    KeyCode::Char('-') | KeyCode::Char('_') => {
                        view.scale /= 0.7;
                        dirty = true;
                    }
                    KeyCode::Char('j') => {
                        view.mode = match view.mode {
                            Mode::Mandelbrot => Mode::Julia {
                                c_re: view.center_re,
                                c_im: view.center_im,
                            },
                            Mode::Julia { .. } => Mode::Mandelbrot,
                        };
                        dirty = true;
                    }
                    KeyCode::Char('r') => {
                        view = default_view();
                        dirty = true;
                    }
                    KeyCode::Char(']') => {
                        view.max_iter = (view.max_iter + 50).min(2000);
                        dirty = true;
                    }
                    KeyCode::Char('[') => {
                        view.max_iter = view.max_iter.saturating_sub(50).max(50);
                        dirty = true;
                    }
                    _ => {}
                }
            }
        }
    }

    execute!(stdout, cursor::Show, LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn status_line(view: &View) -> String {
    let mode = match view.mode {
        Mode::Mandelbrot => "mandelbrot",
        Mode::Julia { .. } => "julia",
    };
    let scale_note = if view.scale <= MIN_SCALE { " (max zoom, f64 precision limit)" } else { "" };
    format!(
        "\x1b[0m{} | re {:.6} im {:.6} | scale {:.2e}{} | iter {} | arrows pan  +/- zoom  j toggle  [/] iter  r reset  q quit",
        mode, view.center_re, view.center_im, view.scale, scale_note, view.max_iter
    )
}
