use std::env;
use std::fs;
use std::process::Command;

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

    println!("Launching {}", jar_path.display());

    Command::new("javaw")
        .arg("-jar")
        .arg(&jar_path)
        .status()
        .unwrap();
}