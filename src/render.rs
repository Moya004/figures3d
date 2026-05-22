use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal::{self, Clear},
};
use std::io::Stdout;

use crate::figures::Figure;
use crate::geometry::{Point2d, clip_near, mutate_z, project, rotate_xyz, scale};

pub fn width() -> u16 {
    terminal::size().unwrap().0
}

pub fn height() -> u16 {
    terminal::size().unwrap().1
}

pub fn point_to_screen(p: Point2d) -> Point2d {
    Point2d {
        x: (p.x + 1.0) / 2.0 * width() as f64,
        y: (1.0 - (p.y + 1.0) / 2.0) * height() as f64,
    }
}

pub fn draw_point(p: Point2d, stdout: &mut Stdout) {
    if !p.x.is_finite() || !p.y.is_finite() {
        return;
    }
    let point = point_to_screen(p);
    let _ = execute!(
        stdout,
        MoveTo(point.x as u16, point.y as u16),
        Print("."),
        SetForegroundColor(crossterm::style::Color::Magenta)
    );
}

pub fn draw_line(
    a: Point2d,
    b: Point2d,
    stdout: &mut Stdout,
    chara: &char,
    color: crossterm::style::Color,
) {
    if !a.x.is_finite() || !a.y.is_finite() || !b.x.is_finite() || !b.y.is_finite() {
        return;
    }
    let pa = point_to_screen(a);
    let pb = point_to_screen(b);
    let mut x0 = pa.x as i32;
    let mut y0 = pa.y as i32;
    let x1 = pb.x as i32;
    let y1 = pb.y as i32;
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    let w = width() as i32;
    let h = height() as i32;
    loop {
        if x0 >= 0 && y0 >= 0 && x0 < w && y0 < h {
            let _ = execute!(
                stdout,
                MoveTo(x0 as u16, y0 as u16),
                Print(chara),
                SetForegroundColor(color)
            );
        }
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
}

pub struct Configs {
    pub fig: &'static Figure,
    pub dz: f64,
    pub angle: f64,
    pub scale_factor: f64,
    pub chara: char,
    pub color: Color,
}

pub fn frame(
    stdout: &mut Stdout,
    Configs {
        fig,
        dz,
        angle,
        scale_factor,
        chara,
        color,
    }: &Configs,
) {
    let _ = execute!(stdout, Clear(terminal::ClearType::All));

    let near = 0.01;
    for f in fig.faces {
        for i in 0..f.len() {
            let a = &fig.points[f[i]];
            let b = &fig.points[f[(i + 1) % f.len()]];

            let ca = mutate_z(&rotate_xyz(&scale(a, *scale_factor), *angle), *dz);
            let cb = mutate_z(&rotate_xyz(&scale(b, *scale_factor), *angle), *dz);

            if let Some((ca, cb)) = clip_near(&ca, &cb, near) {
                draw_line(project(&ca), project(&cb), stdout, &chara, *color);
            }
        }
    }
}
