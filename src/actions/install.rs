use std::path::Path;
use std::process::Command;

pub fn install_dependencies(location: &str) {
    let path = Path::new(location);

    if path.join("pnpm-lock.yaml").exists() {
        run("pnpm", location);
    } else if path.join("bun.lockb").exists() {
        run("bun", location);
    } else if path.join("deno.json").exists() {
        run("deno", location);
    } else {
        run("npm", location);
    }
}

fn run(manager: &str, location: &str) {
    let cmd = if cfg!(target_os = "windows") {
        format!("{}.cmd", manager)
    } else {
        manager.to_string()
    };

    println!("Instalando dependencias con {}...", manager);

    let status = Command::new(cmd)
        .arg("install")
        .current_dir(location)
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("✔ Dependencias instaladas");
        }
        _ => println!("✖ Error instalando dependencias"),
    }
}
