use crate::data::*;
use std::process::Command;
use std::io;
use std::io::Write;

pub fn link(toml: Toml) -> Result<(), String>{
    let sets = match toml.get("LINK") {
        Some(s) => s,
        None => {
            return Err("No LINK section in config file".to_string())
        }
    };

    let mut cmd= Vec::new();
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

    Ok(execute("jlink".to_string(), cmd))
}

pub fn package(toml: Toml) -> Result<(), String> {
    let sets = match toml.get("PACKAGE") {
        Some(s) => s,
        None => {
            return Err("No PACKAGE section in config file".to_string())
        }
    };

    let mut cmd= Vec::new();
    for (k, v) in sets {
        cmd.push(format!("--{}", k));
        cmd.push(v.to_string());
    }

    Ok(execute("jpackage".to_string(), cmd))
}

fn execute(program: String, args: Vec<String>) {
    if QUIET.get() == false {
        println!("[INFO]: It's going to execute command:");
        println!("=============================================================");
        println!("{} {}", program, args.join(" "));
        println!("=============================================================");
        print!("> Do you want to continue? (y/n): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if input.trim().to_lowercase().starts_with("y") {
            println!("\n[INFO]: working...\n");
        } else { println!("\n[INFO]: Cancelled.\n"); return; };
    }

    Command::new(program).args(args).status().unwrap();
}