use std::process::Command;

pub fn generate_rust(name: &str) {
    Command::new("cargo")
        .args(["new", name])
        .status()
        .expect("Error creando proyecto Rust");
}
