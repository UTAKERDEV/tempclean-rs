use std::env;
use std::fs;
use std::path::Path;
use std::io::{self, Write};

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

fn clean_folder(path: &str) -> u64 {
    let folder_path = Path::new(path);
    let before_size = get_folder_size(path);

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

    // Récupérer le nom de l'utilisateur Windows
    if let Ok(user_name) = env::var("USERNAME") {
        let user_temp = format!("C:\\Users\\{}\\AppData\\Local\\Temp", user_name);
        total_freed += clean_folder(&user_temp);
    } else {
        eprintln!("❌ Impossible to gather the username.");
    }

    // Nettoyer le dossier Prefetch (nécessite des droits admin)
    total_freed += clean_folder("C:\\Windows\\Prefetch");

    // Nettoyer le dossier Windows\Temp
    total_freed += clean_folder("C:\\Windows\\Temp");

    println!("\n🎉 Clean finished ! Total space freed : {:.2} MB", total_freed as f64 / 1_048_576.0);

    // Attendre que l'utilisateur appuie sur une touche avant de fermer
    println!("\nPress Enter to exit...");
    io::stdout().flush().unwrap();
    let mut _input = String::new();
    io::stdin().read_line(&mut _input).unwrap();
}
