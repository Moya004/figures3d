use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal::{self, Clear},
};
use std::io::Stdout;

use crate::{
    figures::{Figure, get_figure},
    geometry::{
        Point2d, RotationAxis, clip_near, mutate_z, project, rotate_x, rotate_xy, rotate_xyz,
        rotate_y, rotate_yz, rotate_z, rotate_zx, scale,
    },
    style::parse_color,
};

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

pub fn _draw_point(p: Point2d, stdout: &mut Stdout) {
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
    pub plane: RotationAxis,
}

impl Configs {
    pub fn new(args: Vec<String>) -> Self {
        let fig = args
            .get(1)
            .and_then(|f| get_figure(f))
            .unwrap_or_else(|| get_figure("cube").unwrap());

        let figure_scale = args
            .get(2)
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(2.0);

        let chara = args
            .get(3)
            .and_then(|c| c.parse::<char>().ok())
            .unwrap_or('.');

        let color = args
            .get(4)
            .and_then(|cl| parse_color(cl))
            .unwrap_or(crossterm::style::Color::White);

        let rotate_axis = args
            .get(5)
            .and_then(|plane| Some(*&plane.as_str()))
            .unwrap_or("xyz");

        let distance_from_screen = args
            .get(6)
            .and_then(|d| d.parse::<f64>().ok())
            .unwrap_or(1.5);

        let plane = match rotate_axis {
            "x" => RotationAxis::X,
            "y" => RotationAxis::Y,
            "z" => RotationAxis::Z,
            "xy" | "yx" => RotationAxis::XY,
            "yz" | "zy" => RotationAxis::YZ,
            "xz" | "zx" => RotationAxis::XZ,
            _ => RotationAxis::XYZ,
        };

        Self {
            fig,
            dz: distance_from_screen,
            angle: 0.0,
            scale_factor: figure_scale,
            chara,
            color,
            plane,
        }
    }
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
        plane,
    }: &Configs,
) {
    let _ = execute!(stdout, Clear(terminal::ClearType::All));

    let near = 0.01;
    for f in fig.faces {
        for i in 0..f.len() {
            let a = &fig.points[f[i]];
            let b = &fig.points[f[(i + 1) % f.len()]];

            let scaled_a = scale(a, *scale_factor);
            let scaled_b = scale(b, *scale_factor);

            let (rotated_a, rotated_b) = match *plane {
                RotationAxis::Z => (rotate_z(&scaled_a, *angle), rotate_z(&scaled_b, *angle)),
                RotationAxis::Y => (rotate_y(&scaled_a, *angle), rotate_y(&scaled_b, *angle)),
                RotationAxis::X => (rotate_x(&scaled_a, *angle), rotate_x(&scaled_b, *angle)),
                RotationAxis::XY => (
                    rotate_xy(&scaled_a, *angle, *angle + std::f64::consts::FRAC_PI_4),
                    rotate_xy(&scaled_b, *angle, *angle + std::f64::consts::FRAC_PI_4),
                ),
                RotationAxis::YZ => (
                    rotate_yz(&scaled_a, *angle, *angle + std::f64::consts::FRAC_PI_4),
                    rotate_yz(&scaled_b, *angle, *angle + std::f64::consts::FRAC_PI_4),
                ),
                RotationAxis::XZ => (
                    rotate_zx(&scaled_a, *angle, *angle + std::f64::consts::FRAC_PI_4),
                    rotate_zx(&scaled_b, *angle, *angle + std::f64::consts::FRAC_PI_4),
                ),
                RotationAxis::XYZ => (rotate_xyz(&scaled_a, *angle), rotate_xyz(&scaled_b, *angle)),
            };

            let ca = mutate_z(&rotated_a, *dz);
            let cb = mutate_z(&rotated_b, *dz);

            if let Some((ca, cb)) = clip_near(&ca, &cb, near) {
                draw_line(project(&ca), project(&cb), stdout, &chara, *color);
            }
        }
    }
}
