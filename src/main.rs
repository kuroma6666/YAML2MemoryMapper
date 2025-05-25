mod app;

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("Usage: eeprom_mapper <input.yaml>");
    if let Err(e) = app::run(&path) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
