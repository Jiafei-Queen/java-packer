mod config_manager;
mod executor;
mod clean;
mod data;

use std::env;
use clean::clean;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let command = {
        match args.get(0) {
            Some(cmd) => cmd,
            None => { print_usage(); return; }
        }
    };

    let mut conf = String::from("jpc.toml");

    let mut skip = false;
    for i in 1..args.len() {
        if skip { skip=false; continue; }

        match args[i].as_str() {
            "-q" | "--quiet" => unsafe { executor::QUIET = true },
            "-c" | "--config" => { conf = args[i+1].clone(); skip = true; }
            _ => { print_usage(); return; }
        }
    }

    match command.as_str() {
        "-v" | "--version" => { print_version() }
        "-h" | "--help" => { print_usage() }
        "init" => { config_manager::init(&conf) }

        "link" => { executor::link(config_manager::load(&conf).unwrap_or_else(|e| {
            eprintln!("{}", e);
            std::process::exit(1);
        })).unwrap_or_else(|e| eprintln!("[ERROR]: {}", e)) }

        "package" => { executor::package(config_manager::load(&conf).unwrap_or_else(|e| {
            eprintln!("{}", e); std::process::exit(1);
        })).unwrap_or_else(|e| eprintln!("[ERROR]: {}", e)) }

        "clean" => { clean(config_manager::load(&conf).unwrap_or_else(|e| {
            eprintln!("{}", e); std::process::exit(1);
        })).unwrap_or_else(|e| eprintln!("[ERROR]: {}", e)) }

        _ => { print_usage() }
    }
}

fn print_version() {
    println!(":: java-packer [jpc] :: (v{})", env!("CARGO_PKG_VERSION"));
}

fn print_usage() {
    println!("Usage: jpc [COMMAND] [OPTION]");
    println!("Command:");
    println!("  init\t\t\tinitialize a new configuration file");
    println!("  link\t\t\tuse jlink to make a smaller JRE image");
    println!("  package\t\tuse jpackage to pack a JAR file with JRE image into a distributable package");
    println!("  clean\t\t\tclean generated output directory");
    println!("Option:");
    println!("  -c, --config\t\tspecify config path (default: jpc.toml)");
    println!("  -q, --quiet\t\tno check");
    println!("Others:");
    println!("  -h, --help\t\tprint this message");
    println!("  -v, --version\t\tprint version information");
}
