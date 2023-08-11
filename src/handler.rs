use inquire::Confirm;

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
    println!("Downloading NetProbe...");
    // download netprobe
    let mut response: reqwest::blocking::Response = reqwest::blocking::get(define::NETPROBE_GUI_DIST_URL).unwrap();
    let mut file: std::fs::File = std::fs::File::create(define::NETPROBE_GUI_FILENAME).unwrap();
    response.copy_to(&mut file).unwrap();
    println!("NetProbe installed successfully !");
    // print install path
    println!("Install path: {}", get_install_path());
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
