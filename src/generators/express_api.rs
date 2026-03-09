use std::fs;
use std::path::Path;

use crate::utils::write_file;

use super::{
    auth::generate_auth, clean_arch::generate_clean_arch, jest::generate_jest,
    prisma::generate_prisma, swagger::generate_swagger, typeorm::generate_typeorm,
    winston::generate_winston,
};

pub fn generate_express_api(
    name: &str,
    db: &str,
    orm: &str,
    jwt: bool,
    swagger: bool,
    jest: bool,
    winston: bool,
) {
    let base = Path::new(name);

    fs::create_dir_all(base).unwrap();

    generate_clean_arch(base);

    write_file(
        &base.join("package.json"),
        r#"{
"name":"api",
"scripts":{
"dev":"nodemon src/index.ts",
"build":"tsc",
"start":"node dist/index.js"
}
}"#,
    );

    if orm == "Prisma" {
        generate_prisma(base, db);
    }

    if orm == "TypeORM" {
        generate_typeorm(base, db);
    }

    if jwt {
        generate_auth(base);
    }

    if swagger {
        generate_swagger(base);
    }

    if jest {
        generate_jest(base);
    }

    if winston {
        generate_winston(base);
    }
}
