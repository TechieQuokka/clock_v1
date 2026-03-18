use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::{
    io::{self, Write},
    time::{Duration, Instant},
};

use crate::terminal::{bell, fmt_hms, RawModeGuard};

pub fn run(total: Duration) -> Result<(), String> {
    let _guard = RawModeGuard::new().map_err(|e: std::io::Error| e.to_string())?;
    let deadline = Instant::now() + total;

    println!("Timer: {}  (press Q to quit)", fmt_hms(total));

    loop {
        let now = Instant::now();
        let remaining = deadline.saturating_duration_since(now);

        print!("\r  {}  ", fmt_hms(remaining));
        let _ = io::stdout().flush();

        if remaining.is_zero() {
            break;
        }

        // Poll for keypress with 200ms timeout
        if event::poll(Duration::from_millis(200)).map_err(|e| e.to_string())? {
            if let Event::Key(KeyEvent { code, .. }) =
                event::read().map_err(|e| e.to_string())?
            {
                match code {
                    KeyCode::Char('q') | KeyCode::Char('Q') => {
                        println!("\r\nTimer cancelled.");
                        return Ok(());
                    }
                    _ => {}
                }
            }
        }
    }

    println!("\r\nTime's up!");
    bell();
    Ok(())
}
