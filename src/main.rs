mod actions;
mod cli;
mod generators;
mod utils;

use figlet_rs::FIGfont;

fn main() {
    let font = FIGfont::standard().unwrap();
    let figure = font.convert("Create Project CLI");

    if let Some(text) = figure {
        println!("{}", text);
    }

    cli::start_cli();
}
