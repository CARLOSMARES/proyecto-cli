use super::{
    angular::generate_angular, express_api::generate_express_api, ionic::generate_ionic,
    python::generate_python, react::generate_react, rust::generate_rust, vue::generate_vue,
};
use inquire::{Confirm, Select};

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
            )
        }

        _ => println!("Tipo de proyecto no soportado"),
    }
}
