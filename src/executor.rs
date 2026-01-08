use std::collections::HashMap;
use std::process::Command;
use std::io;
use std::io::Write;
use std::env;
use std::fs::{remove_dir_all, exists};
use crate::data::*;

fn push_arg(conf: &HashMap<String, String>, sets: Vec<Config>, mut arg: Vec<String>, optional: bool) -> Result<Vec<String>, String> {
    let mut ok = true;
    let mut err = String::from("[ERROR]: Loss config in the config file");

    // 判空和读取并添加配置
    let mut num = 1;
    for i in 0..sets.len() {
        match conf.get(&sets[i].key) {
            None => {
                // 添加
                if ok { ok = false; }
                err.push_str(format!("\n{}. [{}]",num, &sets[i].key).as_str());
                num += 1;
            }
            Some(c) => {
                // 推参数
                arg.push(sets[i].prefix.clone());
                arg.push(c.to_string());
            }
        }
    }

    if ok || optional { Ok(arg) } else { Err(err) }
}

pub fn link(config: HashMap<String, HashMap<String, String>>) -> Result<(), String> {
    // 获取 LINK 区
    let conf = {
        match config.get("LINK") {
            Some(c) => c,
            None => { return Err("No LINK section in config file".to_string()) }
        }
    };

    // 初始化设置
    let sets = Config::from_twin_vec(
        vec!["--module-path", "--add-modules", "--compress", "--output"],
        vec!["module-path", "modules", "compress-level", "output"]
    ).unwrap();

    // 初始化参数
    let mut arg = vec![
        "--no-header-files".to_string(),
        "--no-man-pages".to_string(),
        "--strip-debug".to_string(),
    ];

    // 拼接参数
    arg = push_arg(&conf, sets, arg, false)?;

    // 检查并执行命令
    Ok(execute(format!("jlink {}", arg.join(" "))))
}

pub fn package(config: HashMap<String, HashMap<String, String>>) -> Result<(), String> {
    let conf = {
        match config.get("PACKAGE") {
            Some(c) => c,
            None => { return Err("No LINK section in config file".to_string()) }
        }
    };

    let sets = Config::from_twin_vec(
        vec!["--type", "--input", "--main-jar", "--main-class", "--name", "--app-version", "--vendor", "--runtime-image", "--dest"],
        vec!["type", "input", "jar", "main-class", "project-name", "version", "vendor", "runtime-image", "dest"]
    ).unwrap();

    let mut arg = push_arg(&conf, sets, vec![], false)?;

    let optional_sets = Config::from_twin_vec(
        vec!["--description", "--copyright", "--icon", "--java-options"],
        vec!["description", "copyright", "icon-path", "java-options"]
    ).unwrap();

    // 拼接参数
    arg = push_arg(&conf, optional_sets, arg, true).unwrap();

    // 检查并执行命令
    Ok(execute(format!("jpackage {}", arg.join(" "))))
}

pub fn clean(config: HashMap<String, HashMap<String, String>>) -> Result<(), String> {
    const LEN: usize = 2;
    let set: [(String, String); LEN] = [
        ("LINK".to_string(), "output".to_string()),
        ("PACKAGE".to_string(), "dest".to_string()),
    ];

    let mut is_cleaned = false;
    for i in 0..LEN {
        let (section, key) = &set[i];

        match config.get(section.as_str()) {
            Some(m) => {
                // 在其中找到 output 值
                match m.get(key.as_str()) {
                    Some(o) => {
                        match remove_dir_all(o) {
                            Err(e) => {
                                if exists(o).unwrap() {
                                    eprintln!("[ERROR]: Failed to remove <{}>\n\t{}", o, e);
                                }
                            }
                            Ok(_) => {
                                is_cleaned = true;
                                println!("[INFO]: Removed {}", o);
                            }
                        }
                    },
                    None => {
                        return Err(format!("No {} in {} section in config file", key, section))
                    }
                }
            },
            None => {
                return Err(format!("No {} section in config file", section))
            }
        }
    }

    if !is_cleaned {
        println!("Nothing was cleaned.");
    }

    Ok(())
}

fn execute(cmd: String) {
    let os = env::consts::OS;
    let shell = if os == "windows" { "cmd" } else { "sh" };
    let arg = if os == "windows" { "/c" } else { "-c" };

    println!("\n[INFO]: Loaded config file <pcm.toml>");
    println!("[INFO]: It's going to execute command:");
    println!("=============================================================");
    println!("{} {} {}", shell, arg, cmd);
    println!("=============================================================");
    print!("> Do you want to continue? (y/n): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    if input.trim().to_lowercase().starts_with("y") {
        println!("\n[INFO]: working...\n");
    } else { println!("\n[INFO]: Cancelled.\n"); return; };

    Command::new(shell).arg(arg).arg(cmd).status().unwrap();
}