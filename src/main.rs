use std::{
    fmt::Display,
    io::{self, Stdin, Write},
    ops::{Add, Mul},
};

use termion::{
    event::{Event, Key, MouseEvent},
    input::{MouseTerminal, TermRead},
    raw::IntoRawMode,
};

#[derive(Debug, Clone, Copy)]
struct C {
    pub im: f64,
    pub re: f64,
}

impl Display for C {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} + i * {}", self.re, self.im)
    }
}

impl C {
    fn norm(self) -> f64 {
        self.im * self.im + self.re * self.re
    }
}

impl From<(f64, f64)> for C {
    fn from(value: (f64, f64)) -> Self {
        C {
            re: value.0,
            im: value.1,
        }
    }
}

// (a + ib) * (u + iw)
// (a * u + i * a * w + i * b * u - b * w)
// (a * u - b * w) + i (a * w + b * u)

impl Mul for C {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        C {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

impl Add for C {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        C {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

fn convert_term_coords(
    term_x: u16,
    term_y: u16,
    term_width: u16,
    term_height: u16,
    bounds: ((f64, f64), (f64, f64)),
) -> (f64, f64) {
    let (x_min, width) = bounds.0;
    let (y_min, height) = bounds.1;

    let x = (term_x as f64 / term_width as f64) * width + x_min;
    let y = (term_y as f64 / term_height as f64) * height + y_min;

    return (x, y);
}

fn draw_buffer(buffer: String) {
    print!(
        "{}{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        buffer
    );
}

fn check_convergence(c: C) -> Option<u16> {
    let max_iterations: u16 = 900;
    let mut i = 0;
    let mut z = C { im: 0., re: 0. };
    let cutoff = 10.;

    loop {
        if z.norm() > cutoff {
            return Some(i);
        }

        if i > max_iterations {
            return None;
        }

        z = z * z + c;
        i += 1;
    }
}

fn push_pixel(convergence_result: Option<u16>, buffer: &mut String) {
    let c = match convergence_result {
        None => "@",
        Some(0..=100) => " ",
        Some(101..=200) => ".",
        Some(201..=300) => ":",
        Some(301..=400) => "-",
        Some(401..=500) => "=",
        Some(501..=600) => "+",
        Some(601..=700) => "*",
        Some(701..=800) => "#",
        Some(801..) => "%",
    };

    buffer.push_str(c)
}

fn draw_mandelbrot(term_width: u16, term_height: u16, bounds: ((f64, f64), (f64, f64))) {
    let mut buffer = String::new();

    for term_y in 0..term_height {
        for term_x in 0..term_width {
            let (x, y) = convert_term_coords(term_x, term_y, term_width, term_height, bounds);
            let convergence_result = check_convergence(C::from((x, y)));
            push_pixel(convergence_result, &mut buffer);
        }
    }

    draw_buffer(buffer);
}

fn scale_origin(f: f64, x: f64, x0: f64) -> f64 {
    x - f * x + f * x0
}

fn scale_bounds(
    f: f64,
    term_x: u16,
    term_y: u16,
    term_height: u16,
    term_width: u16,
    bounds: ((f64, f64), (f64, f64)),
) -> ((f64, f64), (f64, f64)) {
    let (x, y) = convert_term_coords(term_x, term_y, term_width, term_height, bounds);
    let (x0_new, y0_new) = (
        scale_origin(f, x, bounds.0 .0),
        scale_origin(f, y, bounds.1 .0),
    );

    ((x0_new, f * bounds.0 .1), (y0_new, f * bounds.1 .1))
}

fn handle_mouse_events(term_height: u16, term_width: u16, mut bounds: ((f64, f64), (f64, f64))) {
    let stdin = io::stdin();
    let mut stdout = MouseTerminal::from(io::stdout().into_raw_mode().unwrap());
    for c in stdin.events() {
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Char('q')) => break,
            Event::Key(k) => {
                match k {
                    Key::Right => bounds.0 .0 += bounds.0 .1 * 0.1,
                    Key::Left => bounds.0 .0 -= bounds.0 .1 * 0.1,
                    Key::Down => bounds.1 .0 += bounds.1 .1 * 0.1,
                    Key::Up => bounds.1 .0 -= bounds.1 .1 * 0.1,
                    _ => (),
                };
                draw_mandelbrot(term_width, term_height, bounds)
            }
            Event::Mouse(MouseEvent::Press(button, term_x, term_y)) => {
                let f = match button {
                    termion::event::MouseButton::Left => 0.5,
                    _ => 1.5,
                };

                bounds = scale_bounds(f, term_x, term_y, term_height, term_width, bounds);

                draw_mandelbrot(term_width, term_height, bounds)
            }
            _ => (),
        }
        stdout.flush().unwrap();
    }
}

fn main() {
    let (term_width, term_height) = termion::terminal_size().unwrap();
    let bounds = ((-3f64, 4f64), (-2f64, 4f64));

    draw_mandelbrot(term_width, term_height, bounds);

    handle_mouse_events(term_height, term_width, bounds);
}
