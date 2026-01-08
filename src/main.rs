mod config_manager;
mod executor;
mod clean;
mod data;
mod cross;

use std::env;
use config_manager::*;
use executor::*;
use clean::clean;
use cross::cross;
use crate::data::OS;

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
            "-q" | "--quiet" => unsafe { QUIET = true },
            "-c" | "--config" => { conf = args[i+1].clone(); skip = true; }
            _ => { print_usage(); return; }
        }
    }

    match command.as_str() {
        "-v" | "--version" => { print_version() }
        "-h" | "--help" => { print_usage() }
        "init" => { init(&conf) }

        "link" => { link(load(&conf).unwrap_or_else(|e| {
            eprintln!("{}", e);
            std::process::exit(1);
        })).unwrap_or_else(|e| eprintln!("[ERROR]: {}", e)) }

        "package" => { package(load(&conf).unwrap_or_else(|e| {
            eprintln!("{}", e); std::process::exit(1);
        })).unwrap_or_else(|e| eprintln!("[ERROR]: {}", e)) }

        "clean" => { clean(load(&conf).unwrap_or_else(|e| {
            eprintln!("{}", e); std::process::exit(1);
        })).unwrap_or_else(|e| eprintln!("[ERROR]: {}", e)) }

        "cross-unix" => { cross(&load(&conf).unwrap_or_else(|e| {
            eprintln!("{}", e);
            std::process::exit(1);
        }), OS::Unix).unwrap_or_else(|e| eprintln!("[ERROR]: {}", e))}

        "cross-win" => { cross(&load(&conf).unwrap_or_else(|e| {
            eprintln!("{}", e);
            std::process::exit(1);
        }), OS::Windows).unwrap_or_else(|e| eprintln!("[ERROR]: {}", e))}

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
    println!("  clean\t\t\tclean generated output directory");
    println!("  link\t\t\tuse jlink to make a smaller JRE image");
    println!("  package\t\tuse jpackage to pack a JAR file with JRE image into a distributable package");
    println!("  cross-unix\t\tcreate a fast-build for Unix-like");
    println!("  cross-win\t\tcreate a fast-build for Windows");
    println!("Option:");
    println!("  -c, --config\t\tspecify config path (default: jpc.toml)");
    println!("  -q, --quiet\t\tno check");
    println!("Others:");
    println!("  -h, --help\t\tprint this message");
    println!("  -v, --version\t\tprint version information");
}
