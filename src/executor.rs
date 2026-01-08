use crate::data::*;
use std::process::Command;
use std::io;
use std::io::Write;
use std::env;

pub static mut QUIET: bool = false;

pub fn link(toml: Toml) -> Result<(), String>{
    let sets = match toml.get("LINK") {
        Some(s) => s,
        None => {
            return Err("No LINK section in config file".to_string())
        }
    };

    let mut cmd= vec!["jlink".to_string()];
    for (k, v) in sets {
        if k == "default-arg" {
            for a in v.split(" ") {
                cmd.push(a.to_string());
            }
            continue;
        }

        cmd.push(format!("--{}", k));
        cmd.push(v.to_string());
    }

    Ok(execute(cmd))
}

pub fn package(toml: Toml) -> Result<(), String> {
    let sets = match toml.get("PACKAGE") {
        Some(s) => s,
        None => {
            return Err("No PACKAGE section in config file".to_string())
        }
    };

    let mut cmd= vec!["jpackage".to_string()];
    for (k, v) in sets {
        cmd.push(format!("--{}", k));
        cmd.push(v.to_string());
    }

    Ok(execute(cmd))
}

fn execute(cmd: Vec<String>) {
    let os = env::consts::OS;
    let shell = if os == "windows" { "cmd" } else { "sh" };
    let arg = if os == "windows" { "/c" } else { "-c" };

    if !unsafe { QUIET } {
        println!("\n[INFO]: Loaded config file <pcm.toml>");
        println!("[INFO]: It's going to execute command:");
        println!("=============================================================");
        println!("{} {} {}", shell, arg, cmd.join(" "));
        println!("=============================================================");
        print!("> Do you want to continue? (y/n): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if input.trim().to_lowercase().starts_with("y") {
            println!("\n[INFO]: working...\n");
        } else { println!("\n[INFO]: Cancelled.\n"); return; };
    }

    Command::new(shell).arg(arg).arg(cmd.join(" ")).status().unwrap();
}