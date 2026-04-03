use inquire::{Confirm, Select, Text};

use crate::actions::*;

use crate::generators::router::create_project;

pub const VERSION: &str = "2.2.0";

pub fn print_help() {
    println!("\n=== AYUDA DEL CLI v{} ===\n", VERSION);
    println!("MODO INTERACTIVO:");
    println!("  Ejecutar sin argumentos para iniciar el modo interactivo");
    println!("  o usar: create-proyect-cli interactive\n");
    println!("MODO COMANDOS:");
    println!("  create-proyect-cli create --template=react --name=my-app");
    println!("  create-proyect-cli generate --template=vue --name=my-vue-app");
    println!("  create-proyect-cli new --template=api --name=my-api --orm=drizzle\n");
    println!("OPCIONES DE TEMPLATE:");
    println!("  react        - React con Vite");
    println!("  vue          - Vue con Vite");
    println!("  angular     - Angular");
    println!("  ionic       - Ionic");
    println!("  api         - API Express (TypeScript)");
    println!("  rust        - Rust");
    println!("  python      - Python\n");
    println!("OPCIONES DE API:");
    println!("  --orm=drizzle|prisma|typeorm  - ORM a usar");
    println!("  --database=mysql|postgresql|sqlite - Base de datos");
    println!("  --jwt       - Incluir autenticación JWT");
    println!("  --security  - Incluir seguridad (helmet, rate limit, validation)");
    println!("  --swagger   - Incluir Swagger");
    println!("  --jest     - Incluir Jest para testing");
    println!("  --winston  - Incluir Winston para logs");
    println!("  --docker   - Incluir Docker");
    println!("  --docker-compose - Incluir Docker Compose");
    println!("  --git      - Inicializar repositorio Git");
    println!("  --install  - Instalar dependencias automáticamente\n");
    println!("EJEMPLOS:");
    println!("  # Crear proyecto React");
    println!("  create-proyect-cli create --template=react --name=my-app --install");
    println!();
    println!("  # Crear API con Drizzle y todas las opciones");
    println!("  create-proyect-cli create --template=api --name=my-api \\");
    println!("    --orm=drizzle --database=postgresql --jwt --security --swagger --install\n");
    println!("Presione Enter para continuar...");
    let _ = Text::new("").with_default("").prompt();
}

pub fn handle_create_command(
    template: Option<String>,
    name: Option<String>,
    orm: Option<String>,
    database: Option<String>,
    jwt: bool,
    security: bool,
    swagger: bool,
    jest: bool,
    winston: bool,
    docker: bool,
    docker_compose: bool,
    git: bool,
    install: bool,
) {
    let template = template.unwrap_or_else(|| {
        let templates = vec!["react", "vue", "angular", "ionic", "api", "rust", "python"];
        let selected = Select::new("Seleccione el tipo de proyecto", templates)
            .prompt()
            .unwrap_or("react")
            .to_string();
        selected
    });

    let project_name = name.unwrap_or_else(|| {
        Text::new("Nombre del proyecto")
            .prompt()
            .unwrap_or_else(|_| "my-project".to_string())
    });

    match template.as_str() {
        "react" => {
            generate_react_with_vite(&project_name, install);
        }
        "vue" => {
            generate_vue_with_vite(&project_name, install);
        }
        "angular" => {
            create_project("Angular", &project_name);
        }
        "ionic" => {
            create_project("Ionic", &project_name);
        }
        "api" => {
            let orm_value = orm.unwrap_or_else(|| {
                let orm_options = vec![
                    "Ninguno".to_string(),
                    "Drizzle".to_string(),
                    "Prisma".to_string(),
                    "TypeORM".to_string(),
                ];
                Select::new("Seleccione el ORM", orm_options)
                    .prompt()
                    .unwrap_or_else(|_| "Ninguno".to_string())
            });

            let db = database.unwrap_or_else(|| {
                let db_options = vec![
                    "MySQL".to_string(),
                    "PostgreSQL".to_string(),
                    "SQLite".to_string(),
                ];
                Select::new("Seleccione la base de datos", db_options)
                    .prompt()
                    .unwrap_or_else(|_| "PostgreSQL".to_string())
            });

            let use_jwt = if !jwt {
                Confirm::new("¿Desea incluir autenticación JWT?")
                    .with_default(true)
                    .prompt()
                    .unwrap_or(true)
            } else {
                jwt
            };

            let use_security = if !security {
                Confirm::new("¿Desea incluir seguridad (helmet, rate limit, validation)?")
                    .with_default(true)
                    .prompt()
                    .unwrap_or(true)
            } else {
                security
            };

            let use_swagger = if !swagger {
                Confirm::new("¿Desea incluir Swagger?")
                    .with_default(true)
                    .prompt()
                    .unwrap_or(true)
            } else {
                swagger
            };

            let use_jest = if !jest {
                Confirm::new("¿Desea incluir Jest para testing?")
                    .with_default(true)
                    .prompt()
                    .unwrap_or(true)
            } else {
                jest
            };

            let use_winston = if !winston {
                Confirm::new("¿Desea incluir Winston para logs?")
                    .with_default(true)
                    .prompt()
                    .unwrap_or(true)
            } else {
                winston
            };

            let use_docker = if !docker {
                Confirm::new("¿Desea incluir Docker?")
                    .with_default(false)
                    .prompt()
                    .unwrap_or(false)
            } else {
                docker
            };

            let use_docker_compose = if docker_compose {
                true
            } else if use_docker {
                Confirm::new("¿Desea incluir Docker Compose con base de datos?")
                    .with_default(true)
                    .prompt()
                    .unwrap_or(true)
            } else {
                false
            };

            generate_express_api(
                &project_name,
                &db,
                &orm_value,
                use_jwt,
                use_security,
                use_swagger,
                use_jest,
                use_winston,
                use_docker,
                use_docker_compose,
            );

            if git {
                init_git_repo(&project_name);
            }

            if install {
                install_dependencies(&project_name);
            }
        }
        "rust" => {
            create_project("Rust", &project_name);
        }
        "python" => {
            create_project("Python", &project_name);
        }
        _ => {
            println!("Template '{}' no soportado", template);
        }
    }
}

