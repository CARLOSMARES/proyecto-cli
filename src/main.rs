use figlet_rs::FIGfont;
use inquire::{Confirm, Select, Text};
use spinners::{Spinner, Spinners};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::process::Command;

// 1. Clonar repositorio
fn clone_repository(repo_url: &str, clone_location: &str) {
    let mut sp = Spinner::new(Spinners::Dots, "Clonando repositorio...".into());

    let status = Command::new("git")
        .args(["clone", repo_url, clone_location])
        .status();

    match status {
        Ok(s) if s.success() => sp.stop_with_message(format!(
            "✔ Repositorio clonado exitosamente en: {}",
            clone_location
        )),
        _ => sp.stop_with_message(String::from("✖ Error al clonar el repositorio")),
    }
}

// 2. Instalar dependencias
fn install_dependencias(location: &str) {
    let mut sp = Spinner::new(Spinners::Dots, "Instalando Dependencias...".into());
    let npm_cmd = if cfg!(target_os = "windows") {
        "npm.cmd"
    } else {
        "npm"
    };

    let status = Command::new(npm_cmd)
        .arg("install")
        .current_dir(location)
        .status();

    match status {
        Ok(s) if s.success() => {
            sp.stop_with_message(String::from("✔ Dependencias Instaladas Exitosamente"))
        }
        _ => sp.stop_with_message(String::from("✖ Error al Instalar las dependencias")),
    }
}

// 3. Eliminar repo local
fn delete_repo(location: &str) {
    let mut sp = Spinner::new(Spinners::Dots, "Eliminando Repositorio Local...".into());

    match fs::remove_dir_all(location) {
        Ok(_) => sp.stop_with_message(String::from("✔ Directorio eliminado correctamente.")),
        Err(e) => sp.stop_with_message(format!("✖ Error al Eliminar el repositorio: {}", e)),
    }
}

