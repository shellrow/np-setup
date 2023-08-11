mod define;
mod models;
mod sys;
mod npcap;
mod handler;

use std::fmt::{Display, Formatter};
use inquire::Select;

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
    println!("GUI: {}", define::NETPROBE_GUI_URL);
    println!("CLI: {}", define::NETPROBE_CLI_URL);
}

fn main() {
    show_banner();
    println!();
    let selected_menu: Menu = Select::new("Select options: ", Menu::VARIANTS.to_vec()).prompt().unwrap();
    match selected_menu {
        Menu::InstallNetProbe | Menu::InstallNetProbeCli | Menu::Update => {
            // for Windows: Check dependencies
            println!("Checking dependencies...");
            if handler::check_dependencies() {
                //println!("Dependencies already installed !");
            } else {
                println!("Failed to resolve dependencies. exiting...");
                return;
            }
            match selected_menu {
                Menu::InstallNetProbe => {
                    println!("Install NetProbe");
                    handler::install_netprobe();
                },
                Menu::InstallNetProbeCli => {
                    println!("Install NetProbe CLI");
                    handler::install_netprobe_cli();
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


