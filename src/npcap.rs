use std::error::Error;
use std::fs::File;
use std::process::Command;
use sha2::{Sha256, Digest};

// Check if npcap is installed
pub fn is_npcap_installed() -> bool {
    let output = Command::new("sc")
        .arg("query")
        .arg("npf")
        .output()
        .expect("failed to execute process");
    let output = String::from_utf8_lossy(&output.stdout);
    if output.contains("RUNNING") {
        return true;
    }
    false
}

// Download and Run npcap installer
pub fn install_npcap() -> Result<(), Box<dyn Error>> {
    let npcap_installer_filename = "npcap-1.76.exe";
    let hash = "3C846F5F62A217E3CF2052749CDE159E946248022781097C58815386DA6B9C46";
    let npcap_installer_url = "https://npcap.com/dist/".to_owned() + npcap_installer_filename;

    // Download npcap installer
    let mut response = reqwest::blocking::get(&npcap_installer_url)?;
    let mut file = File::create(npcap_installer_filename)?;
    response.copy_to(&mut file)?;

    // Checksum
    let mut file = File::open(npcap_installer_filename)?;
    let mut hasher = Sha256::new();
    std::io::copy(&mut file, &mut hasher)?;
    let hash_result = hasher.finalize();
    let hash_result = format!("{:X}", hash_result);

    // Close file
    drop(file);

    if hash_result != hash {
        // print downloaded file hash
        println!("Downloaded file hash: {}", hash_result);
        return Err("Error: checksum failed...".into());
    }

    // Run installer
    let mut child = Command::new(npcap_installer_filename)
        .arg("/loopback_support=yes")
        .arg("/winpcap_mode=yes")
        .spawn()?;
    let ecode = child.wait()?;
    if !ecode.success() {
        return Err("Error: Npcap installation failed !".into());
    }

    Ok(())
}