// 4. Utilidad para escribir archivos
fn write_file(path: &Path, content: &str) {
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let mut file = File::create(path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}

// 5. Generador de Proyecto Python
fn create_python_project(project_name: &str) {
    let mut sp = Spinner::new(Spinners::Dots, "Creando entorno de Python...".into());
    let base_path = Path::new(project_name);
    fs::create_dir_all(base_path).unwrap();

    let main_py = "def main():\n    print('¡Hola desde tu nuevo proyecto en Python!')\n\nif __name__ == '__main__':\n    main()\n";
    write_file(&base_path.join("main.py"), main_py);
    write_file(
        &base_path.join("requirements.txt"),
        "# Agrega tus dependencias aquí\n",
    );

    let python_cmd = if cfg!(target_os = "windows") {
        "python"
    } else {
        "python3"
    };

    // Crear entorno virtual
    Command::new(python_cmd)
        .args(["-m", "venv", "venv"])
        .current_dir(base_path)
        .status()
        .unwrap();

    sp.stop_with_message(format!(
        "✔ Proyecto Python '{}' creado con su entorno virtual (venv).",
        project_name
    ));
}

// 6. Generador Avanzado de API Express (TypeScript)
fn create_typescript_api(project_name: &str) {
    println!("\n--- Configuración de API Express (TypeScript) ---");

    let db_options = vec![
        "MySQL",
        "MongoDB",
        "SQL Server",
        "MariaDB",
        "SQLite",
        "Ninguno",
    ];
    let db_choice = Select::new("¿Deseas configurar una base de datos?", db_options)
        .prompt()
        .unwrap();

    let mut orm_choice = "Ninguno";
    if db_choice != "Ninguno" {
        let orm_options = if db_choice == "MongoDB" {
            vec!["Mongoose", "Prisma", "Ninguno"]
        } else {
            vec!["TypeORM", "Prisma", "Ninguno"]
        };
        orm_choice = Select::new("¿Deseas usar un ORM/ODM?", orm_options)
            .prompt()
            .unwrap();
    }

    let docker_options = vec!["Ninguno", "Solo Docker", "Docker y Docker Compose"];
    let docker_choice = Select::new("¿Deseas incluir configuración de Docker?", docker_options)
        .prompt()
        .unwrap();

    let use_swagger = Confirm::new("¿Deseas instalar y configurar Swagger para documentar la API?")
        .with_default(true)
        .prompt()
        .unwrap();

    let use_git = Confirm::new("¿Deseas inicializar un repositorio con Git?")
        .with_default(true)
        .prompt()
        .unwrap();

    println!("\nGenerando proyecto, por favor espera...");
    let mut sp = Spinner::new(Spinners::Dots, "Creando estructura y archivos...".into());

    let base_path = Path::new(project_name);
    fs::create_dir_all(base_path).unwrap();

    // -- Archivos Base (package.json, tsconfig.json, etc.) --
    let package_json = format!(
        r#"{{
  "name": "{}",
  "version": "1.0.0",
  "description": "API TypeScript generada por create-proyect-cli",
  "main": "dist/index.js",
  "scripts": {{
    "start": "node dist/index.js",
    "dev": "nodemon src/index.ts",
    "build": "tsc"
  }}
}}"#,
        project_name
    );
    write_file(&base_path.join("package.json"), &package_json);

    let tsconfig = r#"{
  "compilerOptions": {
    "target": "es2016",
    "module": "commonjs",
    "rootDir": "./src",
    "outDir": "./dist",
    "esModuleInterop": true,
    "strict": true,
    "skipLibCheck": true,
    "experimentalDecorators": true,
    "emitDecoratorMetadata": true
  }
}"#;
    write_file(&base_path.join("tsconfig.json"), tsconfig);

    // -- Variables de Entorno --
    let env_content = match db_choice {
        "MySQL" | "MariaDB" => "PORT=3000\nDATABASE_URL=mysql://root:root@localhost:3306/mydb\n",
        "MongoDB" => "PORT=3000\nDATABASE_URL=mongodb://localhost:27017/mydb\n",
        "SQL Server" => "PORT=3000\nDATABASE_URL=sqlserver://localhost:1433;database=mydb;user=sa;password=Your_password123;\n",
        "SQLite" => "PORT=3000\nDATABASE_URL=file:./dev.db\n",
        _ => "PORT=3000\n"
    };
    write_file(&base_path.join(".env"), env_content);
    write_file(&base_path.join(".env.example"), env_content);

    // -- Docker & Docker Compose --
    if docker_choice != "Ninguno" {
        let dockerfile = r#"FROM node:18-alpine
WORKDIR /usr/src/app
COPY package*.json ./
RUN npm install
COPY . .
RUN npm run build
EXPOSE 3000
CMD ["npm", "start"]
"#;
        write_file(&base_path.join("Dockerfile"), dockerfile);
        write_file(
            &base_path.join(".dockerignore"),
            "node_modules\ndist\n.env\n",
        );

        if docker_choice == "Docker y Docker Compose" {
            let mut compose = String::from("version: '3.8'\nservices:\n  api:\n    build: .\n    ports:\n      - \"3000:3000\"\n    env_file:\n      - .env\n");

            match db_choice {
                "MySQL" | "MariaDB" => {
                    compose.push_str("    depends_on:\n      - db\n");
                    compose.push_str("  db:\n    image: mysql:8\n    environment:\n      MYSQL_ROOT_PASSWORD: root\n      MYSQL_DATABASE: mydb\n    ports:\n      - \"3306:3306\"\n");
                }
                "MongoDB" => {
                    compose.push_str("    depends_on:\n      - db\n");
                    compose
                        .push_str("  db:\n    image: mongo\n    ports:\n      - \"27017:27017\"\n");
                }
                "SQL Server" => {
                    compose.push_str("    depends_on:\n      - db\n");
                    compose.push_str("  db:\n    image: mcr.microsoft.com/mssql/server:2019-latest\n    environment:\n      ACCEPT_EULA: Y\n      SA_PASSWORD: Your_password123\n    ports:\n      - \"1433:1433\"\n");
                }
                _ => {}
            }
            write_file(&base_path.join("docker-compose.yml"), &compose);
        }
    }

    // -- Git --
    if use_git {
        write_file(
            &base_path.join(".gitignore"),
            "node_modules\n.env\ndist\ncoverage\n",
        );
        Command::new("git")
            .arg("init")
            .current_dir(base_path)
            .output()
            .unwrap();
    }

    // -- src/middleware/errorHandler.ts --
    let error_handler = "import { Request, Response, NextFunction } from 'express';\n\nexport const errorHandler = (err: any, req: Request, res: Response, next: NextFunction) => {\n    console.error(err.stack);\n    res.status(500).json({\n        success: false,\n        message: err.message || 'Error interno del servidor',\n    });\n};\n";
    write_file(
        &base_path.join("src/middleware/errorHandler.ts"),
        error_handler,
    );

    // -- src/route/route.ts --
    let mut route_ts =
        String::from("import { Router } from 'express';\n\nexport const router = Router();\n\n");
    if use_swagger {
        route_ts.push_str("/**\n * @swagger\n * /api/hello:\n * get:\n * summary: Retorna un saludo de prueba\n * responses:\n * 200:\n * description: Saludo exitoso\n */\n");
    }
    route_ts.push_str("router.get('/hello', (req, res) => {\n    res.json({ message: '¡Hola desde tu API en TypeScript!' });\n});\n");
    write_file(&base_path.join("src/route/route.ts"), &route_ts);

    // -- src/config/swagger.ts --
    if use_swagger {
        let swagger_ts = "import swaggerJSDoc from 'swagger-jsdoc';\n\nconst options = {\n  definition: {\n    openapi: '3.0.0',\n    info: {\n      title: 'API Express TypeScript',\n      version: '1.0.0',\n      description: 'Documentación de la API generada',\n    },\n  },\n  apis: ['./src/route/*.ts'],\n};\n\nexport const swaggerSpec = swaggerJSDoc(options);\n";
        write_file(&base_path.join("src/config/swagger.ts"), swagger_ts);
    }

    // -- src/database/connection.ts --
    if db_choice != "Ninguno" {
        let db_ts = match orm_choice {
            "TypeORM" => "export const connectDB = async () => {\n    console.log('Conexión a BD pendiente (TypeORM)');\n};\n",
            "Prisma" => "export const connectDB = async () => {\n    console.log('Base de datos gestionada por Prisma.');\n};\n",
            "Mongoose" => "import mongoose from 'mongoose';\n\nexport const connectDB = async () => {\n    try {\n        await mongoose.connect(process.env.DATABASE_URL || '');\n        console.log('MongoDB conectado');\n    } catch (error) {\n        console.error('Error conectando a MongoDB', error);\n    }\n};\n",
            _ => "export const connectDB = async () => {\n    console.log('Configura tu conexión a la base de datos aquí.');\n};\n"
        };
        write_file(&base_path.join("src/database/connection.ts"), db_ts);
    }

    // -- src/index.ts --
    let mut index_ts = String::from("import 'reflect-metadata';\nimport express from 'express';\nimport cors from 'cors';\nimport dotenv from 'dotenv';\nimport { router } from './route/route';\nimport { errorHandler } from './middleware/errorHandler';\n");

    if use_swagger {
        index_ts.push_str("import swaggerUi from 'swagger-ui-express';\nimport { swaggerSpec } from './config/swagger';\n");
    }
    if db_choice != "Ninguno" {
        index_ts.push_str("import { connectDB } from './database/connection';\n");
    }

    index_ts.push_str("\ndotenv.config();\n\nconst app = express();\nconst PORT = process.env.PORT || 3000;\n\napp.use(cors());\napp.use(express.json());\n\n");

    if db_choice != "Ninguno" {
        index_ts.push_str("connectDB();\n\n");
    }
    if use_swagger {
        index_ts
            .push_str("app.use('/api-docs', swaggerUi.serve, swaggerUi.setup(swaggerSpec));\n\n");
    }

    index_ts.push_str("app.use('/api', router);\napp.use(errorHandler);\n\napp.listen(PORT, () => {\n    console.log(`Servidor corriendo en http://localhost:${PORT}`);\n});\n");
    write_file(&base_path.join("src/index.ts"), &index_ts);

    sp.stop_with_message("✔ Archivos y estructura creados exitosamente.".into());

    // -- Instalación de Dependencias --
    let mut sp_deps = Spinner::new(Spinners::Dots, "Instalando dependencias via npm...".into());
    let npm_cmd = if cfg!(target_os = "windows") {
        "npm.cmd"
    } else {
        "npm"
    };

    let mut deps = vec!["express", "cors", "dotenv", "reflect-metadata"];
    let mut dev_deps = vec![
        "typescript",
        "@types/node",
        "@types/express",
        "@types/cors",
        "ts-node",
        "nodemon",
    ];

    if use_swagger {
        deps.extend(vec!["swagger-ui-express", "swagger-jsdoc"]);
        dev_deps.extend(vec!["@types/swagger-ui-express", "@types/swagger-jsdoc"]);
    }

    if orm_choice == "Prisma" {
        deps.push("@prisma/client");
        dev_deps.push("prisma");
    } else if orm_choice == "TypeORM" {
        deps.push("typeorm");
        match db_choice {
            "MySQL" | "MariaDB" => deps.push("mysql2"),
            "SQL Server" => deps.push("mssql"),
            "SQLite" => deps.push("sqlite3"),
            "MongoDB" => deps.push("mongodb"),
            _ => {}
        }
    } else if orm_choice == "Mongoose" {
        deps.push("mongoose");
    }

    Command::new(npm_cmd)
        .args(["install"])
        .args(&deps)
        .current_dir(base_path)
        .output()
        .unwrap();
    Command::new(npm_cmd)
        .args(["install", "-D"])
        .args(&dev_deps)
        .current_dir(base_path)
        .output()
        .unwrap();

    sp_deps.stop_with_message("✔ Dependencias instaladas correctamente.".into());

    if orm_choice == "Prisma" {
        let mut sp_prisma = Spinner::new(Spinners::Dots, "Inicializando Prisma...".into());
        let npx_cmd = if cfg!(target_os = "windows") {
            "npx.cmd"
        } else {
            "npx"
        };
        Command::new(npx_cmd)
            .args(["prisma", "init"])
            .current_dir(base_path)
            .output()
            .unwrap();
        sp_prisma.stop_with_message("✔ Prisma inicializado.".into());
    }

    println!("\n🚀 ¡Proyecto {} configurado exitosamente!", project_name);
}

