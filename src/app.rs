use crate::generator::c_struct::generate_c_structs;
use crate::parser::load_yaml;
use crate::utils::file_utils::get_header_path;
use std::fs::File;
use std::io::Write;

pub fn run(path: &str) -> Result<(), String> {
    let map = load_yaml(path).map_err(|e| e.to_string())?;
    println!("Successfully loaded EEPROM map: {:?}", map);

    let c_code = generate_c_structs(&map);
    println!("\nGenerated C code:\n{}", c_code);

    let header_path = get_header_path();
    File::create(&header_path)
        .and_then(|mut f| f.write_all(c_code.as_bytes()))
        .map_err(|e| format!("Failed to write header file: {}", e))?;

    println!("Header file written to {}", header_path);
    Ok(())
}
