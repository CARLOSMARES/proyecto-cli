use crate::utils::write_file;
use std::path::Path;

pub fn generate_jest(base: &Path) {
    write_file(
        &base.join("jest.config.js"),
        r#"
module.exports={
preset:'ts-jest',
testEnvironment:'node'
}
"#,
    );
}