// 7. Router General de Proyectos
fn create_project(project_type: &str, project_name: &str) {
    let npx_cmd = if cfg!(target_os = "windows") {
        "npx.cmd"
    } else {
        "npx"
    };
    let mut command = Command::new(npx_cmd);
    let mut msg = String::new();

    match project_type {
        "Angular" => {
            command.args(["ng", "new", project_name]);
            msg = String::from("Creando proyecto Angular...");
        }
        "React" => {
            command.args(["create-react-app", project_name]);
            msg = String::from("Creando proyecto React...");
        }
        "Vue" => {
            // Usamos create-vite que es el estándar moderno para Vue
            command.args(["create-vite", project_name, "--template", "vue"]);
            msg = String::from("Creando proyecto Vue (via Vite)...");
        }
        "Ionic" => {
            command.args(["ionic", "start", project_name, "blank"]);
            msg = String::from("Creando proyecto Ionic...");
        }
        "Rust" => {
            let mut cargo_cmd = Command::new("cargo");
            cargo_cmd.args(["new", project_name]);
            let mut sp = Spinner::new(Spinners::Dots, "Creando proyecto Rust...".into());

            match cargo_cmd.status() {
                Ok(s) if s.success() => sp.stop_with_message(format!(
                    "✔ Proyecto Rust '{}' creado exitosamente.",
                    project_name
                )),
                _ => sp.stop_with_message(String::from("✖ Error al crear el proyecto en Rust")),
            }
            return;
        }
        "Python" => {
            create_python_project(project_name);
            return;
        }
        "API Express (TypeScript)" => {
            create_typescript_api(project_name);
            return;
        }
        _ => {
            println!("Tipo de proyecto no reconocido.");
            return;
        }
    }

    let mut sp = Spinner::new(Spinners::Dots, msg);

    match command.status() {
        Ok(s) if s.success() => {
            sp.stop_with_message(format!("✔ Proyecto {} creado exitosamente.", project_name))
        }
        _ => sp.stop_with_message(String::from("✖ Error al crear el proyecto")),
    }
}

