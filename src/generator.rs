use crate::model::{EepromMap, Entry, Type};
use crate::utils::size_of;
use std::collections::HashSet;
use std::fmt::Write;

fn generate_struct(name: &str, fields: &[Entry], map: &EepromMap, visited: &mut HashSet<String>, output: &mut String) {
    if visited.contains(name) {
        return;
    }
    visited.insert(name.to_string());

    let mut current_offset = 0;
    writeln!(output, "typedef struct {{").unwrap();
    for field in fields {
        let size = size_of(&field.ty);
        let pad = if (field.offset as usize) > current_offset {
            (field.offset as usize) - current_offset
        } else {
            0
        };
        if pad > 0 {
            writeln!(output, "    uint8_t _pad{}[{}];", current_offset, pad).unwrap();
            current_offset += pad;
        }
        if let Some(desc) = &field.description {
            writeln!(output, "    // {}", desc).unwrap();
        }
        let c_type = match &field.ty {
            Type::Uint8 => "uint8_t".to_string(),
            Type::Uint16 => "uint16_t".to_string(),
            Type::Uint32 => "uint32_t".to_string(),
            Type::Struct(subfields) => {
                let subname = format!("{}_t", field.name);
                generate_struct(&field.name, subfields, map, visited, output);
                subname
            }
            Type::Custom(s) => {
                if let Some(custom_fields) = map.types.get(s) {
                    generate_struct(s, custom_fields, map, visited, output);
                }
                format!("{}_t", s)
            }
        };
        writeln!(output, "    {} {};", c_type, field.name).unwrap();
        current_offset += size;
    }
    writeln!(output, "}} {}_t;\n", name).unwrap();
}

pub fn generate_c_structs(map: &EepromMap) -> String {
    let mut output = String::new();
    writeln!(output, "#pragma once\n#include <stdint.h>\n").unwrap();

    let mut visited = HashSet::new();

    for entry in &map.entries {
        if let Type::Struct(ref fields) = entry.ty {
            generate_struct(&entry.name, fields, map, &mut visited, &mut output);
        } else if let Type::Custom(ref name) = entry.ty {
            if let Some(fields) = map.types.get(name) {
                generate_struct(name, fields, map, &mut visited, &mut output);
            }
        }
    }

    writeln!(output, "typedef struct {{").unwrap();
    let mut current_offset = 0;
    for entry in &map.entries {
        let size = size_of(&entry.ty);
        let pad = if (entry.offset as usize) > current_offset {
            (entry.offset as usize) - current_offset
        } else {
            0
        };
        if pad > 0 {
            writeln!(output, "    uint8_t _pad{}[{}];", current_offset, pad).unwrap();
            current_offset += pad;
        }
        if let Some(desc) = &entry.description {
            writeln!(output, "    // {}", desc).unwrap();
        }
        let c_type = match &entry.ty {
            Type::Uint8 => "uint8_t".to_string(),
            Type::Uint16 => "uint16_t".to_string(),
            Type::Uint32 => "uint32_t".to_string(),
            Type::Struct(_) => format!("{}_t", entry.name),
            Type::Custom(s) => format!("{}_t", s),
        };
        writeln!(output, "    {} {};", c_type, entry.name).unwrap();
        current_offset += size;
    }
    writeln!(output, "}} eeprom_map_t;").unwrap();
    output
}