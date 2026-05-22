mod figures;
mod geometry;
mod render;
mod style;

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{self, Clear, disable_raw_mode, enable_raw_mode},
};
use std::{
    env,
    io::stdout,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::Duration,
};

use crate::{figures::get_figure, render::Configs};
use crate::{render::frame, style::parse_color};

static FPS: u64 = 100;

fn main() {
    let mut stdout = stdout();
    let _ = execute!(stdout, Clear(terminal::ClearType::All), Hide);
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        panic!("Please provide the figure (cube, pyramid, tetrahedron, dodecahedron, icosahedron)");
    }

    let figure_name = &args[1];
    let fig = match get_figure(figure_name) {
        Some(f) => f,
        None => panic!(
            "unknown figure '{}'. available: cube, pyramid, tetrahedron, dodecahedron, icosahedron",
            figure_name
        ),
    };
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

    let mut config = Configs {
        fig: fig,
        dz: 0.0,
        angle: 0.0,
        scale_factor: figure_scale,
        chara: chara,
        color: color,
    };
    enable_raw_mode().unwrap();

    let running = Arc::new(AtomicBool::new(true));
    let render_running = Arc::clone(&running);

    let render_handle = thread::spawn(move || {
        let mut stdout = std::io::stdout();
        let dt = 1.0 / FPS as f64;
        config.dz = 1.5;
        while render_running.load(Ordering::Relaxed) {
            config.angle += 2.0 * std::f64::consts::PI * dt;
            frame(&mut stdout, &config);
            thread::sleep(Duration::from_millis(1000 / FPS));
        }
    });

    while running.load(Ordering::Relaxed) {
        if let Ok(Event::Key(KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
            ..
        })) = event::read()
        {
            running.store(false, Ordering::Relaxed);
        }
    }

    let _ = render_handle.join();
    let _ = execute!(stdout, Show, MoveTo(0, 0));
    let _ = disable_raw_mode();
}
