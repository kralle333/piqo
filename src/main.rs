mod commands;
mod data_storage;
mod models;
mod utils;

fn main() {
    commands::parse().unwrap();
}
