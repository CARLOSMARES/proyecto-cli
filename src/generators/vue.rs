use std::process::Command;

pub fn generate_vue(name: &str) {

    let npx = if cfg!(target_os = "windows") {
        "npx.cmd"
    } else {
        "npx"
    };

    println!("Creando proyecto Vue...");

    Command::new(npx)
        .args(["create-vite", name, "--template", "vue"])
        .status()
        .expect("Error creando proyecto Vue");

}