//! Value formatting shared by every tab. A missing value is always a dash.

pub const DASH: &str = "-";

/// Character range of the first case-insensitive match of `needle` in `text`.
/// An empty needle matches everything at an empty range.
pub fn find_match(text: &str, needle: &str) -> Option<(usize, usize)> {
    let fold = |c: char| c.to_lowercase().next().unwrap_or(c);
    let hay: Vec<char> = text.chars().map(fold).collect();
    let pin: Vec<char> = needle.chars().map(fold).collect();
    if pin.is_empty() {
        return Some((0, 0));
    }
    hay.windows(pin.len())
        .position(|w| w == pin.as_slice())
        .map(|start| (start, start + pin.len()))
}

pub fn pct(value: f32) -> String {
    format!("{value:.1}%")
}

pub fn pct_opt(value: Option<f32>) -> String {
    value.map(pct).unwrap_or_else(|| DASH.to_string())
}

pub fn bytes(value: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    let mut size = value as f64;
    let mut unit = 0;
    while size >= 1024.0 && unit < UNITS.len() - 1 {
        size /= 1024.0;
        unit += 1;
    }
    if unit == 0 {
        format!("{} {}", value, UNITS[0])
    } else if size >= 100.0 {
        format!("{size:.0} {}", UNITS[unit])
    } else {
        format!("{size:.1} {}", UNITS[unit])
    }
}

pub fn bytes_opt(value: Option<u64>) -> String {
    value.map(bytes).unwrap_or_else(|| DASH.to_string())
}

/// Byte throughput for disks.
pub fn rate(bytes_per_sec: f64) -> String {
    if bytes_per_sec <= 0.0 {
        return "0 B/s".to_string();
    }
    const UNITS: [&str; 4] = ["B/s", "KB/s", "MB/s", "GB/s"];
    let mut size = bytes_per_sec;
    let mut unit = 0;
    while size >= 1024.0 && unit < UNITS.len() - 1 {
        size /= 1024.0;
        unit += 1;
    }
    if unit == 0 {
        format!("{size:.0} {}", UNITS[unit])
    } else {
        format!("{size:.1} {}", UNITS[unit])
    }
}

/// Bit throughput for network adapters, as Windows 10 shows it.
pub fn bit_rate(bytes_per_sec: f64) -> String {
    let bits = bytes_per_sec * 8.0;
    if bits <= 0.0 {
        return "0 bps".to_string();
    }
    const UNITS: [&str; 4] = ["bps", "Kbps", "Mbps", "Gbps"];
    let mut size = bits;
    let mut unit = 0;
    while size >= 1000.0 && unit < UNITS.len() - 1 {
        size /= 1000.0;
        unit += 1;
    }
    if unit == 0 {
        format!("{size:.0} {}", UNITS[unit])
    } else {
        format!("{size:.1} {}", UNITS[unit])
    }
}

/// Clock duration such as 3:04:05, or 2d 03:04:05 for uptimes over a day.
pub fn duration(seconds: u64) -> String {
    let days = seconds / 86_400;
    let rest = seconds % 86_400;
    let hours = rest / 3_600;
    let minutes = (rest % 3_600) / 60;
    let secs = rest % 60;
    if days > 0 {
        format!("{days}d {hours:02}:{minutes:02}:{secs:02}")
    } else if hours > 0 {
        format!("{hours}:{minutes:02}:{secs:02}")
    } else {
        format!("{minutes}:{secs:02}")
    }
}

/// Accumulated CPU time, which is what App history shows.
pub fn cpu_time(ms: u64) -> String {
    duration(ms / 1000)
}

pub fn mhz(value: u64) -> String {
    if value == 0 {
        DASH.to_string()
    } else if value >= 1000 {
        format!("{:.2} GHz", value as f64 / 1000.0)
    } else {
        format!("{value} MHz")
    }
}

pub fn link_speed(value: Option<u64>) -> String {
    match value {
        Some(bits) if bits > 0 => bit_rate(bits as f64 / 8.0),
        _ => DASH.to_string(),
    }
}

/// Cut to width, adding an ellipsis when it does not fit.
pub fn truncate(text: &str, width: usize) -> String {
    if width == 0 {
        return String::new();
    }
    let count = text.chars().count();
    if count <= width {
        let mut out = text.to_string();
        out.push_str(&" ".repeat(width - count));
        return out;
    }
    if width <= 1 {
        return "…".chars().take(width).collect();
    }
    let mut out: String = text.chars().take(width - 1).collect();
    out.push('…');
    out
}

