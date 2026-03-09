use super::{
    angular::generate_angular, express_api::generate_express_api, ionic::generate_ionic,
    python::generate_python, react::generate_react, rust::generate_rust, vue::generate_vue,
};

pub fn create_project(project_type: &str, name: &str) {
    match project_type {
        "React" => generate_react(name),

        "Angular" => generate_angular(name),

        "Vue" => generate_vue(name),

        "Ionic" => generate_ionic(name),

        "Rust" => generate_rust(name),

        "Python" => generate_python(name),

        "API Express (TypeScript)" => {
            generate_express_api(name, "MySQL", "Prisma", true, true, true, true)
        }

        _ => println!("Tipo de proyecto no soportado"),
    }
}
