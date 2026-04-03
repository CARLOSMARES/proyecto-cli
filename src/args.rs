use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "create-project-cli")]
#[command(version = "2.2.0")]
#[command(about = "CLI para crear proyectos rápidamente", long_about = None)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Parser, Debug)]
pub enum Commands {
    #[command(about = "Crear un nuevo proyecto")]
    Create {
        #[arg(
            long = "template",
            help = "Tipo de proyecto (react, vue, angular, ionic, api, rust, python)"
        )]
        template: Option<String>,

        #[arg(long = "name", help = "Nombre del proyecto")]
        name: Option<String>,

        #[arg(long = "orm", help = "ORM a usar (drizzle, prisma, typeorm)")]
        orm: Option<String>,

        #[arg(long = "database", help = "Base de datos (mysql, postgresql, sqlite)")]
        database: Option<String>,

        #[arg(long = "jwt", help = "Incluir autenticación JWT")]
        jwt: bool,

        #[arg(
            long = "security",
            help = "Incluir seguridad (helmet, rate limit, validation)"
        )]
        security: bool,

        #[arg(long = "swagger", help = "Incluir Swagger")]
        swagger: bool,

        #[arg(long = "jest", help = "Incluir Jest para testing")]
        jest: bool,

        #[arg(long = "winston", help = "Incluir Winston para logs")]
        winston: bool,

        #[arg(long = "docker", help = "Incluir Docker")]
        docker: bool,

        #[arg(
            long = "docker-compose",
            help = "Incluir Docker Compose con base de datos"
        )]
        docker_compose: bool,

        #[arg(long = "git", help = "Inicializar repositorio Git")]
        git: bool,

        #[arg(long = "install", help = "Instalar dependencias automáticamente")]
        install: bool,
    },

    #[command(about = "Generar un nuevo proyecto (alias de create)")]
    Generate {
        #[arg(long = "template")]
        template: Option<String>,

        #[arg(long = "name")]
        name: Option<String>,

        #[arg(long = "orm")]
        orm: Option<String>,

        #[arg(long = "database")]
        database: Option<String>,

        #[arg(long = "jwt")]
        jwt: bool,

        #[arg(long = "security")]
        security: bool,

        #[arg(long = "swagger")]
        swagger: bool,

        #[arg(long = "jest")]
        jest: bool,

        #[arg(long = "winston")]
        winston: bool,

        #[arg(long = "docker")]
        docker: bool,

        #[arg(long = "docker-compose")]
        docker_compose: bool,

        #[arg(long = "git")]
        git: bool,

        #[arg(long = "install")]
        install: bool,
    },

    #[command(about = "Crear un nuevo proyecto (alias de create)")]
    New {
        #[arg(long = "template")]
        template: Option<String>,

        #[arg(long = "name")]
        name: Option<String>,

        #[arg(long = "orm")]
        orm: Option<String>,

        #[arg(long = "database")]
        database: Option<String>,

        #[arg(long = "jwt")]
        jwt: bool,

        #[arg(long = "security")]
        security: bool,

        #[arg(long = "swagger")]
        swagger: bool,

        #[arg(long = "jest")]
        jest: bool,

        #[arg(long = "winston")]
        winston: bool,

        #[arg(long = "docker")]
        docker: bool,

        #[arg(long = "docker-compose")]
        docker_compose: bool,

        #[arg(long = "git")]
        git: bool,

        #[arg(long = "install")]
        install: bool,
    },

    #[command(about = "Iniciar el CLI en modo interactivo")]
    Interactive,
}

pub const VERSION: &str = "2.2.0";