/// Cut to width with no padding and no ellipsis.
pub fn clip(text: &str, width: usize) -> String {
    text.chars().take(width).collect()
}

/// Greedy word wrap. Long words are split at the width.
pub fn wrap(text: &str, width: usize) -> Vec<String> {
    if width == 0 {
        return Vec::new();
    }
    let mut lines = Vec::new();
    let mut current = String::new();
    for word in text.split_whitespace() {
        let mut word = word.to_string();
        while word.chars().count() > width {
            if !current.is_empty() {
                lines.push(std::mem::take(&mut current));
            }
            let head: String = word.chars().take(width).collect();
            lines.push(head);
            word = word.chars().skip(width).collect();
        }
        if current.is_empty() {
            current = word;
        } else if current.chars().count() + 1 + word.chars().count() <= width {
            current.push(' ');
            current.push_str(&word);
        } else {
            lines.push(std::mem::take(&mut current));
            current = word;
        }
    }
    if !current.is_empty() {
        lines.push(current);
    }
    lines
}

pub fn pad_left(text: &str, width: usize) -> String {
    let count = text.chars().count();
    if count >= width {
        return text.chars().take(width).collect();
    }
    format!("{}{}", " ".repeat(width - count), text)
}

pub fn pad_right(text: &str, width: usize) -> String {
    let count = text.chars().count();
    if count >= width {
        return text.chars().take(width).collect();
    }
    format!("{}{}", text, " ".repeat(width - count))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_match_is_case_insensitive_and_char_based() {
        assert_eq!(find_match("Google Chrome", "chr"), Some((7, 10)));
        assert_eq!(find_match("Ünïcode Äpp", "äp"), Some((8, 10)));
        assert_eq!(find_match("node", ""), Some((0, 0)));
        assert_eq!(find_match("node", "x"), None);
    }

    #[test]
    fn percentages_use_one_decimal() {
        assert_eq!(pct(12.5), "12.5%");
        assert_eq!(pct(0.0), "0.0%");
        assert_eq!(pct_opt(None), DASH);
    }

    #[test]
    fn memory_units() {
        assert_eq!(bytes(512), "512 B");
        assert_eq!(bytes(1536), "1.5 KB");
        assert_eq!(bytes(380 * 1024 * 1024), "380 MB");
        assert_eq!(bytes(3 * 1024 * 1024 * 1024), "3.0 GB");
        assert_eq!(bytes_opt(None), DASH);
    }

    #[test]
    fn rates() {
        assert_eq!(rate(0.0), "0 B/s");
        assert_eq!(rate(1024.0), "1.0 KB/s");
        assert_eq!(bit_rate(1_250_000.0), "10.0 Mbps");
        assert_eq!(link_speed(None), DASH);
        assert_eq!(link_speed(Some(1_000_000_000)), "1.0 Gbps");
    }

    #[test]
    fn durations() {
        assert_eq!(duration(59), "0:59");
        assert_eq!(duration(3_661), "1:01:01");
        assert_eq!(duration(3 * 86_400 + 3_661), "3d 01:01:01");
        assert_eq!(cpu_time(61_000), "1:01");
    }

    #[test]
    fn padding_and_truncation() {
        assert_eq!(truncate("abc", 5), "abc  ");
        assert_eq!(truncate("abcdef", 4), "abc…");
        assert_eq!(truncate("abcdef", 1), "…");
        assert_eq!(clip("abcdef", 3), "abc");
        assert_eq!(pad_left("42", 5), "   42");
        assert_eq!(pad_right("ab", 4), "ab  ");
    }

    #[test]
    fn wrapping_respects_width() {
        let lines = wrap("the quick brown fox jumps", 10);
        assert!(lines.iter().all(|l| l.chars().count() <= 10));
        assert_eq!(lines, vec!["the quick", "brown fox", "jumps"]);
        assert_eq!(wrap("abcdefghij", 4), vec!["abcd", "efgh", "ij"]);
        assert!(wrap("", 10).is_empty());
    }

    #[test]
    fn mhz_and_zero_disk() {
        assert_eq!(mhz(0), DASH);
        assert_eq!(mhz(3204), "3.20 GHz");
        assert_eq!(mhz(800), "800 MHz");
        assert_eq!(mhz(0), DASH);
    }
}
