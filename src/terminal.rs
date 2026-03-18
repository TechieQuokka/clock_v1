use crossterm::{
    cursor,
    terminal::{self, ClearType},
    ExecutableCommand,
};
use std::io::{self, Write};

/// RAII guard: enables raw mode on construction, disables on drop.
pub struct RawModeGuard;

impl RawModeGuard {
    pub fn new() -> io::Result<Self> {
        terminal::enable_raw_mode()?;
        Ok(Self)
    }
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        let _ = terminal::disable_raw_mode();
    }
}

/// Move cursor up `n` lines and clear to end of screen.
pub fn redraw_top(n: u16) -> io::Result<()> {
    let mut stdout = io::stdout();
    if n > 0 {
        stdout.execute(cursor::MoveUp(n))?;
    }
    stdout.execute(cursor::MoveToColumn(0))?;
    stdout.execute(terminal::Clear(ClearType::FromCursorDown))?;
    Ok(())
}

/// Print a BEL character to ring the terminal bell.
pub fn bell() {
    print!("\x07");
    let _ = io::stdout().flush();
}

/// Format a duration as MM:SS.mmm
pub fn fmt_duration(d: std::time::Duration) -> String {
    let total_ms = d.as_millis();
    let ms = total_ms % 1000;
    let total_s = total_ms / 1000;
    let secs = total_s % 60;
    let mins = total_s / 60;
    format!("{mins:02}:{secs:02}.{ms:03}")
}

/// Format a duration as HH:MM:SS (for timer/alarm countdown)
pub fn fmt_hms(d: std::time::Duration) -> String {
    let total_s = d.as_secs();
    let secs = total_s % 60;
    let mins = (total_s / 60) % 60;
    let hours = total_s / 3600;
    if hours > 0 {
        format!("{hours:02}:{mins:02}:{secs:02}")
    } else {
        format!("{mins:02}:{secs:02}")
    }
}
