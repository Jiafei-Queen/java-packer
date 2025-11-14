use std::collections::HashMap;
use std::process::Command;
use std::io;
use std::io::Write;
use std::env;

struct Config {
    prefix: String,
    key: String,
}

impl Config {
    fn new(prefix: Vec<&str>, keys: Vec<&str>) -> Result<Vec<Self>, String> {
        if prefix.is_empty() || keys.is_empty() {
            return Err("prefix and keys must not be empty".to_string());
        }

        if prefix.len() != keys.len() {
            return Err("prefix and keys must have the same length".to_string());
        }

        let mut vec: Vec<Self> = Vec::new();
        for i in 0..keys.len() {
            vec.push(Self { prefix: prefix[i].to_string(), key: keys[i].to_string() });
        }

        Ok(vec)
    }
}

fn push_arg(conf: &HashMap<String, String>, sets: Vec<Config>, mut arg: Vec<String>, optional: bool) -> Result<Vec<String>, String> {
    let mut ok = true;
    let mut err = String::from("[ERROR]: Lose config in LINK section in config file <pcm.toml>");

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

pub fn link(config: HashMap<String, HashMap<String, String>>) {
    // 获取 LINK 区
    let conf = {
        match config.get("LINK") {
            Some(c) => c,
            None => { eprintln!("[ERROR]: No LINK section in config file <pcm.toml>"); return; }
        }
    };

    // 初始化设置
    let sets = Config::new(
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
    arg = {
        match push_arg(&conf, sets, arg, false) {
            Ok(v) => v,
            Err(e) => { eprintln!("{}", e); return; }
        }
    };

    // 检查并执行命令
    execute(format!("jlink {}", arg.join(" ")));
}

pub fn package(config: HashMap<String, HashMap<String, String>>) {
    let conf = {
        match config.get("PACKAGE") {
            Some(c) => c,
            None => { eprintln!("[ERROR]: No LINK section in config file <pcm.toml>"); return; }
        }
    };

    let sets = Config::new(
        vec!["--type", "--input", "--main-jar", "--main-class", "--name", "--app-version", "--vendor", "--runtime-image", "--dest"],
        vec!["type", "input", "jar", "main-class", "project-name", "version", "vendor", "runtime-image", "dest"]
    ).unwrap();

    let mut arg = {
        match push_arg(&conf, sets, vec![], false) {
            Ok(v) => v,
            Err(e) => { eprintln!("{}", e); return; }
        }
    };

    let optional_sets = Config::new(
        vec!["--description", "--copyright", "--icon", "--java-options"],
        vec!["description", "copyright", "icon-path", "java-options"]
    ).unwrap();

    // 拼接参数
    arg = push_arg(&conf, optional_sets, arg, true).unwrap();

    // 检查并执行命令
    execute(format!("jpackage {}", arg.join(" ")));
}

fn execute(cmd: String) {
    let os = env::consts::OS;
    let shell = if os == "windows" { "cmd" } else { "sh" };
    let arg = if os == "windows" { "/c" } else { "-c" };
    let cmd = cmd.replace("\"", "\\\"");

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