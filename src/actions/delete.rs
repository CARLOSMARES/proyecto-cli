use std::fs;

pub fn delete_repo(location: &str) {
    println!("Eliminando repositorio...");

    match fs::remove_dir_all(location) {
        Ok(_) => println!("✔ Repositorio eliminado"),

        Err(e) => println!("✖ Error eliminando repo: {}", e),
    }
}
