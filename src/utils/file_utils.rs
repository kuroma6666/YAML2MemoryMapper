
pub fn get_header_path() -> String {
    std::env::args()
        .nth(2)
        .unwrap_or_else(|| "eeprom_map.h".to_string())
}