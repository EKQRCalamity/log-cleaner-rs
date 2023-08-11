use std::process::{Command, exit};
use std::time::Duration;
use std::{io, thread};
use std::path::{Path, PathBuf};
use dirs::{config_dir, config_local_dir};
use std::fs;

pub fn read_input(prompt: &str) -> String {
    use std::io::Write;
    let mut buffer: String = String::new();
    print!("{} ", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_owned()
}

pub fn exit_in(seconds: i64) {
    use std::io::Write;
    print!("\rExiting in {} seconds", seconds);
    io::stdout().flush().unwrap();
    if seconds == 0 {
        std::process::exit(0);
    } else {
        thread::sleep(Duration::from_secs(1));
        exit_in(seconds - 1);
    }
}

fn main() {
    kill_riot_processes();
    clear_log();
    println!("Made by vanargand0002");
    exit_in(8);
}

fn clear_log() {
    let mut driveletter = get_drive_letter(r"Riot Games\Riot Client");
    if !driveletter.is_empty() {
        println!("Couldn't identify riot installation drive.");
        driveletter = read_input("Please input the drive letter/path to the riot folder (Example: A:\\):");
        if driveletter.is_empty() {
            exit(0);
        }
        if driveletter.ends_with("Riot Games") {
            driveletter = driveletter.strip_suffix("Riot Games").expect("Error removing substr").to_owned();
        }
        if driveletter.ends_with("Riot Games\\") {
            driveletter = driveletter.strip_suffix("Riot Games\\").expect("Error removing substr").to_owned();
        }
    } else {
        println!("Drive letter identified as {}", driveletter);
    }
    let mut windriveletter = get_drive_letter(r"ProgramData\Riot Games");
    if !driveletter.is_empty() {
        println!("Couldn't identify ProgramData folder.");
        windriveletter = read_input("Please input the drive letter/path to the ProgramData folder (Example: A:\\):");
        if windriveletter.is_empty() {
            exit(0);
        }
        if windriveletter.ends_with("ProgramData") {
            windriveletter = windriveletter.strip_suffix("ProgramData").expect("Error removing substr").to_owned();
        }
        if windriveletter.ends_with("ProgramData\\") {
            windriveletter = windriveletter.strip_suffix("ProgramData\\").expect("Error removing substr").to_owned();
        }
    } else {
        println!("Windows drive letter identified as {}", windriveletter);
    }
    let paths_to_delete = vec![
        format!("{}ProgramData\\Riot Games\\", windriveletter),
        format!("{}ProgramData\\Riot Games\\machine.cfg", windriveletter),
        format!("{}Riot Games\\League of Legends\\Config", driveletter),
        format!("{}Riot Games\\League of Legends\\Logs", driveletter),
        format!("{}Riot Games\\League of Legends\\debug.log", driveletter),
        format!("{}Riot Games\\Riot Client\\UX\\natives_blob.bin", driveletter),
        format!("{}Riot Games\\Riot Client\\UX\\snapshot_blob.bin", driveletter),
        format!("{}Riot Games\\Riot Client\\UX\\v8_context_snapshot.bin", driveletter),
        format!("{}Riot Games\\Riot Client\\UX\\icudtl.dat", driveletter),
        format!("{}Riot Games\\Riot Client\\UX\\GPUCache\\", driveletter),
    ];

    for path in paths_to_delete {
        if Path::new(path.as_str()).exists() {
            match delete_path(Path::new(path.as_str())) {
                Ok(_) => println!("Deleted: {}", path),
                Err(error) => eprintln!("Error while deleting {} - {}", path, error),
            }
        } else {
            println!("{} not found", (if path.split("\\").last().unwrap().is_empty() { format!("{}ProgramData\\Riot Games", driveletter).to_string() } else { path.split("\\").last().unwrap().to_string() } ));
        }
    }

    // Delete the Riot Games appdata folder
    if let Some(appdata_dir) = get_riot_appdata_folder() {
        match delete_path(&appdata_dir) {
            Ok(_) => println!("Deleted appdata folder: {:?}", appdata_dir),
            Err(error) => eprintln!("Error deleting appdata folder: {}", error),
        }
        
    } else {
        println!("Riot Games appdata folder not found.");
    }
}

fn delete_path(path: &Path) -> io::Result<()> {
    if path.is_dir() {
        // Delete directories with all their contents
        fs::remove_dir_all(path)?;
    } else if path.is_file() {
        // Delete individual files
        fs::remove_file(path)?;
    }
    Ok(())
}

fn get_riot_appdata_folder() -> Option<PathBuf> {
    // On Windows, the Riot Games appdata folder is typically in the LocalAppData directory
    // Example: C:\Users\{Username}\AppData\Local\Riot Games\
    if let Some(local_appdata) = config_dir() {
        let riot_appdata_dir = local_appdata.join("Riot Games");
        if riot_appdata_dir.exists() {
            return Some(riot_appdata_dir);
        }
    }
    if let Some(local_appdata) = config_local_dir() {
        let riot_appdata_dir = local_appdata.join("Riot Games");
        if riot_appdata_dir.exists() {
            return Some(riot_appdata_dir);
        }
    }
    None
}


fn get_drive_letter(file_path: &str) -> String {
    let drives = get_available_drives();
    let drive_l: String = "".to_string();
    for drive in drives {
        let full_path = format!("{}{}", drive, file_path);
        let path = Path::new(&full_path);

        if path.exists() {
            return drive.to_owned();
        }
    }
    return drive_l;
}

fn get_available_drives() -> Vec<String> {
    // On Windows, drives are usually represented by single uppercase letters (e.g., C:, D:, etc.)
    // We'll iterate through each letter to check for the existence of the drive.
    let drives = (b'A'..=b'Z')
        .map(|letter| {
            let drive_str = format!("{}:\\", letter as char);
            PathBuf::from(drive_str)
        })
        .filter(|drive| fs::metadata(drive).is_ok()) // Filter out drives that don't exist
        .collect::<Vec<_>>();

    // Convert PathBufs to strings
    drives.iter()
        .map(|drive| drive.to_string_lossy().to_string())
        .collect()
}

fn kill_riot_processes() {
    let processes_to_kill = vec![
        "RiotClientUx.exe",
        "RiotClientUxRender.exe",
        "RiotClientCrashHandler.exe",
        "RiotClientServices.exe",
        "LeagueClient.exe",
        "LeagueClientUx.exe",
        "LeagueClientUxRender.exe",
        "riot*",
        "league*",
    ];

    for process_name in processes_to_kill {
        if let Err(err) = kill_process_by_name(process_name) {
            eprintln!("Error while killing {}: {}", process_name, err);
        } else {
            println!("Killed {}", process_name);
        }
    }
}

fn kill_process_by_name(process_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Use taskkill to terminate the process by name
    Command::new("taskkill")
        .args(&["/F", "/IM", process_name])
        .output()?;

    Ok(())
}
