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

pub fn get_local_git_email() -> Option<String> {
    let output = get_git_email("local");
    let local_email = extract_email(output);
    if local_email.is_some() {
        return local_email;
    }
    extract_email(get_git_email("global"))
}

pub fn gen_4digit_id() -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1000..9999)
}

pub fn get_unused_id(current_ids: Vec<u64>) -> u64 {
    let mut rand_id = gen_4digit_id();

    loop {
        if !current_ids.contains(&rand_id) {
            break;
        }
        rand_id = gen_4digit_id();
    }

    rand_id
}

pub fn truncate(s: &str, max_len: usize) -> String {
    if s.chars().count() > max_len - 1 {
        format!("{}…", &s[..max_len - 1])
    } else {
        s.to_owned()
    }
}
pub fn center_align(text: &str, width: usize) -> String {
    let padding = (width - text.chars().count()) / 2;
    format!(
        "{:<width$}",
        format!("{}{}", " ".repeat(padding), text),
        width = width
    )
}
pub fn left_align(text: &str, width: usize) -> String {
    format!("{:<width$}", text, width = width)
}

pub fn print_line_left(text: &str, width: usize) {
    println!("|{}|", self::left_align(text, width));
}
pub fn print_line_centered(text: &str, width: usize) {
    println!("|{}|", self::center_align(text, width));
}
pub fn print_divider(width: usize) {
    println!("|{}|", "-".repeat(width));
}
pub fn format_description(s: &str, max_len: usize) -> Vec<String> {
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
