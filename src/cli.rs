use inquire::{Select, Text};

use crate::actions::*;

use crate::generators::router::create_project;

const VERSION: &str = "2.1.8";

pub fn start_cli() {
    loop {
        let actions = vec![
            "Clonar repositorio",
            "Crear proyecto",
            "Instalar dependencias",
            "Eliminar repositorio local",
            "Ayuda",
            "Salir",
        ];

        let action = Select::new(
            &format!("CLI v{} - ¿Qué acción desea realizar?", VERSION),
            actions,
        )
        .prompt()
        .unwrap();

        match action {
            "Clonar repositorio" => {
                let repo = Text::new("URL del repositorio").prompt().unwrap();

                let location = Text::new("Carpeta destino")
                    .with_default("./")
                    .prompt()
                    .unwrap();

                clone_repository(&repo, &location);
            }

            "Crear proyecto" => {
                let project_types = vec![
                    "React",
                    "Angular",
                    "Vue",
                    "Ionic",
                    "Rust",
                    "Python",
                    "API Express (TypeScript)",
                ];

                let project = Select::new("Seleccione el tipo de proyecto", project_types)
                    .prompt()
                    .unwrap();

                let name = Text::new("Nombre del proyecto").prompt().unwrap();

                create_project(project, &name);
            }

            "Instalar dependencias" => {
                let location = Text::new("Ruta del proyecto")
                    .with_default("./")
                    .prompt()
                    .unwrap();

                install_dependencies(&location);
            }

            "Eliminar repositorio local" => {
                let location = Text::new("Ruta del repositorio").prompt().unwrap();

                delete_repo(&location);
            }

            "Ayuda" => {
                println!("\n=== AYUDA DEL CLI v{} ===\n", VERSION);
                println!(
                    "1. Clonar repositorio: Clona un repositorio git desde una URL a una carpeta local"
                );
                println!("2. Crear proyecto: Crea un nuevo proyecto según el tipo seleccionado");
                println!("   - React: Crea una aplicación React");
                println!("   - Angular: Crea un proyecto Angular");
                println!("   - Vue: Crea un proyecto Vue");
                println!("   - Ionic: Crea un proyecto Ionic");
                println!("   - Rust: Crea un proyecto Rust");
                println!("   - Python: Crea un proyecto Python");
                println!("   - API Express (TypeScript): Crea una API con Express y TypeScript");
                println!("     * Opciones: ORM (Prisma/TypeORM), JWT, Swagger, Jest, Winston, Docker, Docker Compose");
                println!("3. Instalación de dependencias: Instala las dependencias del proyecto (npm/pnpm/bun)");
                println!("4. Eliminar repositorio local: Elimina un repositorio local");
                println!("5. Salir: Termina la ejecución del CLI\n");
                println!("Presione Enter para continuar...");
                let _ = Text::new("").with_default("").prompt();
            }

            "Salir" => {
                println!("¡Hasta luego!");
                break;
            }

            _ => {}
        }
    }
}
