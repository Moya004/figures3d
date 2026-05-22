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

use crate::render::Configs;
use crate::render::frame;

static FPS: u64 = 100;

fn main() {
    let mut stdout = stdout();
    let _ = execute!(stdout, Clear(terminal::ClearType::All), Hide);
    let args: Vec<String> = env::args().collect();

    let mut config = Configs::new(args);

    enable_raw_mode().unwrap();

    let running = Arc::new(AtomicBool::new(true));
    let render_running = Arc::clone(&running);

    let render_handle = thread::spawn(move || {
        let mut stdout = std::io::stdout();
        let dt = 1.0 / FPS as f64;
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
