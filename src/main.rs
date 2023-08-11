mod define;
mod models;
mod sys;
mod npcap;

use std::fmt::{Display, Formatter};
use inquire::{Confirm, Select};

#[derive(Debug, Copy, Clone)]
enum Menu {
    InstallNetProbe,
    InstallNetProbeCli,
    UninstallNetProbe,
    UninstallNetProbeCli,
    Update,
    Exit,
}

impl Menu {
    const VARIANTS: &'static [Menu] = &[
        Self::InstallNetProbe,
        Self::InstallNetProbeCli,
        Self::UninstallNetProbe,
        Self::UninstallNetProbeCli,
        Self::Update,
        Self::Exit,
    ];
}

impl Display for Menu {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::InstallNetProbe => write!(f, "Install NetProbe"),
            Self::InstallNetProbeCli => write!(f, "Install NetProbe CLI (np)"),
            Self::UninstallNetProbe => write!(f, "Uninstall NetProbe"),
            Self::UninstallNetProbeCli => write!(f, "Uninstall NetProbe CLI (np)"),
            Self::Update => write!(f, "Check Update"),
            Self::Exit => write!(f, "Exit"),
        }
        //write!(f, "{self:?}")
    }
}

pub fn show_banner() {
    println!("{} {}", define::APP_NAME, define::APP_VERSION);
    println!("{}", define::APP_DESCRIPTION);
    println!("GUI: {}", define::APP_NETPROBE_GUI_URL);
    println!("CLI: {}", define::APP_NETPROBE_CLI_URL);
}

fn main() {
    show_banner();
    println!();
    let selected_menu: Menu = Select::new("Select options: ", Menu::VARIANTS.to_vec()).prompt().unwrap();
    match selected_menu {
        Menu::InstallNetProbe | Menu::InstallNetProbeCli | Menu::Update => {
            // for Windows: Check dependencies
            println!("Checking dependencies...");
            if check_dependencies() {
                //println!("Dependencies already installed !");
            } else {
                println!("Failed to resolve dependencies. exiting...");
                return;
            }
            match selected_menu {
                Menu::InstallNetProbe => {
                    println!("Install NetProbe");
                    // TODO: install netprobe
                },
                Menu::InstallNetProbeCli => {
                    println!("Install NetProbe CLI");
                    // TODO: install netprobe-cli
                },
                Menu::Update => {
                    println!("Update");
                    // TODO: check update
                },
                _ => unreachable!(),
            }
        },
        Menu::UninstallNetProbe => {
            println!("Uninstall NetProbe");
        },
        Menu::UninstallNetProbeCli => {
            println!("Uninstall NetProbe CLI");
        },
        Menu::Exit => {
            println!("exiting...");
            return;
        },
    }

    println!("Press enter to exit...");
    let _ = std::io::stdin().read_line(&mut String::new());
    
}

// Only for windows platform
#[cfg(target_os = "windows")]
fn check_dependencies() -> bool {
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
