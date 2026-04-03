mod actions;
mod args;
mod cli;
mod generators;
mod utils;

use args::CliArgs;
use clap::Parser;
use figlet_rs::FIGfont;

fn main() {
    let args = CliArgs::parse();

    let font = FIGfont::standard().unwrap();
    let figure = font.convert("Create Project CLI");

    if let Some(text) = figure {
        println!("{}", text);
    }

    match args.command {
        Some(args::Commands::Create {
            template,
            name,
            orm,
            database,
            jwt,
            security,
            swagger,
            jest,
            winston,
            docker,
            docker_compose,
            git,
            install,
        }) => {
            cli::handle_create_command(
                template,
                name,
                orm,
                database,
                jwt,
                security,
                swagger,
                jest,
                winston,
                docker,
                docker_compose,
                git,
                install,
            );
        }
        Some(args::Commands::Generate {
            template,
            name,
            orm,
            database,
            jwt,
            security,
            swagger,
            jest,
            winston,
            docker,
            docker_compose,
            git,
            install,
        }) => {
            cli::handle_create_command(
                template,
                name,
                orm,
                database,
                jwt,
                security,
                swagger,
                jest,
                winston,
                docker,
                docker_compose,
                git,
                install,
            );
        }
        Some(args::Commands::New {
            template,
            name,
            orm,
            database,
            jwt,
            security,
            swagger,
            jest,
            winston,
            docker,
            docker_compose,
            git,
            install,
        }) => {
            cli::handle_create_command(
                template,
                name,
                orm,
                database,
                jwt,
                security,
                swagger,
                jest,
                winston,
                docker,
                docker_compose,
                git,
                install,
            );
        }
        Some(args::Commands::Interactive) | None => {
            cli::start_cli();
        }
    }
}
