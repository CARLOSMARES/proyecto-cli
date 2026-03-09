use std::process::Command;

pub fn clone_repository(repo_url: &str, location: &str) {

    println!("Clonando repositorio...");

    let status = Command::new("git")
        .args(["clone", repo_url, location])
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("✔ Repositorio clonado correctamente");
        }
        _ => println!("✖ Error al clonar el repositorio"),
    }
}