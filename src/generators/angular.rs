use std::process::Command;

pub fn generate_angular(name: &str) {

    let npx = if cfg!(target_os = "windows") {
        "npx.cmd"
    } else {
        "npx"
    };

    println!("Creando proyecto Angular...");

    Command::new(npx)
        .args(["@angular/cli", "new", name])
        .status()
        .expect("Error creando proyecto Angular");

}