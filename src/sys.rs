use crate::models::AppInfo;
use std::collections::HashMap;
use winreg::enums::HKEY_LOCAL_MACHINE;
use winreg::RegKey;

pub fn get_os_bit() -> String {
    if cfg!(target_pointer_width = "32") {
        return "32-bit".to_owned();
    } else if cfg!(target_pointer_width = "64") {
        return "64-bit".to_owned();
    } else {
        return "unknown".to_owned();
    }
}

// Get npcap installation status
// Return: (installed, version)
pub fn get_npcap_status() -> (bool, String) {
    let hklm: RegKey = RegKey::predef(HKEY_LOCAL_MACHINE);
    let os_bit: String = get_os_bit();
    let npcap_key: RegKey = 
    if os_bit == "32-bit" {
        match hklm.open_subkey("SOFTWARE\\Npcap") {
            Ok(key) => key,
            Err(_) => return (false, String::new()),
        }
    }else{
        match hklm.open_subkey("SOFTWARE\\WOW6432Node\\Npcap") {
            Ok(key) => key,
            Err(_) => return (false, String::new()),
        }
    };
    let version: String = npcap_key.get_value("").unwrap_or(String::new());
    return (true, version);
}

#[allow(dead_code)]
pub fn get_installed_apps() {
    let hklm: RegKey = winreg::RegKey::predef(HKEY_LOCAL_MACHINE);
    let uninstall_key: RegKey = hklm
        .open_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall")
        .expect("key is missing");

    let mut apps: HashMap<String, AppInfo> = HashMap::new();
    for key in uninstall_key.enum_keys() {
        let key = match key {
            Ok(key) => key,
            Err(_) => continue,
        };
        //let key = key.unwrap();
        let subkey: RegKey = uninstall_key
            .open_subkey(key.clone())
            .expect("key is missing");
        let app: AppInfo = AppInfo {
            display_name: subkey.get_value("DisplayName").unwrap_or(String::new()),
            display_version: subkey.get_value("DisplayVersion").unwrap_or(String::new()),
            uninstall_string: subkey.get_value("UninstallString").unwrap_or(String::new()),
        };
        apps.insert(key, app);
    }
    for v in apps.values() {
        println!("{:?}", v);
    }
}
