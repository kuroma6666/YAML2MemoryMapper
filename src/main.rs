use yaml2_memory_mapper::app::run;
fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("Usage: eeprom_mapper <input.yaml>");
    if let Err(e) = run(&path) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
