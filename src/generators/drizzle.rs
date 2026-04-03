use crate::utils::write_file;
use std::fs;
use std::path::Path;

pub fn generate_drizzle(base: &Path, db: &str) {
    let driver = match db {
        "MySQL" => "mysql2",
        "PostgreSQL" | _ => "postgres",
    };

    let schema = format!(
        r#"
import {{ serial, text, timestamp, mysqlTable }} from 'drizzle-orm/{}';

export const users = mysqlTable('users', {{
    id: serial('id').primaryKey(),
    email: text('email').notNull().unique(),
    password: text('password').notNull(),
    createdAt: timestamp('created_at').defaultNow(),
}});

export type User = typeof users.$inferSelect;
export type NewUser = typeof users.$inferInsert;
"#,
        driver
    );

    let db_dir = base.join("src/db");
    fs::create_dir_all(&db_dir).unwrap();
    write_file(&db_dir.join("schema.ts"), &schema);

    let drizzle_config = match db {
        "MySQL" => {
            r#"
import { drizzle } from 'drizzle-orm/mysql2';
import mysql from 'mysql2/promise';
import * as schema from './schema';

const pool = mysql.createPool({
    host: process.env.DB_HOST || 'localhost',
    port: parseInt(process.env.DB_PORT || '3306'),
    user: process.env.DB_USER || 'root',
    password: process.env.DB_PASSWORD || '',
    database: process.env.DB_NAME || 'api_db',
    waitForConnections: true,
    connectionLimit: 10,
});

export const db = drizzle(pool, {{ schema }});
"#
        }
        "PostgreSQL" | _ => {
            r#"
import {{ drizzle }} from 'drizzle-orm/node-postgres';
import {{ Client }} from 'pg';
import * as schema from './schema';

const client = new Client({{
    host: process.env.DB_HOST || 'localhost',
    port: parseInt(process.env.DB_PORT || '5432'),
    user: process.env.DB_USER || 'postgres',
    password: process.env.DB_PASSWORD || '',
    database: process.env.DB_NAME || 'api_db',
}});

export const db = drizzle(client, {{ schema }});
"#
        }
    };

    write_file(&db_dir.join("index.ts"), &drizzle_config);

    let migrations_dir = base.join("drizzle");
    fs::create_dir_all(&migrations_dir).unwrap();

    let meta = r#"
{
  "version": "1"
}
"#;
    write_file(&migrations_dir.join("meta/_journal.json"), &meta);
}
