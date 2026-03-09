use inquire::{Select, Text};

use crate::actions::*;

use crate::generators::router::create_project;

pub fn start_cli() {
    let actions = vec![
        "Clonar repositorio",
        "Crear proyecto",
        "Instalar dependencias",
        "Eliminar repositorio local",
    ];

    let action = Select::new("¿Qué acción desea realizar?", actions)
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

        _ => {}
    }
}
