use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::{
    io::{self, Write},
    time::{Duration, Instant},
};

use crate::terminal::{fmt_duration, redraw_top, RawModeGuard};

enum State {
    Idle,
    Running { started: Instant, base: Duration },
    Paused { elapsed: Duration },
}

impl State {
    fn elapsed(&self) -> Duration {
        match self {
            State::Idle => Duration::ZERO,
            State::Running { started, base } => *base + started.elapsed(),
            State::Paused { elapsed } => *elapsed,
        }
    }

    fn is_running(&self) -> bool {
        matches!(self, State::Running { .. })
    }
}

pub fn run() -> Result<(), String> {
    let _guard = RawModeGuard::new().map_err(|e: std::io::Error| e.to_string())?;

    let mut state = State::Idle;
    let mut laps: Vec<Duration> = Vec::new();
    // Lines rendered in the previous frame (used for redraw_top).
    let mut prev_lines: u16 = 0;

    // Initial render
    render(&state, &laps, prev_lines)?;
    prev_lines = render_line_count(&laps);

    loop {
        if event::poll(Duration::from_millis(100)).map_err(|e| e.to_string())? {
            if let Event::Key(KeyEvent { code, modifiers, .. }) =
                event::read().map_err(|e| e.to_string())?
            {
                match (code, modifiers) {
                    // Ctrl+C or Esc: force quit
                    (KeyCode::Char('c'), KeyModifiers::CONTROL) | (KeyCode::Esc, _) => {
                        redraw_top(prev_lines).map_err(|e| e.to_string())?;
                        print!("Stopped.\r\n");
                        io::stdout().flush().ok();
                        break;
                    }
                    // Space: toggle start / pause
                    (KeyCode::Char(' '), _) => {
                        state = match state {
                            State::Idle => State::Running {
                                started: Instant::now(),
                                base: Duration::ZERO,
                            },
                            State::Running { started, base } => State::Paused {
                                elapsed: base + started.elapsed(),
                            },
                            State::Paused { elapsed } => State::Running {
                                started: Instant::now(),
                                base: elapsed,
                            },
                        };
                    }
                    // L: record lap
                    (KeyCode::Char('l') | KeyCode::Char('L'), _) => {
                        if state.is_running() {
                            laps.push(state.elapsed());
                        }
                    }
                    // R: reset
                    (KeyCode::Char('r') | KeyCode::Char('R'), _) => {
                        state = State::Idle;
                        laps.clear();
                    }
                    // Q: quit
                    (KeyCode::Char('q') | KeyCode::Char('Q'), _) => {
                        redraw_top(prev_lines).map_err(|e| e.to_string())?;
                        print!("Stopped.\r\n");
                        io::stdout().flush().ok();
                        break;
                    }
                    _ => {}
                }
            }
        }

        render(&state, &laps, prev_lines)?;
        prev_lines = render_line_count(&laps);
    }

    Ok(())
}

/// Lines to MoveUp before re-rendering.
/// = number of \r\n outputs = lap lines + separator + timer line
/// (help line has no trailing \r\n, so cursor stays on that row)
fn render_line_count(laps: &[Duration]) -> u16 {
    laps.len() as u16 + 2
}

fn render(state: &State, laps: &[Duration], prev_lines: u16) -> Result<(), String> {
    redraw_top(prev_lines).map_err(|e| e.to_string())?;

    let mut stdout = io::stdout();

    for (i, lap) in laps.iter().enumerate() {
        write!(stdout, "  Lap {:<3}  {}\r\n", i + 1, fmt_duration(*lap))
            .map_err(|e| e.to_string())?;
    }

    write!(stdout, "  {}\r\n", "─".repeat(20)).map_err(|e| e.to_string())?;

    let status = match state {
        State::Idle => "[IDLE]",
        State::Running { .. } => "[RUNNING]",
        State::Paused { .. } => "[PAUSED]",
    };
    write!(stdout, "  {}  {}\r\n", fmt_duration(state.elapsed()), status)
        .map_err(|e| e.to_string())?;

    write!(stdout, "  Space=start/pause  L=lap  R=reset  Q=quit")
        .map_err(|e| e.to_string())?;

    stdout.flush().map_err(|e| e.to_string())?;
    Ok(())
}
