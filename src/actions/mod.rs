pub mod clone;
pub mod delete;
pub mod install;

pub use clone::clone_repository;
pub use delete::delete_repo;
pub use install::install_dependencies;
