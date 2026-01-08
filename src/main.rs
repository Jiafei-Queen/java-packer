mod config_manager;
mod executor;
mod data;

use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let command = {
        match args.get(0) {
            Some(cmd) => cmd,
            None => { print_usage(); return; }
        }
    };

    let config_path = {
        match args.get(1) {
            Some(path) => path,
            None => "jpc.toml"
        }
    };

    match command.as_str() {
        "-v" | "--version" => { print_version() }
        "-h" | "--help" => { print_usage() }
        "init" => { config_manager::init(config_path) }

        "link" => { executor::link(config_manager::load(config_path).unwrap_or_else(|e| {
            eprintln!("{}", e);
            std::process::exit(1);
        })).unwrap_or_else(|e| eprintln!("[ERROR]: {}", e)) }

        "package" => { executor::package(config_manager::load(config_path).unwrap_or_else(|e| {
            eprintln!("{}", e); std::process::exit(1);
        })).unwrap_or_else(|e| eprintln!("[ERROR]: {}", e)) }

        "clean" => { executor::clean(config_manager::load(config_path).unwrap_or_else(|e| {
            eprintln!("{}", e); std::process::exit(1);
        })).unwrap_or_else(|e| eprintln!("[ERROR]: {}", e)) }

        _ => { print_usage() }
    }
}

fn print_version() {
    println!(":: java-packer [jpc] ::");
    println!("(v{})", env!("CARGO_PKG_VERSION"));
}

fn print_usage() {
    println!("Usage: jpc <command> [config-path]");
    println!("<general>");
    println!("  -h | --help : print this message");
    println!("  -v | --version : print version information");
    println!("<commands>");
    println!("  init : initialize a new configuration file");
    println!("  link : use jlink to make a smaller JRE image");
    println!("  package : use jpackage to pack a JAR file with JRE image into a distributable package");
    println!("[config-path]");
    println!("  specify the path of the configuration file, default is pcm.toml]");
}
