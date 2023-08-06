mod models;
mod sys;
mod npcap;

fn main() {    
    println!("Netprobe setup tool");
    println!("Press enter to continue...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    if input.trim() != "" {
        println!("Error: invalid input. exiting...");
        return;
    }
    println!("Checking dependencies...");
    if !npcap::is_npcap_installed() {
        println!("Npcap is not installed, would you like to install it ? (y/n)");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim() != "y" {
            println!("Exiting...");
            return;
        }
        println!("Installing Npcap...");
        match npcap::install_npcap() {
            Ok(_) => println!("Npcap installed successfully !"),
            Err(e) => println!("{}", e),
        }
    } else {
        println!("Npcap is already installed !");
    }
    // TODO: check if netprobe is installed
    // TODO: install netprobe
    // Print exit message
    println!("Press any key to exit...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}
