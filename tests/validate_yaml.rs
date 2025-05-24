// tests/validate_yaml.rs
use std::path::Path;
use yaml2_memory_mapper::parser::load_yaml;

#[test]
fn validate_primitive_yaml() {
    let path = Path::new("examples/primitive.yaml");
    let result = load_yaml(path);
    assert!(result.is_ok(), "Failed to parse primitive.yaml: {:?}", result.err());
}

#[test]
fn validate_nested_struct_yaml() {
    let path = Path::new("examples/nested_struct.yaml");
    let result = load_yaml(path);
    assert!(result.is_ok(), "Failed to parse nested_struct.yaml: {:?}", result.err());
}
