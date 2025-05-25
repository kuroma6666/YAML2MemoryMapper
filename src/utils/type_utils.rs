use crate::model::{EepromMap, Type};
pub fn size_of(ty: &Type, map: &EepromMap) -> usize {
    match ty {
        Type::Uint8 => 1,
        Type::Uint16 => 2,
        Type::Uint32 => 4,

        Type::StructWrapper { r#struct: fields } => {
            let mut offset = 0;
            for field in fields {
                let field_size = size_of(&field.ty, map);
                let pad = (field.offset as usize).saturating_sub(offset);
                offset += pad + field_size;
            }
            offset
        }

        Type::Custom(name) | Type::CustomCandidate(name) => {
            if let Some(fields) = map.types.get(name) {
                size_of(
                    &Type::StructWrapper {
                        r#struct: fields.clone(),
                    },
                    map,
                )
            } else {
                0
            }
        }
    }
}

#[allow(dead_code)]
pub fn validate_bounds(offset: u16, ty: &Type, map: &EepromMap, eeprom_size: usize) -> bool {
    let size = size_of(ty, map);
    (offset as usize) + size <= eeprom_size
}
