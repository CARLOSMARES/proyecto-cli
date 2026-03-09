use std::fs;
use std::path::Path;

use crate::utils::write_file;

pub fn generate_python(name:&str){

let base = Path::new(name);

fs::create_dir_all(base).unwrap();

write_file(
&base.join("main.py"),
r#"
def main():
    print("Hello Python")

if __name__ == "__main__":
    main()
"#
);

}