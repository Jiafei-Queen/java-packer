#![windows_subsystem = "windows"]

use std::env;
use std::fs;
use std::process::Command;
use std::os::windows::process::CommandExt;

fn main() {
    let exe_dir = env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();

    let target_dir = exe_dir.join("target");

    let jar_path = fs::read_dir(&target_dir)
        .unwrap()
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .find(|path| {
            path.is_file()
                && path
                .extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case("jar"))
        })
        .expect("No JAR file found in target directory");

    let mut cmd = Command::new("java");
    cmd.arg("-jar").arg(&jar_path);

    #[cfg(target_os = "windows")]
    {
        // 0x00000008 是 DETACHED_PROCESS 标志位
        // 它会让 Java 脱离 Rust 的无窗口环境，完全由 Windows 根据 Java 的类型来决定：
        cmd.creation_flags(0x00000008);
    }

    cmd.status().unwrap();
}