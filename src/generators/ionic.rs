use std::process::Command;

pub fn generate_ionic(name: &str) {
    let npx = if cfg!(target_os = "windows") {
        "npx.cmd"
    } else {
        "npx"
    };

    println!("Creando proyecto Ionic...");

    Command::new(npx)
        .args(["ionic", "start", name, "blank"])
        .status()
        .expect("Error creando proyecto Ionic");
}
