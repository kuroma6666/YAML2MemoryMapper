use std::fs::{self, File};
use std::io::Write;
use yaml2_memory_mapper::app;

#[test]
fn test_app_run_nonexistent_file() {
    let result = app::run("no_such_file.yaml");
    assert!(result.is_err(), "app::run should fail for nonexistent file");
}

#[test]
fn test_app_run_invalid_yaml() {
    // 不正なYAMLを作成
    let test_path = "test_app_run_invalid.yaml";
    let mut file = File::create(test_path).unwrap();
    file.write_all(b"invalid: [this is not: valid").unwrap();

    let result = app::run(test_path);

    let _ = fs::remove_file(test_path);

    assert!(result.is_err(), "app::run should fail for invalid YAML");
}
