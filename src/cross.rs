use crate::data::Toml;
use crate::data::OS;
use fs_extra::dir::{copy, CopyOptions};
use std::fs;
use std::os::unix::fs::PermissionsExt; // 确保已导入

pub fn cross(toml: &Toml, platform: OS) -> Result<(), String> {
    // 获得 CROSS-Section
    let sets = match toml.get("CROSS") {
        Some(s) => s,
        None => return Err("CROSS section not found".to_string())
    };

    // 获得值
    let keys = vec!["output", "jar", "jre"];
    let mut value: Vec<String> = Vec::new();
    for k in keys {
        match sets.get(k) {
            Some(v) => value.push(v.to_string()),
            None => return Err(format!("{} not found in CROSS section", k))
        }
    }

    // 提取参数
    let (output, jar, jre) = (value[0].clone(), value[1].clone(), value[2].clone());

    // 创建 输出根目录
    match fs::create_dir(&output) {
        Err(e) => return Err(format!("Failed to create OUTPUT directory: {}", e)),
        Ok(_) => {}
    }

    // 创建 TARGET 目录
    let target_dir = format!("{}/target", &output);
    match fs::create_dir(&target_dir) {
        Err(e) => return Err(format!("Failed to create TARGET directory: {}", e)),
        Ok(_) => {}
    }

    // 复制 JAR
    let jar_path = format!("{}/{}", target_dir, &jar);
    match fs::copy(&jar, jar_path) {
        Err(e) => return Err(format!("Failed to copy JAR file: {}", e)),
        Ok(_) => {}
    }

    // 设置复制选项
    let options = CopyOptions {
        overwrite: true, // 覆盖已存在的文件
        skip_exist: false, // 不跳过已存在的文件
        buffer_size: 64000, // 缓冲区大小
        copy_inside: true, // 是否复制到目标目录内部
        content_only: false, // 是否只复制内容（不复制目录本身）
        ..CopyOptions::default()
    };

    // 复制 JRE 到 RUNTIME
    let runtime_dir = format!("{}/runtime", &output);
    match copy(&jre, runtime_dir, &options) {
        Err(e) => return Err(format!("Failed to copy JRE: {}", e)),
        Ok(_) => {}
    }

    match platform {
        OS::Unix => {
            // 写入运行脚本
            let script_path = format!("{}/run.sh", output);
            match fs::write(&script_path, get_unix_content(&jar.as_str())) {
                Err(e) => return Err(format!("Failed to write script file: {}", e)),
                Ok(_) => {}
            }

            // 添加脚本运行权限
            match fs::set_permissions(&script_path, fs::Permissions::from_mode(0o755)) {
                Err(e) => return Err(format!("Failed to set script file permissions: {}", e)),
                Ok(_) => {}
            }
        }
        OS::Windows => {
            let script_path = format!("{}/run.bat", output);
            match fs::write(&script_path, get_windows_content(&jar.as_str())) {
                Err(e) => return Err(format!("Failed to write batch file: {}", e)),
                Ok(_) => {}
            }
        }
    }

    Ok(())
}

fn get_unix_content(jar: &str) -> String {
    format!(
        "#!/bin/sh\n\
         SCRIPT_DIR=\"$(cd \"$(dirname \"$0\")\" && pwd)\"\n\
         \n\
         ${{SCRIPT_DIR}}/runtime/bin/java -jar \"${{SCRIPT_DIR}}/target/{}\"\n",
        jar
    )
}

fn get_windows_content(jar: &str) -> String {
    format!(
        "@echo off\n\
         cd /d \"%～dp0\"\n\
         \n\
         .\\runtime\\bin\\java.exe -jar .\\target\\{}\n",
        jar
    )
}