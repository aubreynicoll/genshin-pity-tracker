use std::process;

fn main() {
    if let Err(e) = genshin_pity_tracker::run() {
        eprintln!("Application Error: {}", e);
        process::exit(1);
    }
}
