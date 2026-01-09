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
use crate::data::{Toml, OS};

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

    // 提前解析，避免 config 提前传来错误
    match command.as_str() {
        "-v" | "--version" => { print_version(); return; }
        "-h" | "--help" => { print_usage(); return;}
        "init" => { init(&conf); return; }
        _ => {}
    }

    let config = load(&conf).unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    });

    parse_cmd(command, config).unwrap_or_else(|e| {
        eprintln!("[ERROR]: {}", e);
    })
}

fn parse_cmd(cmd: &str, config: Toml) -> Result<(), String>{
    match cmd {
        "link" => { Ok(link(config)?) }
        "package" => { Ok(package(config)?) }
        "clean" => { Ok(clean(config)?) }
        "cross-unix" => { Ok(cross(&config, OS::Unix)?) }
        "cross-win" => { Ok(cross(&config, OS::Windows)?) }
        _ => Ok(print_usage())
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
