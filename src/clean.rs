use crate::data::*;
use std::fs::*;

pub fn clean(config: Toml) -> Result<(), String> {
    const LEN: usize = 3;
    let set: [(String, String); LEN] = [
        ("LINK".to_string(), "output".to_string()),
        ("PACKAGE".to_string(), "dest".to_string()),
        ("CROSS".to_string(), "output".to_string())
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
            None => {}
        }
    }

    if !is_cleaned {
        println!("Nothing was cleaned.");
    }

    Ok(())
}