use crate::model::{EepromMap, Entry, Type};
use std::collections::HashMap;

pub fn resolve_types(map: &mut EepromMap) -> Result<(), String> {
    let keys: Vec<String> = map.types.keys().cloned().collect();
    for key in keys {
        let types_snapshot = map.types.clone(); // 回避策: cloneで不変参照を確保
        if let Some(fields) = map.types.get_mut(&key) {
            for field in fields.iter_mut() {
                resolve_type(&mut field.ty, &types_snapshot)?;
            }
        }
    }

    let types_snapshot = map.types.clone();
    for entry in &mut map.entries {
        resolve_type(&mut entry.ty, &types_snapshot)?;
    }

    Ok(())
}

fn resolve_type(ty: &mut Type, types: &HashMap<String, Vec<Entry>>) -> Result<(), String> {
    match ty {
        Type::StructWrapper { r#struct: fields } => {
            for field in fields.iter_mut() {
                resolve_type(&mut field.ty, types)?;
            }
            Ok(())
        }
        Type::Custom(name) => {
            if !types.contains_key(name) {
                Err(format!("Undefined custom type: {}", name))
            } else {
                Ok(())
            }
        }
        Type::CustomCandidate(name) => {
            match name.as_str() {
                "uint8" => { *ty = Type::Uint8; Ok(()) }
                "uint16" => { *ty = Type::Uint16; Ok(()) }
                "uint32" => { *ty = Type::Uint32; Ok(()) }
                _ => {
                    if types.contains_key(name) {
                        *ty = Type::Custom(name.clone());
                        Ok(())
                    } else {
                        Err(format!("Unknown type '{}'", name))
                    }
                }
            }
        }
        Type::Uint8 | Type::Uint16 | Type::Uint32 => Ok(()),
    }
}


