use std::time::Duration;

/// Parse a duration string like "5m", "1h30m", "90s", "1h30m45s"
pub fn parse_duration(s: &str) -> Result<Duration, String> {
    let mut total_secs: u64 = 0;
    let mut num_buf = String::new();
    let mut found_unit = false;

    for ch in s.chars() {
        if ch.is_ascii_digit() {
            num_buf.push(ch);
        } else {
            let n: u64 = if num_buf.is_empty() {
                return Err(format!("expected number before unit '{ch}'"));
            } else {
                num_buf.parse().map_err(|_| format!("number too large: {num_buf}"))?
            };
            num_buf.clear();

            let unit_secs: u64 = match ch {
                'h' => 3600,
                'm' => 60,
                's' => 1,
                _ => return Err(format!("unknown unit '{ch}'; use h, m, or s")),
            };

            let added = n
                .checked_mul(unit_secs)
                .and_then(|v| total_secs.checked_add(v))
                .ok_or_else(|| "duration overflow".to_string())?;
            total_secs = added;
            found_unit = true;
        }
    }

    if !num_buf.is_empty() {
        return Err(format!("missing unit after '{num_buf}'; use h, m, or s"));
    }
    if !found_unit {
        return Err("empty duration string".to_string());
    }

    Ok(Duration::from_secs(total_secs))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seconds() {
        assert_eq!(parse_duration("90s").unwrap(), Duration::from_secs(90));
    }

    #[test]
    fn test_minutes() {
        assert_eq!(parse_duration("5m").unwrap(), Duration::from_secs(300));
    }

    #[test]
    fn test_hours() {
        assert_eq!(parse_duration("1h").unwrap(), Duration::from_secs(3600));
    }

    #[test]
    fn test_combined() {
        assert_eq!(parse_duration("1h30m").unwrap(), Duration::from_secs(5400));
        assert_eq!(parse_duration("1h30m45s").unwrap(), Duration::from_secs(5445));
    }

    #[test]
    fn test_error_no_unit() {
        assert!(parse_duration("42").is_err());
    }

    #[test]
    fn test_error_unknown_unit() {
        assert!(parse_duration("5x").is_err());
    }

    #[test]
    fn test_error_empty() {
        assert!(parse_duration("").is_err());
    }
}
