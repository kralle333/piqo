use rand::Rng;

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

pub fn to_segments(s: &str, max_len: usize) -> Vec<String> {
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

fn create_printer(fields: Vec<i32>) -> String {
    let sum: i32 = fields.iter().sum();

    if sum > 80 {
        panic!("too long fields")
    }

    return fields
        .iter()
        .map(|f| format!("{{:<{}}}", f))
        .collect::<Vec<String>>()
        .join("|");
}
