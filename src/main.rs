mod commands;
mod data_storage;
mod models;
mod printing;
mod utils;

fn main() {
    // env::set_var("RUST_BACKTRACE", "full");
    let result = commands::parse();
    if let Err(e) = result {
        println!("Error: {}", e);
    }
}
