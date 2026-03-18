use chrono::Local;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::{
    io::{self, Write},
    time::Duration,
};

use crate::terminal::RawModeGuard;

pub fn run() -> Result<(), String> {
    let _guard = RawModeGuard::new().map_err(|e: std::io::Error| e.to_string())?;

    // Print initial lines so redraw has something to overwrite
    let time_str = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    print!("{}\r\n", time_str);
    print!("  Press Q / Esc / Ctrl+C to exit\r\n");
    io::stdout().flush().map_err(|e| e.to_string())?;

    loop {
        // Poll with 200ms timeout for responsive key handling
        if event::poll(Duration::from_millis(200)).map_err(|e| e.to_string())? {
            if let Event::Key(KeyEvent { code, modifiers, .. }) =
                event::read().map_err(|e| e.to_string())?
            {
                match (code, modifiers) {
                    (KeyCode::Char('c'), KeyModifiers::CONTROL)
                    | (KeyCode::Esc, _)
                    | (KeyCode::Char('q') | KeyCode::Char('Q'), _) => break,
                    _ => {}
                }
            }
        }

        let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        // Move up 2 lines, clear to end, reprint both lines
        print!("\x1B[2A\x1B[J{}\r\n  Press Q / Esc / Ctrl+C to exit\r\n", now);
        io::stdout().flush().map_err(|e| e.to_string())?;
    }

    Ok(())
}
