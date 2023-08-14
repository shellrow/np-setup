use inquire::Confirm;
use std::process::{ExitStatus, Command};

use crate::npcap;
use crate::define;

// Only for windows platform
#[cfg(target_os = "windows")]
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

// Other platforms
#[cfg(not(target_os = "windows"))]
fn check_dependencies() -> bool {
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
    println!("Installing NetProbe CLI...");
    println!("Downloading NetProbe CLI...");
    // download netprobe cli
    let mut response: reqwest::blocking::Response = reqwest::blocking::get(define::NETPROBE_CLI_DIST_URL).unwrap();
    let mut file: std::fs::File = std::fs::File::create(define::NETPROBE_CLI_FILENAME).unwrap();
    response.copy_to(&mut file).unwrap();
    println!("NetProbe CLI installed successfully !");
    // print install path
    println!("Install path: {}", get_install_path());
}
