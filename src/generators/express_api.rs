use std::fs;
use std::path::Path;

use crate::utils::write_file;

use super::{
    auth::generate_auth, clean_arch::generate_clean_arch, docker::generate_docker,
    jest::generate_jest, prisma::generate_prisma, swagger::generate_swagger,
    typeorm::generate_typeorm, winston::generate_winston,
};

pub fn generate_express_api(
    name: &str,
    db: &str,
    orm: &str,
    jwt: bool,
    swagger: bool,
    jest: bool,
    winston: bool,
    docker: bool,
    docker_compose: bool,
) {
    let base = Path::new(name);

    fs::create_dir_all(base).unwrap();

    generate_clean_arch(base);

    let mut dependencies = vec![
        "express".to_string(),
        "dotenv".to_string(),
        "cors".to_string(),
        "typescript".to_string(),
        "ts-node".to_string(),
        "nodemon".to_string(),
        "@types/node".to_string(),
        "@types/express".to_string(),
        "@types/cors".to_string(),
    ];

    let mut dev_dependencies = vec![];

    if orm == "Prisma" {
        dependencies.push("prisma".to_string());
        dependencies.push("@prisma/client".to_string());
        generate_prisma(base, db);
    }

    if orm == "TypeORM" {
        dependencies.push("typeorm".to_string());
        dependencies.push("reflect-metadata".to_string());
        if db == "MySQL" {
            dependencies.push("mysql2".to_string());
        } else if db == "PostgreSQL" {
            dependencies.push("pg".to_string());
        } else if db == "SQLite" {
            dependencies.push("better-sqlite3".to_string());
        }
        generate_typeorm(base, db);
    }

    if jwt {
        dependencies.push("jsonwebtoken".to_string());
        dependencies.push("@types/jsonwebtoken".to_string());
        dependencies.push("bcryptjs".to_string());
        dependencies.push("@types/bcryptjs".to_string());
        generate_auth(base);
    }

    if swagger {
        dependencies.push("swagger-jsdoc".to_string());
        dependencies.push("swagger-ui-express".to_string());
        generate_swagger(base);
    }

    if jest {
        dev_dependencies.push("jest".to_string());
        dev_dependencies.push("@types/jest".to_string());
        dev_dependencies.push("ts-jest".to_string());
        dev_dependencies.push("supertest".to_string());
        dev_dependencies.push("@types/supertest".to_string());
        generate_jest(base);
    }

    if winston {
        dependencies.push("winston".to_string());
        generate_winston(base);
    }

    let deps_str = dependencies
        .iter()
        .map(|d| format!("\"{}\"", d))
        .collect::<Vec<_>>()
        .join(",\n        ");

    let dev_deps_str = dev_dependencies
        .iter()
        .map(|d| format!("\"{}\"", d))
        .collect::<Vec<_>>()
        .join(",\n        ");

    let package_json = format!(
        r#"{{
  "name": "api",
  "scripts": {{
    "dev": "nodemon src/index.ts",
    "build": "tsc",
    "start": "node dist/index.js"
  }},
  "dependencies": {{
    {}
  }},
  "devDependencies": {{
    {}
  }}
}}"#,
        deps_str, dev_deps_str
    );

    write_file(&base.join("package.json"), &package_json);

    if docker {
        generate_docker(base, db, orm, docker_compose);
    }
}
