use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::Write;
use crate::data::{Set, Toml};

/// 格式化错误信息
/// - reason: 错误信息
/// - count: （是否输出）错误行号
/// - line:  （是否输出）错误行
/// - path:   配置文件路径
fn format_err(reason: &str, count: Option<usize>, line: Option<&str>, path: &str) -> String {
    const RESET: &str = "\x1b[0m";
    const BOLD: &str = "\x1b[1m";
    const RED: &str = "\x1b[31m";

    let reason = format!("\n\t{}{}{}", RED, reason, RESET);

    let line_print = match line {
        Some(c) => format!("\n{}>>>  {} {}{}", BOLD, count.unwrap_or_else(|| 0), c, RESET),
        None => "".to_string()
    };

    format!(
        "[ERROR]: Failed to load config file <{}>: {}{}",
        path, line_print, reason
    )
}

/// 加载配置文件
pub fn load(path: &str) -> Result<Toml, String> {
    // 尝试读取配置文件
    let content = fs::read_to_string(path)
        .map_err(|_| format_err("No such file", None, None, path))?;

    // 将配置解析成 字符串向量
    let lines: Vec<&str> = content.lines().collect();

    // 变量
    let mut map: Toml = HashMap::new();
    let mut current_section = String::new();
    let mut sets: Set = HashMap::new();

    // 遍历 字符串向量
    let mut count: usize = 0;
    for raw in lines {
        let line = raw.trim();
        count += 1;

        // 跳过无意义的行
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if line.starts_with('[') && line.ends_with(']') {
            // 写入上一个 section
            if !current_section.is_empty() {
                map.insert(current_section.clone(), sets.clone());
                sets.clear();
            }
            current_section = line[1..line.len()-1].to_string();
            continue;
        }

        if current_section.is_empty() {
            return Err(format_err("Missing section", Some(count), Some(line), path));
        }

        let parts: Vec<&str> = line.splitn(2, '=').collect();
        if parts.len() != 2 || !parts[1].trim().starts_with('"') || !parts[1].trim().ends_with('"') {
            return Err(format_err("Invalid key=\"value\"", Some(count), Some(line), path));
        }

        sets.insert(parts[0].trim().to_string(),
                    parts[1].trim()[1..parts[1].trim().len()-1].to_string());
    }

    // 插入最后一节
    if !current_section.is_empty() {
        map.insert(current_section, sets);
    }

    Ok(map)
}

/// 初始化配置文件
pub fn init(path: &str) {
    if fs::exists(path).unwrap() {
        println!("[WARNING]: Config file <{}> already existed", path);
        print!("> Do you want to overwrite it? (y/n): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if !input.trim().to_lowercase().starts_with("y") {
            println!("\n[INFO]: Cancelled.\n");
            return;
        }
    }

    println!("\n[INFO]: Initializing config file...");
    match fs::write(path, get_content()) {
        Ok(_) => println!("[INFO]: Done!\n"),
        Err(e) => eprintln!("[ERROR]: Failed to create config file\n{}\n", e)
    }
}

fn get_content() -> &'static str {
r#"[LINK]
# use ':' to separate path on unix-like
# use ';' to separate path on windows
module-path = "$JAVA_HOME/jmods:$JFX_HOME"

# use ',' without space to separate modules
modules = "java.base,javafx.base,javafx.controls,javafx.graphics"
# 0,1,2 only
compress-level = "2"
output = "runtime"

[PACKAGE]
# general: app-image
# linux: deb, rpm
# macos: dmg, pkg
# windows: exe, msi
type = "app-image"
# if jar-path is 'target/demo-0.0.1.jar'
input = "target"
jar = "demo-0.0.1.jar"

runtime-image = "runtime"
project-name = "example"
main-class = "com.example.demo.Main"
# use '"' to wrap the value if it contains space
version = "0.0.1"
vendor = "'0.0.1 Copyright (C) 2025-forever example.com'"
# these options are optional
# description = "Just a Example"
# copyright = "0.0.1 Copyright (C) 2025-forever"
# icon-path = "src/main/resources/favicon.png"
# java-options = "-Xmx128m"

dest = "dist"
"#
}
