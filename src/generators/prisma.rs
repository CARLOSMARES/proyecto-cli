use crate::utils::write_file;
use std::path::Path;

pub fn generate_prisma(base: &Path, db: &str) {
    let provider = match db {
        "MySQL" => "mysql",
        "MongoDB" => "mongodb",
        "SQLite" => "sqlite",
        _ => "postgresql",
    };

    let schema = format!(
        r#"
generator client {{
provider="prisma-client-js"
}}

datasource db {{
provider="{}"
url=env("DATABASE_URL")
}}

model User {{
id Int @id @default(autoincrement())
email String @unique
password String
}}
"#,
        provider
    );

    write_file(&base.join("prisma/schema.prisma"), &schema);
}
