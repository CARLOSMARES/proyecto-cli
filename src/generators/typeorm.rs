use crate::utils::write_file;
use std::path::Path;

pub fn generate_typeorm(base: &Path, db: &str) {
    let driver = match db {
        "MySQL" => "mysql",
        "SQLite" => "sqlite",
        "SQL Server" => "mssql",
        _ => "mysql",
    };

    let ds = format!(
        r#"
import {{DataSource}} from 'typeorm'

export const AppDataSource=new DataSource({{
type:'{}',
url:process.env.DATABASE_URL,
entities:['src/entities/*.ts'],
synchronize:true
}})
"#,
        driver
    );

    write_file(&base.join("src/config/data-source.ts"), &ds);
}
