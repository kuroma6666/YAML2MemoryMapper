use yaml2_memory_mapper::parser::load_yaml;
use yaml2_memory_mapper::generator::generate_c_structs;
use std::fs::File;
use std::io::Write;

fn main() {
    let path = std::env::args().nth(1).expect("Usage: eeprom_mapper <input.yaml>");
    match load_yaml(&path) {
        Ok(map) => {
            println!("Successfully loaded EEPROM map: {:?}", map);
            let c_code = generate_c_structs(&map);
            println!("\nGenerated C code:\n{}", c_code);

            let header_path = "eeprom_map.h";
            match File::create(header_path).and_then(|mut f| f.write_all(c_code.as_bytes())) {
                Ok(_) => println!("Header file written to {}", header_path),
                Err(e) => eprintln!("Failed to write header file: {}", e),
            }
        }
        Err(e) => {
            eprintln!("Failed to load EEPROM map: {}", e);
            std::process::exit(1);
        }
    }
}