fn generate_react_with_vite(name: &str, install: bool) {
    println!("\n🚀  Creando proyecto React con Vite: {}\n", name);

    let output = std::process::Command::new("cmd")
        .args([
            "/C",
            &format!("npx create-vite@latest {} --template react-ts", name),
        ])
        .output();

    match output {
        Ok(result) => {
            if result.status.success() {
                println!("✅  Proyecto React creado con Vite");
                if install {
                    install_dependencies(name);
                }
            } else {
                let error = String::from_utf8_lossy(&result.stderr);
                println!("❌  Error al crear proyecto: {}", error);
            }
        }
        Err(e) => {
            println!("❌  Error ejecutando npx: {}", e);
        }
    }
}

fn generate_vue_with_vite(name: &str, install: bool) {
    println!("\n🚀  Creando proyecto Vue con Vite: {}\n", name);

    let output = std::process::Command::new("cmd")
        .args([
            "/C",
            &format!("npx create-vite@latest {} --template vue-ts", name),
        ])
        .output();

    match output {
        Ok(result) => {
            if result.status.success() {
                println!("✅  Proyecto Vue creado con Vite");
                if install {
                    install_dependencies(name);
                }
            } else {
                let error = String::from_utf8_lossy(&result.stderr);
                println!("❌  Error al crear proyecto: {}", error);
            }
        }
        Err(e) => {
            println!("❌  Error ejecutando npx: {}", e);
        }
    }
}

fn generate_express_api(
    name: &str,
    db: &str,
    orm: &str,
    jwt: bool,
    security: bool,
    swagger: bool,
    jest: bool,
    winston: bool,
    docker: bool,
    docker_compose: bool,
) {
    use crate::generators::express_api::generate_express_api as gen_api;
    gen_api(
        name,
        db,
        orm,
        jwt,
        security,
        swagger,
        jest,
        winston,
        docker,
        docker_compose,
    );
}

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
                println!("MODO INTERACTIVO:");
                println!("  1. Clonar repositorio: Clona un repositorio git desde una URL a una carpeta local");
                println!("  2. Crear proyecto: Crea un nuevo proyecto según el tipo seleccionado");
                println!("     - React: Crea una aplicación React (usa Vite)");
                println!("     - Angular: Crea un proyecto Angular");
                println!("     - Vue: Crea un proyecto Vue (usa Vite)");
                println!("     - Ionic: Crea un proyecto Ionic");
                println!("     - Rust: Crea un proyecto Rust");
                println!("     - Python: Crea un proyecto Python");
                println!("     - API Express (TypeScript): Crea una API con Express y TypeScript");
                println!("       * Opciones: ORM (Drizzle/Prisma/TypeORM), JWT, Seguridad, Swagger, Jest, Winston, Docker, Docker Compose");
                println!("       * Seguridad: helmet, express-rate-limit, zod validation, CORS");
                println!("       * Arquitectura: SOLID (Domain/Application/Infrastructure layers)");
                println!("  3. Instalación de dependencias: Instala las dependencias del proyecto");
                println!("  4. Eliminar repositorio local: Elimina un repositorio local");
                println!("  5. Salir: Termina la ejecución del CLI\n");
                println!("MODO COMANDOS:");
                println!("  create-proyect-cli create --template=react --name=my-app");
                println!("  create-proyect-cli generate --template=vue --name=my-vue-app");
                println!("  create-proyect-cli new --template=api --name=my-api --orm=drizzle\n");
                println!("  Presione Enter para continuar...");
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
