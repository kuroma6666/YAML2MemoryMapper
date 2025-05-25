use crate::model::EepromMap;
use std::fmt::Write;

pub fn generate_defines(map: &EepromMap) -> String {
    let mut result = String::new();
    writeln!(&mut result, "#define EEPROM_VERSION {}", map.version).unwrap();
    writeln!(
        &mut result,
        "#define EEPROM_BASE_ADDRESS {}",
        map.base_address
    )
    .unwrap();
    result
}
