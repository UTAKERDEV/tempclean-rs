// Import the necessary modules
use std::env;
use std::fs;
use std::path::Path;
use std::io::{self, Write};

// Get the size of a folder and its content
fn get_folder_size(path: &str) -> u64 {
    let mut size = 0;
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let entry_path = entry.path();
            if entry_path.is_file() {
                if let Ok(metadata) = fs::metadata(&entry_path) {
                    size += metadata.len();
                }
            } else if entry_path.is_dir() {
                size += get_folder_size(entry_path.to_str().unwrap());
            }
        }
    }
    size
}

// Clean a folder by deleting all its content
fn clean_folder(path: &str) -> u64 {
    let folder_path = Path::new(path);
    let before_size = get_folder_size(path);

    // Clean the folder
    if folder_path.exists() {
        match fs::read_dir(folder_path) {
            Ok(entries) => {
                for entry in entries.flatten() {
                    let entry_path = entry.path();
                    if entry_path.is_file() {
                        fs::remove_file(&entry_path).ok();
                    } else if entry_path.is_dir() {
                        fs::remove_dir_all(&entry_path).ok();
                    }
                }
                // Check the size after cleaning
                let after_size = get_folder_size(path);
                let freed_space = before_size.saturating_sub(after_size);
                println!("✅ Folder cleaned : {} (Space gained : {:.2} MB)", path, freed_space as f64 / 1_048_576.0);
                return freed_space;
            }
            Err(err) => eprintln!("❌ Error durin folder reading {} : {}", path, err),
        }
    } else {
        eprintln!("⚠️ The folder {} is non-existent", path);
    }
    0
}

fn main() {
    let mut total_freed = 0;

    // Gather windows username
    if let Ok(user_name) = env::var("USERNAME") {
        let user_temp = format!("C:\\Users\\{}\\AppData\\Local\\Temp", user_name);
        total_freed += clean_folder(&user_temp);
    } else {
        eprintln!("❌ Impossible to gather the username.");
    }

    // Clean Prefetch folder (need admin rights)
    total_freed += clean_folder("C:\\Windows\\Prefetch");

    // Clean Windows\Temp folder
    total_freed += clean_folder("C:\\Windows\\Temp");

    println!("\n🎉 Clean finished ! Total space freed : {:.2} MB", total_freed as f64 / 1_048_576.0);

    // Wait for user to press a key before closing
    println!("\nPress Enter to exit...");
    io::stdout().flush().unwrap();
    let mut _input = String::new();
    io::stdin().read_line(&mut _input).unwrap();
}
