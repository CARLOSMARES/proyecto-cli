use std::fs;
use std::path::Path;

use crate::utils::write_file;

fn log_creating(file: &str) {
    println!("{}", format!("  create   {}", file));
}

fn log_create_dir(dir: &str) {
    println!("{}", format!("  create   {}/", dir));
}

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

    println!("\nCREATE PROJECT {}/\n", name);

    fs::create_dir_all(base).unwrap();

    generate_clean_arch(base, db, orm, jwt, swagger, winston);

    log_create_dir("src/");
    log_create_dir("src/domain/");
    log_create_dir("src/domain/entities/");
    log_creating("src/domain/entities/User.ts");
    log_create_dir("src/application/");
    log_create_dir("src/application/usecases/");
    log_creating("src/application/usecases/createUser.ts");
    log_create_dir("src/infrastructure/");
    log_create_dir("src/infrastructure/controllers/");
    log_creating("src/infrastructure/controllers/userController.ts");
    log_create_dir("src/infrastructure/routes/");
    log_creating("src/infrastructure/routes/route.ts");
    log_creating("src/index.ts");
    log_creating("tsconfig.json");
    log_creating(".env");
    log_creating(".env.example");

    let mut dependencies = vec![
        "express".to_string(),
        "dotenv".to_string(),
        "cors".to_string(),
        "typescript".to_string(),
        "ts-node".to_string(),
        "ts-node-dev".to_string(),
        "@types/node".to_string(),
        "@types/express".to_string(),
        "@types/cors".to_string(),
    ];

    let mut dev_dependencies = vec![];

    if orm == "Prisma" {
        dependencies.push("prisma".to_string());
        dependencies.push("@prisma/client".to_string());
        generate_prisma(base, db);
        log_creating("prisma/schema.prisma");
        log_creating("prisma/.env");
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
        log_creating("src/data-source.ts");
    }

    if jwt {
        dependencies.push("jsonwebtoken".to_string());
        dependencies.push("@types/jsonwebtoken".to_string());
        dependencies.push("bcryptjs".to_string());
        dependencies.push("@types/bcryptjs".to_string());
        generate_auth(base);
        log_create_dir("src/infrastructure/middleware/");
        log_creating("src/infrastructure/middleware/auth.ts");
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
        log_creating("jest.config.js");
        log_creating("test/example.controller.test.ts");
    }

    if winston {
        dependencies.push("winston".to_string());
        generate_winston(base);
        log_creating("src/infrastructure/logger.ts");
    }

    let deps_str = dependencies
        .iter()
        .map(|d| format!("\"{}\": \"*\"", d))
        .collect::<Vec<_>>()
        .join(",\n    ");

    let dev_deps_str = dev_dependencies
        .iter()
        .map(|d| format!("\"{}\": \"*\"", d))
        .collect::<Vec<_>>()
        .join(",\n    ");

    let mut scripts = vec![
        "\"dev\": \"ts-node-dev --respawn --transpile-only src/index.ts\"".to_string(),
        "\"build\": \"tsc\"".to_string(),
        "\"start\": \"node dist/index.js\"".to_string(),
    ];

    if orm == "Prisma" {
        scripts.push("\"migrate:create\": \"prisma migrate dev --name\"".to_string());
        scripts.push("\"migrate:run\": \"prisma migrate deploy\"".to_string());
        scripts.push("\"migrate:revert\": \"prisma migrate reset --force\"".to_string());
    }

    if orm == "TypeORM" {
        scripts.push("\"migration:create\": \"typeorm migration:create\"".to_string());
        scripts.push("\"migration:run\": \"typeorm migration:run\"".to_string());
        scripts.push("\"migration:revert\": \"typeorm migration:revert\"".to_string());
    }

    let scripts_str = scripts.join(",\n        ");

    let package_json = format!(
        r#"{{
  "name": "api",
  "scripts": {{
        {}
  }},
  "dependencies": {{
    {}
  }},
  "devDependencies": {{
    {}
  }}
}}"#,
        scripts_str, deps_str, dev_deps_str
    );

    write_file(&base.join("package.json"), &package_json);
    log_creating("package.json");

    if docker {
        generate_docker(base, db, orm, docker_compose);
        log_creating("Dockerfile");
        if docker_compose {
            log_creating("docker-compose.yml");
        }
    }

    if jest {
        generate_test_example(base);
        log_creating("test/example.controller.test.ts");
    }

    println!("\n✅  Project '{}' created successfully!\n", name);
}

fn generate_test_example(base: &Path) {
    let test_dir = base.join("test");
    fs::create_dir_all(&test_dir).unwrap();

    write_file(
        &test_dir.join("example.controller.test.ts"),
        r#"import request from 'supertest';
import app from '../src/index';

describe('Example Controller Tests', () => {
    it('should return 200 on health check', async () => {
        const response = await request(app).get('/health');
        expect(response.status).toBe(200);
    });

    it('should return 404 for unknown routes', async () => {
        const response = await request(app).get('/unknown');
        expect(response.status).toBe(404);
    });
});
"#,
    );
}
