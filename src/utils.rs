use owo_colors::{colors::xterm, Style};
use rand::Rng;

fn extract_email(output: std::process::Output) -> Option<String> {
    if output.status.success() {
        let git_email = String::from_utf8(output.stdout).unwrap();
        Some(git_email.trim().to_owned())
    } else {
        None
    }
}

fn get_git_email(t: &str) -> std::process::Output {
    std::process::Command::new("git")
        .args(["config", &format!("--{}", t), "user.email"])
        .output()
        .expect("Failed to execute git command")
}

pub(crate) fn get_local_git_email() -> Option<String> {
    let output = get_git_email("local");
    let local_email = extract_email(output);
    if local_email.is_some() {
        return local_email;
    }
    extract_email(get_git_email("global"))
}

pub(crate) fn gen_4digit_id() -> u64 {
    let mut rng = rand::rng();
    rng.random_range(1000..9999)
}

pub(crate) fn get_unused_id(current_ids: Vec<u64>) -> u64 {
    let mut rand_id = gen_4digit_id();

    loop {
        if !current_ids.contains(&rand_id) {
            break;
        }
        rand_id = gen_4digit_id();
    }

    rand_id
}

pub(crate) fn truncate(s: &str, max_len: usize) -> String {
    if s.chars().count() > max_len - 1 {
        format!("{}…", &s[..max_len - 1])
    } else {
        s.to_owned()
    }
}
pub(crate) fn center_align(text: &str, width: usize) -> String {
    format!("{: ^width$}", text, width = width)
}
pub(crate) fn left_align(text: &str, width: usize) -> String {
    format!("{:<width$}", text, width = width)
}

pub(crate) fn truncate_then_center_align(text: &str, width: usize) -> String {
    let truncated = self::truncate(text, width);
    self::center_align(&truncated, width)
}

pub(crate) fn print_line_left(text: &str, width: usize) {
    println!("{}", self::left_align(text, width));
}
pub(crate) fn print_line_centered(text: &str, width: usize) {
    println!("{}", self::center_align(text, width));
}
pub(crate) fn print_divider(width: usize) {
    println!("{}", "-".repeat(width));
}
pub(crate) fn format_description(s: &str, max_len: usize) -> Vec<String> {
    let mut segments = Vec::new();
    s.lines().for_each(|f| {
        segments.extend(self::to_segments(f, max_len));
    });

    segments
}

fn to_segments(s: &str, max_len: usize) -> Vec<String> {
    let mut segments = Vec::new();
    let mut current_segment = String::new();
    let mut current_len = 0;

    for word in s.split_whitespace() {
        if current_len + word.chars().count() + 1 > max_len {
            segments.push(current_segment);
            current_segment = String::new();
            current_len = 0;
        }
        current_segment.push_str(&format!("{} ", word));
        current_len += word.chars().count() + 1;
    }
    segments.push(current_segment);
    segments
}
pub fn display_due_date_time(seconds_till: i64) -> (String, Style) {
    let mut seconds_till = seconds_till;

    let is_negative = seconds_till < 0;

    if is_negative {
        seconds_till = -seconds_till;
    }

    let (time_val, letter, color) = match seconds_till {
        0..=59 => (seconds_till, "s", Style::new().fg::<xterm::GuardsmanRed>()),
        60..=3599 => {
            let minutes = seconds_till / 60;
            (minutes, "m", Style::new().fg::<xterm::DecoOrange>())
        }
        3600..=86399 => {
            let hours = seconds_till / 3600;
            (hours, "h", Style::new().fg::<xterm::GreenYellow>())
        }
        86400..=604799 => {
            let days = seconds_till / 86400;
            (days, "d", Style::new().fg::<xterm::CaribbeanGreen>())
        }
        604800..=31535999 => {
            let weeks = seconds_till / 604800;
            (weeks, "w", Style::new().fg::<xterm::DarkGray>())
        }
        _ => {
            let years = seconds_till / 31536000;
            (years, "y", Style::new().fg::<xterm::White>())
        }
    };
    let mut color = color;
    let mut time_val = time_val.to_string();
    if is_negative {
        color = Style::new().fg::<xterm::GuardsmanRed>();
        time_val = format!("-{}", time_val);
    }

    (format!("{}{}", time_val, letter), color)
}
