use crate::model::Type;

pub fn size_of(ty: &Type) -> usize {
    match ty {
        Type::Uint8 => 1,
        Type::Uint16 => 2,
        Type::Uint32 => 4,
        Type::Struct(fields) => {
            let mut offset = 0;
            for field in fields {
                let field_size = size_of(&field.ty);
                let pad = if field.offset as usize > offset {
                    field.offset as usize - offset
                } else {
                    0
                };
                offset += pad + field_size;
            }
            offset
        },
        Type::Custom(_) => 0, // 解決されるまで未定義
    }
}

#[allow(dead_code)]
pub fn validate_bounds(offset: u16, ty: &Type, eeprom_size: usize) -> bool {
    let size = size_of(ty);
    (offset as usize) + size <= eeprom_size
}
