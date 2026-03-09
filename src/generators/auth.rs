use crate::utils::write_file;
use std::path::Path;

pub fn generate_auth(base: &Path) {
    write_file(
        &base.join("src/application/usecases/login.ts"),
        r#"
import jwt from 'jsonwebtoken'

export function login(user){

return jwt.sign(user,process.env.JWT_SECRET)

}
"#,
    );
}
