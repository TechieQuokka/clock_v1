use chrono::{Local, NaiveTime, Timelike};
use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

use crate::terminal::{bell, fmt_hms};

pub fn run(time_str: &str) -> Result<(), String> {
    let target = parse_time(time_str)?;

    println!("Alarm set for {}", format_ampm(target));

    loop {
        let now = Local::now().time();
        let remaining = remaining_secs(now, target);

        print!("\r  {} remaining  ", fmt_hms(Duration::from_secs(remaining)));
        let _ = io::stdout().flush();

        if remaining == 0 {
            break;
        }

        thread::sleep(Duration::from_millis(500));
    }

    println!("\r\x07Alarm! {}         ", format_ampm(target));
    bell();
    Ok(())
}

fn parse_time(s: &str) -> Result<NaiveTime, String> {
    let parts: Vec<&str> = s.splitn(2, ':').collect();
    if parts.len() != 2 {
        return Err(format!("invalid time format '{s}'; expected HH:MM"));
    }
    let hour: u32 = parts[0]
        .parse()
        .map_err(|_| format!("invalid hour '{}'", parts[0]))?;
    let minute: u32 = parts[1]
        .parse()
        .map_err(|_| format!("invalid minute '{}'", parts[1]))?;

    NaiveTime::from_hms_opt(hour, minute, 0)
        .ok_or_else(|| format!("invalid time {hour:02}:{minute:02}"))
}

/// Returns seconds until target, handling midnight rollover.
fn remaining_secs(now: NaiveTime, target: NaiveTime) -> u64 {
    let now_secs = now.num_seconds_from_midnight() as i64;
    let target_secs = target.num_seconds_from_midnight() as i64;
    let mut diff = target_secs - now_secs;
    if diff < 0 {
        diff += 24 * 3600;
    }
    diff as u64
}

fn format_ampm(t: NaiveTime) -> String {
    let h = t.hour();
    let m = t.minute();
    if h < 12 {
        format!("오전 {:02}:{:02}", h, m)
    } else {
        format!("오후 {:02}:{:02}", h % 12, m)
    }
}