// 8. MAIN - Punto de Entrada
fn main() {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("Create Project CLI");
    if let Some(text) = figure {
        println!("{}", text);
    }

    let options = vec![
        "Clonar repositorio",
        "Crear proyecto",
        "Instalar Dependencias",
        "Eliminar Repositorio Local",
    ];

    let action = Select::new("¿Qué acción desea realizar?", options).prompt();

    match action {
        Ok(choice) => {
            if choice == "Clonar repositorio" {
                let repo_url = Text::new("Ingrese la URL del repositorio:")
                    .prompt()
                    .unwrap();
                let clone_location = Text::new("Ingrese la ubicación donde desea clonar:")
                    .with_default("./")
                    .prompt()
                    .unwrap();
                clone_repository(&repo_url, &clone_location);
            } else if choice == "Crear proyecto" {
                let project_types = vec![
                    "Angular",
                    "React",
                    "Vue",
                    "Ionic",
                    "Rust",
                    "Python",
                    "API Express (TypeScript)",
                ];
                let project_type =
                    Select::new("Seleccione el tipo de framework/lenguaje:", project_types)
                        .prompt()
                        .unwrap();
                let project_name = Text::new("Ingrese el nombre del proyecto:")
                    .prompt()
                    .unwrap();
                create_project(project_type, &project_name);
            } else if choice == "Instalar Dependencias" {
                let location =
                    Text::new("Ingrese la ubicación donde desea instalar las dependencias:")
                        .with_default("./")
                        .prompt()
                        .unwrap();
                install_dependencias(&location);
            } else if choice == "Eliminar Repositorio Local" {
                let location = Text::new("Ingrese la ruta del repositorio local:")
                    .with_default("./")
                    .prompt()
                    .unwrap();
                delete_repo(&location);
            }
        }
        Err(_) => println!("Operación cancelada."),
    }
}
