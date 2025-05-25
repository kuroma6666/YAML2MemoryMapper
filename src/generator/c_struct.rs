use crate::generator::c_define::generate_defines;
use crate::model::{EepromMap, Entry, Type};
use crate::utils::type_utils::size_of;
use std::collections::HashSet;
use std::fmt::Write;

fn resolve_c_type(field_type: &Type, field_name: &str) -> String {
    match field_type {
        Type::Uint8 => "uint8_t".into(),
        Type::Uint16 => "uint16_t".into(),
        Type::Uint32 => "uint32_t".into(),
        Type::StructWrapper { .. } => format!("{}_t", field_name),
        Type::Custom(name) | Type::CustomCandidate(name) => format!("{}_t", name),
        Type::Array { base, .. } => resolve_c_type(base, field_name), // NOTE:base 型だけ返す
    }
}

fn generate_padding(output: &mut String, current_offset: usize, target_offset: usize) -> usize {
    if target_offset > current_offset {
        let pad = target_offset - current_offset;
        writeln!(output, "    uint8_t _pad{}[{}];", current_offset, pad).unwrap();
        pad
    } else {
        0
    }
}

fn emit_field_declaration(output: &mut String, ty: &Type, name: &str, map: &EepromMap) {
    match ty {
        Type::Array { base, length } => {
            let base_type = resolve_c_type(base, name);
            writeln!(
                output,
                "    {} {}[{}]; // size: {}",
                base_type,
                name,
                length,
                size_of(ty, map)
            )
            .unwrap();
        }
        _ => {
            let c_type = resolve_c_type(ty, name);
            writeln!(
                output,
                "    {} {}; // size: {}",
                c_type,
                name,
                size_of(ty, map)
            )
            .unwrap();
        }
    }
}

fn generate_struct(
    name: &str,
    fields: &[Entry],
    map: &EepromMap,
    visited: &mut HashSet<String>,
    output: &mut String,
) {
    writeln!(output, "typedef struct {{").unwrap();

    let mut current_offset = 0;
    for field in fields {
        let pad = generate_padding(output, current_offset, field.offset as usize);
        current_offset += pad;

        if let Some(desc) = &field.description {
            writeln!(output, "    // {}", desc).unwrap();
        }

        // NOTE:再帰的構造体出力（必要なら）
        match &field.ty {
            Type::StructWrapper {
                r#struct: sub_fields,
            } => {
                let subname = format!("{}_t", field.name);
                if visited.insert(subname.clone()) {
                    generate_struct(&subname, sub_fields, map, visited, output);
                }
            }
            Type::Custom(name) | Type::CustomCandidate(name) => {
                let subname = format!("{}_t", name);
                if visited.insert(subname.clone()) {
                    if let Some(sub_fields) = map.types.get(name) {
                        generate_struct(&subname, sub_fields, map, visited, output);
                    }
                }
            }
            Type::Array { base, .. } => {
                // NOTE:配列要素の型が構造体なら再帰出力
                match &**base {
                    Type::StructWrapper {
                        r#struct: sub_fields,
                    } => {
                        let subname = format!("{}_t", field.name);
                        if visited.insert(subname.clone()) {
                            generate_struct(&subname, sub_fields, map, visited, output);
                        }
                    }
                    Type::Custom(name) | Type::CustomCandidate(name) => {
                        let subname = format!("{}_t", name);
                        if visited.insert(subname.clone()) {
                            if let Some(sub_fields) = map.types.get(name) {
                                generate_struct(&subname, sub_fields, map, visited, output);
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        emit_field_declaration(output, &field.ty, &field.name, map);
        current_offset += size_of(&field.ty, map);
    }

    writeln!(output, "}} {};\n", name).unwrap();
}

pub fn generate_c_structs(map: &EepromMap) -> String {
    let mut output = String::new();
    writeln!(output, "#pragma once\n#include <stdint.h>\n").unwrap();
    writeln!(output, "{}", generate_defines(map)).unwrap();

    let mut visited = HashSet::new();

    // NOTE:先に user-defined types を展開
    for (name, fields) in &map.types {
        let struct_name = format!("{}_t", name);
        if visited.insert(struct_name.clone()) {
            generate_struct(&struct_name, fields, map, &mut visited, &mut output);
        }
    }

    // NOTE: eeprom_map_t 構造体出力
    writeln!(output, "typedef struct {{").unwrap();
    let mut current_offset = 0;

    for entry in &map.entries {
        let pad = generate_padding(&mut output, current_offset, entry.offset as usize);
        current_offset += pad;

        if let Some(desc) = &entry.description {
            writeln!(output, "    // {}", desc).unwrap();
        }
        // NOTE:再帰的構造体出力（必要なら）
        match &entry.ty {
            Type::StructWrapper {
                r#struct: sub_fields,
            } => {
                let subname = format!("{}_t", entry.name);
                if visited.insert(subname.clone()) {
                    generate_struct(&subname, sub_fields, map, &mut visited, &mut output);
                }
            }
            Type::Custom(name) | Type::CustomCandidate(name) => {
                let subname = format!("{}_t", name);
                if visited.insert(subname.clone()) {
                    if let Some(sub_fields) = map.types.get(name) {
                        generate_struct(&subname, sub_fields, map, &mut visited, &mut output);
                    }
                }
            }
            Type::Array { base, .. } => match &**base {
                Type::StructWrapper {
                    r#struct: sub_fields,
                } => {
                    let subname = format!("{}_t", entry.name);
                    if visited.insert(subname.clone()) {
                        generate_struct(&subname, sub_fields, map, &mut visited, &mut output);
                    }
                }
                Type::Custom(name) | Type::CustomCandidate(name) => {
                    let subname = format!("{}_t", name);
                    if visited.insert(subname.clone()) {
                        if let Some(sub_fields) = map.types.get(name) {
                            generate_struct(&subname, sub_fields, map, &mut visited, &mut output);
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        }

        emit_field_declaration(&mut output, &entry.ty, &entry.name, map);
        current_offset += size_of(&entry.ty, map);
    }

    writeln!(output, "}} eeprom_map_t;\n").unwrap();
    output
}
