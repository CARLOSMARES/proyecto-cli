use crate::utils::write_file;
use std::path::Path;

pub fn generate_swagger(base: &Path) {
    write_file(
        &base.join("src/config/swagger.ts"),
        r#"
import swaggerJSDoc from 'swagger-jsdoc'

export const swaggerSpec=swaggerJSDoc({
definition:{
openapi:'3.0.0',
info:{title:'API',version:'1.0.0'}
},
apis:['./src/**/*.ts']
})
"#,
    );
}
