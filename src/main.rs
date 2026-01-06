// made by LVTKR - epik17.xyz
// =============================
use std::fs::{self, File};
use std::path::PathBuf;
use std::process::Command;
use std::collections::HashMap;
use std::env;
use reqwest::blocking::get;
use zip::ZipArchive;
use winreg::enums::*;
use winreg::RegKey;
use urlencoding::decode;
use chrono::Utc;

const VERSION: &str = "1.0.5";
const EPIKVERSION: &str = "https://www.epik17.xyz/version";
const CZIPURL: &str = "https://www.epik17.xyz/latest.zip";
const EPIKLAUNCHER: &str = "https://www.epik17.xyz/EPIKLauncherBeta.exe";

// poulet u cant say this is a rat...
//its litterally open sourced now..

fn appdata() -> PathBuf {
    let mut path: PathBuf = env::var_os("APPDATA").unwrap().into();
    path.push("EPIK17");
    path
}

fn cdir() -> PathBuf {
    let mut p = appdata();
    p.push("Client");
    p
}

fn cexe() -> PathBuf {
    let mut p = cdir();
    p.push("EPIKPlayerBeta.exe");
    p
}

fn lpath() -> PathBuf {
    let mut p = appdata();
    p.push("EPIKLauncherBeta.exe");
    p
}

fn cachelol(url: &str) -> String {
    let timestamp = Utc::now().timestamp();
    if url.contains('?') {
        format!("{}&_={}", url, timestamp)
    } else {
        format!("{}?_={}", url, timestamp)
    }
}

fn filesdownloader(url: &str, path: &PathBuf) {
    let mut resp = get(url).unwrap();
    let mut out = File::create(path).unwrap();
    std::io::copy(&mut resp, &mut out).unwrap();
}

fn unzip(path: &PathBuf, out_dir: &PathBuf) {
    if out_dir.exists() {
        fs::remove_dir_all(out_dir).unwrap();
    }
    fs::create_dir_all(out_dir).unwrap();
    let file = File::open(path).unwrap();
    let mut archive = ZipArchive::new(file).unwrap();
    archive.extract(out_dir).unwrap();
    fs::remove_file(path).unwrap();
}

fn fuckoffprotocol() {
    // fuck u
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let _ = hkcu.delete_subkey_all(r"Software\Classes\epik17");
}

fn addprotocol() {
    let exe_path_buf = lpath();
    let exe_path_string = exe_path_buf.to_str().unwrap().to_string();
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let key = hkcu.create_subkey(r"Software\Classes\epik17").unwrap().0;
    key.set_value("", &"URL: EPIK17 Protocol").unwrap();
    key.set_value("URL Protocol", &"").unwrap();
    let icon_key = key.create_subkey("DefaultIcon").unwrap().0;
    icon_key.set_value("", &format!("\"{}\",0", exe_path_string)).unwrap();
    let command_key = key.create_subkey(r"shell\open\command").unwrap().0;
    command_key.set_value("", &format!("\"{}\" \"%1\"", exe_path_string)).unwrap();
}

fn updatechecker() -> bool {
    let mut update_needed = false;
    let resp = get(&cachelol(EPIKVERSION));
    if let Ok(r) = resp {
        if let Ok(latest) = r.text() {
            if latest.trim() != VERSION || !cexe().exists() {
                update_needed = true;
            }
        } else {
            update_needed = true;
        }
    } else {
        update_needed = true;
    }
    if update_needed {
        let zip_path = appdata().join("latest.zip");
        filesdownloader(&cachelol(CZIPURL), &zip_path);
        unzip(&zip_path, &cdir());
        fuckoffprotocol();
        addprotocol();
        lclient(&HashMap::new());    
    }
    update_needed
}

//ts detect if its epik17 protocol and parse it
fn epik17(url: &str) -> (String, HashMap<String, String>) {
    let url = url.replace("epik17:", "");
    let parts: Vec<&str> = url.split('+').collect();
    let command = parts[0].to_string();
    let mut params = HashMap::new();
    for p in &parts[1..] {
        if let Some(idx) = p.find(':') {
            let key = &p[..idx];
            let val = &p[idx+1..];
            params.insert(key.to_string(), decode(val).unwrap().to_string());
        }
    }
    (command, params)
}

fn lclient(params: &HashMap<String, String>) {
    let default_ticket = "dummy_ticket".to_string();
    let default_gameid = "1".to_string();
    let ticket = params.get("ticket").unwrap_or(&default_ticket);
    let gameid = params.get("gameid").unwrap_or(&default_gameid);
    let base_url = "www.epik17.xyz";
    let join_url = format!("https://{}/game/PlaceLauncher.ashx?placeId={}&t={}", base_url, gameid, ticket);
    let _ = Command::new(cexe())
        .arg("--play")
        .arg(format!("--authenticationUrl=https://{}/Login/Negotiate.ashx", base_url))
        .arg(format!("--authenticationTicket={}", ticket))
        .arg(format!("--joinScriptUrl={}", join_url))
        .spawn();
}

// yes no wimpy gamelmao
fn gamelmao() {
    let _ = Command::new("cmd").arg("/c").arg("start").arg("").arg("https://www.epik17.xyz/games").spawn();
}

fn main() {
    let appdata = appdata();
    let content = cdir();
    fs::create_dir_all(&appdata).unwrap();
    fs::create_dir_all(&content).unwrap();
    if !lpath().exists() || env::current_exe().unwrap() != lpath() {
        println!("Downloading...");
        filesdownloader(&cachelol(EPIKLAUNCHER), &lpath());
        gamelmao();
    }
    let updated = updatechecker();
    if !updated {
        addprotocol();
    }
    if let Some(arg) = env::args().nth(1) {
        let (cmd, params) = epik17(&arg);
        if cmd == "play" {
            println!("Launching game ID {}...", params.get("gameid").unwrap_or(&"1".to_string()));
            lclient(&params);
        }
    }
}
