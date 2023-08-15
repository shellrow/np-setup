use inquire::Confirm;
use winreg::enums::RegDisposition;
use std::process::{ExitStatus, Command};

use crate::npcap;
use crate::define;

pub fn check_dependencies() -> bool {
    // check if npcap is installed
    if !npcap::is_npcap_installed() {
        let ans: bool = Confirm::new("Npcap is not installed, would you like to install it ?")
        .prompt()
        .unwrap();
        if ans == false {
            println!("Exiting...");
            return false;
        }
        println!("Installing Npcap...");
        match npcap::install_npcap() {
            Ok(_) => println!("Npcap installed successfully !"),
            Err(e) => println!("{}", e),
        }
    } else {
        println!("Npcap is already installed !");
    }
    true
}

pub fn get_install_path() -> String {
    match home::home_dir() {
        Some(path) => {
            let path: String = format!("{}\\{}", path.display(), define::NETPROBE_INSTALL_DIR_NAME);
            path
        },
        None => {
            String::new()
        },
    }
}

pub fn check_cli_installed() -> bool {
    let install_path: &std::path::Path = &std::path::Path::new(&get_install_path()).join(define::NETPROBE_CLI_FILENAME.replace(".zip", ""));
    if !install_path.exists() {
        return false;
    }
    let install_path = &std::path::Path::new(&get_install_path()).join("bin").join("np.exe");
    install_path.exists()
}

pub fn check_env_path() -> bool {
    let reg_key: winreg::RegKey = winreg::RegKey::predef(winreg::enums::HKEY_CURRENT_USER)
        .open_subkey_with_flags("Environment", winreg::enums::KEY_READ)
        .unwrap();
    let reg_value: String = reg_key.get_value("Path").unwrap();
    reg_value.contains(".netprobe")
}

pub fn install_netprobe() {
    println!("Installing NetProbe...");
    println!("Downloading NetProbe Installer...");
    // Download netprobe installer if not exists
    if !std::path::Path::new(define::NETPROBE_GUI_FILENAME).exists() {
        let mut response: reqwest::blocking::Response = reqwest::blocking::get(define::NETPROBE_GUI_DIST_URL).unwrap();
        let mut file: std::fs::File = std::fs::File::create(define::NETPROBE_GUI_FILENAME).unwrap();
        response.copy_to(&mut file).unwrap();
        println!("Waiting for virus scan to complete (10 seconds) ...");
        std::thread::sleep(std::time::Duration::from_secs(10));
    }
    // Run installer
    println!("Running NetProbe Installer...");
    let exit_status: ExitStatus = 
        match Command::new(define::NETPROBE_GUI_FILENAME).spawn() {
            Ok(mut child) => {
                child.wait().unwrap()
            },
            Err(e) => {
                println!("Error: {}", e);
                return;
            },
        };
    if !exit_status.success() {
        println!("Error: NetProbe installation failed or cancelled !");
        return;
    }
    println!("NetProbe installed successfully !");
}

pub fn install_netprobe_cli() {
    if check_cli_installed() {
        println!("NetProbe CLI is already installed !");
        return;
    }
    println!("Installing NetProbe CLI...");
    println!("Downloading NetProbe CLI package...");
    // Download netprobe cli
    let mut response: reqwest::blocking::Response = reqwest::blocking::get(define::NETPROBE_CLI_DIST_URL).unwrap();
    let mut file: std::fs::File = std::fs::File::create(define::NETPROBE_CLI_FILENAME).unwrap();
    response.copy_to(&mut file).unwrap();
    // unzip package
    println!("Unzipping NetProbe CLI package...");
    let mut archive: zip::ZipArchive<std::fs::File> = zip::ZipArchive::new(std::fs::File::open(define::NETPROBE_CLI_FILENAME).unwrap()).unwrap();
    archive.extract(get_install_path()).unwrap();
    // Package path
    let installed_package_path: String = format!("{}\\{}", get_install_path(), define::NETPROBE_CLI_FILENAME.replace(".zip", ""));
    // Move np.exe to bin folder
    println!("Moving NetProbe CLI executable to bin folder...");
    std::fs::rename(
        std::path::Path::new(&installed_package_path).join("np.exe"),
        std::path::Path::new(&get_install_path()).join("bin").join("np.exe"),
    ).unwrap();
    // Add netprobe cli to path
    if !check_env_path() {
        println!("Adding NetProbe CLI to path...");
        // Add netprobe cli bin dir to user environment variables Path (using winreg)
        let hkcu: winreg::RegKey = winreg::RegKey::predef(winreg::enums::HKEY_CURRENT_USER);
        let (path, _): (winreg::RegKey, RegDisposition) = hkcu.create_subkey("Environment").unwrap();
        let mut path_value: String = path.get_value::<String, &str>("Path").unwrap();
        path_value.push(';');
        path_value.push_str(&std::path::Path::new(&get_install_path()).join("bin").to_str().unwrap());
        println!("{}", path_value);
        path.set_value("Path", &path_value).unwrap();
        // Add netprobe cli bin dir to system environment variables Path (using winreg)
        /* let hklm: winreg::RegKey = winreg::RegKey::predef(winreg::enums::HKEY_LOCAL_MACHINE);
        let (path, _): (winreg::RegKey, RegDisposition) = hklm.create_subkey("SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment").unwrap();
        let mut path_value: String = path.get_value::<String, &str>("Path").unwrap();
        path_value.push(';');
        path_value.push_str(&std::path::Path::new(&get_install_path()).join("bin").to_str().unwrap());
        path.set_value("Path", &path_value).unwrap(); */
    }
    // remove zip file
    println!("Removing NetProbe CLI package...");
    std::fs::remove_file(define::NETPROBE_CLI_FILENAME).unwrap();
    // print success message
    println!("NetProbe CLI installed successfully !");
    // print install path
    println!("Install path: {}", get_install_path());
}
