use super::{
    angular::generate_angular, express_api::generate_express_api, ionic::generate_ionic,
    python::generate_python, react::generate_react, rust::generate_rust, vue::generate_vue,
};
use inquire::{Confirm, Select};

use crate::actions::install_dependencies;

fn init_git_repo(name: &str) {
    println!("\n🔧  Initializing git repository...");
    let output = std::process::Command::new("git")
        .args(["init"])
        .current_dir(name)
        .output();

    match output {
        Ok(result) => {
            if result.status.success() {
                println!("✅  Git repository initialized successfully!\n");
            } else {
                println!("⚠️  Failed to initialize git repository");
            }
        }
        Err(e) => {
            println!("⚠️  Error running git: {}", e);
        }
    }
}

pub fn create_project(project_type: &str, name: &str) {
    match project_type {
        "React" => generate_react(name),

        "Angular" => generate_angular(name),

        "Vue" => generate_vue(name),

        "Ionic" => generate_ionic(name),

        "Rust" => generate_rust(name),

        "Python" => generate_python(name),

        "API Express (TypeScript)" => {
            let orm_options = vec!["Ninguno", "Prisma", "TypeORM"];
            let orm = Select::new("Seleccione el ORM", orm_options)
                .prompt()
                .unwrap();

            let (db, orm_value) = if orm == "Ninguno" {
                (String::new(), String::new())
            } else {
                let db_options = vec!["MySQL", "PostgreSQL", "SQLite"];
                let db = Select::new("Seleccione la base de datos", db_options)
                    .prompt()
                    .unwrap();
                (db.to_string(), orm.to_string())
            };

            let jwt = Confirm::new("¿Desea incluir autenticación JWT?")
                .with_default(true)
                .prompt()
                .unwrap();

            let swagger = Confirm::new("¿Desea incluir Swagger?")
                .with_default(true)
                .prompt()
                .unwrap();

            let jest = Confirm::new("¿Desea incluir Jest para testing?")
                .with_default(true)
                .prompt()
                .unwrap();

            let winston = Confirm::new("¿Desea incluir Winston para logs?")
                .with_default(true)
                .prompt()
                .unwrap();

            let docker = Confirm::new("¿Desea incluir Docker?")
                .with_default(false)
                .prompt()
                .unwrap();

            let docker_compose = if docker {
                Confirm::new("¿Desea incluir Docker Compose con base de datos?")
                    .with_default(true)
                    .prompt()
                    .unwrap()
            } else {
                false
            };

            let git_init = Confirm::new("¿Desea inicializar repositorio Git?")
                .with_default(true)
                .prompt()
                .unwrap();

            generate_express_api(
                name,
                &db,
                &orm_value,
                jwt,
                swagger,
                jest,
                winston,
                docker,
                docker_compose,
            );

            if git_init {
                init_git_repo(name);
            }

            let install_and_exit = Confirm::new("¿Desea instalar las dependencias y salir?")
                .with_default(true)
                .prompt()
                .unwrap();

            if install_and_exit {
                install_dependencies(name);
                println!("¡Hasta luego!");
                std::process::exit(0);
            }
        }

        _ => println!("Tipo de proyecto no soportado"),
    }
}
