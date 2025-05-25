use std::path::Path;
use yaml2_memory_mapper::parser::load_yaml;

#[test]
fn validate_primitive_yaml() {
    let path = Path::new("examples/primitive.yaml");
    let result = load_yaml(path);
    assert!(
        result.is_ok(),
        "Failed to parse primitive.yaml: {:?}",
        result.err()
    );
}

#[test]
fn validate_nested_struct_yaml() {
    let path = Path::new("examples/nested_struct.yaml");
    let result = load_yaml(path);
    assert!(
        result.is_ok(),
        "Failed to parse nested_struct.yaml: {:?}",
        result.err()
    );
}

#[test]
fn validate_custom_autodetect_yaml() {
    let path = Path::new("examples/custom_autodetect.yaml");
    let result = load_yaml(path);
    assert!(
        result.is_ok(),
        "Failed to parse custom_autodetect.yaml: {:?}",
        result.err()
    );
}

#[test]
fn validate_nonexistent_yaml() {
    let path = Path::new("examples/does_not_exist.yaml");
    let result = load_yaml(path);
    assert!(result.is_err(), "Should fail for nonexistent file");
}

#[test]
fn validate_invalid_yaml() {
    let path = Path::new("examples/invalid.yaml");
    let result = load_yaml(path);
    assert!(result.is_err(), "Should fail for invalid YAML");
}
