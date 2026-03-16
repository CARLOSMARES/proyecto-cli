use crate::utils::write_file;
use std::path::Path;

pub fn generate_docker(base: &Path, db: &str, orm: &str, docker_compose: bool) {
    let dockerfile = r#"FROM node:20-alpine

WORKDIR /app

COPY package*.json ./

RUN npm install

COPY . .

RUN npm run build

EXPOSE 3000

CMD ["npm", "start"]
"#;

    write_file(&base.join("Dockerfile"), dockerfile);

    let (db_image, db_port, db_name, db_user, db_password) = match db {
        "MySQL" => ("mysql:8", "3306", "mydb", "root", "rootpassword"),
        "PostgreSQL" => ("postgres:15", "5432", "mydb", "postgres", "postgres"),
        "SQLite" => ("", "", "", "", ""),
        _ => ("postgres:15", "5432", "mydb", "postgres", "postgres"),
    };

    let db_service = if db == "SQLite" {
        String::new()
    } else {
        format!(
            r#"
  db:
    image: {}
    ports:
      - "{}:{}"
    environment:
      POSTGRES_DB: {}
      POSTGRES_USER: {}
      POSTGRES_PASSWORD: {}
    volumes:
      - db_data:/var/lib/postgresql/data
"#,
            db_image, db_port, db_port, db_name, db_user, db_password
        )
    };

    if docker_compose {
        let depends_on = if db != "SQLite" {
            "    depends_on:\n      - db\n"
        } else {
            ""
        };

        let compose = format!(
            r#"version: '3.8'

services:
  api:
    build: .
    ports:
      - "3000:3000"
    environment:
      - NODE_ENV=development
      - DATABASE_URL={}
{}{}volumes:
  db_data:
"#,
            get_db_url(db, orm, db_name),
            depends_on,
            db_service
        );

        write_file(&base.join("docker-compose.yml"), &compose);
    }

    let dockerignore = "node_modules\ndist\n.env\n*.log\ncoverage\n".to_string();
    write_file(&base.join(".dockerignore"), &dockerignore);
}

fn get_db_url(db: &str, orm: &str, db_name: &str) -> String {
    if db == "SQLite" {
        "file:./dev.db".to_string()
    } else if orm == "Prisma" {
        format!("postgresql://postgres:postgres@db:5432/{}", db_name)
    } else {
        format!(
            "postgresql://{}:{}@db:5432/{}",
            "postgres", "postgres", db_name
        )
    }
}
