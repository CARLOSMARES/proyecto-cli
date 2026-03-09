use crate::utils::write_file;
use std::path::Path;

pub fn generate_winston(base: &Path) {
    write_file(
        &base.join("src/config/logger.ts"),
        r#"
import winston from 'winston'

export const logger=winston.createLogger({

level:'info',

transports:[
new winston.transports.Console()
]

})
"#,
    );
}
