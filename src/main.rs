mod cli;
mod commands;
mod config;

use cli::*;

fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }
    pretty_env_logger::init();

    let cli = Cli::parse();

    match execute(&cli) {
        Ok(_) => (),
        Err(e) => match &e.message {
            Some(message) => {
                println!("{}: {}", e.class, message);
            }
            None => {
                println!("{}", e.class);
            }
        },
    };
}
