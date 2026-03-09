use std::process::Command;

pub fn generate_react(name: &str) {
    let npx = if cfg!(target_os = "windows") {
        "npx.cmd"
    } else {
        "npx"
    };

    Command::new(npx)
        .args(["create-react-app", name])
        .status()
        .expect("Error creando proyecto React");
}
