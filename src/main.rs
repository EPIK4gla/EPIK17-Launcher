use std::fs::{self, File};
use std::path::PathBuf;
use std::process::{Command, Child};
use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};
use reqwest::blocking::get;
use zip::ZipArchive;
use winreg::enums::*;
use winreg::RegKey;
use urlencoding::decode;
use chrono::Utc;
use ctrlc;
use winapi::um::wincon::GetConsoleWindow;
use winapi::um::winuser::ShowWindow;

// i'll try to add a anti-cheat later
// anways, heres my TODO list:
// [x] auto updater
// [x] protocol handler
// [x] download client if not exists
// [x] download studio if not exists
// [x] launch client with args from protocol
// [-] discord rich presence -- broken for now
// [ ] anti-cheat
// [ ] better error handling
// [ ] GUI launcher
// Maybe a Linux version in the future?

const VERSION: &str = "1.0.9"; // i sometime forget to update this lmao
const EPIKVERSION: &str = "https://www.epik17.xyz/version";
const CZIPURL: &str = "https://www.epik17.xyz/EPIKPlayerBeta.zip";
const SZIPURL: &str = "https://www.epik17.xyz/EPIKStudioBeta.zip";
const EPIKLAUNCHER: &str = "https://www.epik17.xyz/EPIKLauncherBeta.exe";
// const DSCURL: &str = "https://www.epik17.xyz/dsc.exe";
// const DSC: &str = "1233051990582366240";

fn appdata() -> PathBuf {
    let mut p: PathBuf = env::var_os("APPDATA").unwrap().into();
    p.push("EPIK17");
    p
}

fn cdir() -> PathBuf {
    let mut p = appdata();
    p.push("Client");
    p
}

fn sdir() -> PathBuf {
    let mut p = appdata();
    p.push("Studio");
    p
}

fn cexe() -> PathBuf {
    let mut p = cdir();
    p.push("EPIKPlayerBeta.exe");
    p
}

// this function ensures the EPIKPlayerBeta.exe is still here
fn eepikexe() -> PathBuf {
    let mut target = cdir();
    target.push("EPIKPlayerBeta.exe");

    let source = cexe();

    if !target.exists() {
        fs::copy(&source, &target).unwrap();
    } else {
        let src_meta = fs::metadata(&source).unwrap();
        let tgt_meta = fs::metadata(&target).unwrap();
        if src_meta.len() != tgt_meta.len() {
            fs::copy(&source, &target).unwrap();
        }
    }

    target
}

fn lpath() -> PathBuf {
    let mut p = appdata();
    p.push("EPIKLauncherBeta.exe");
    p
}

// fn dsc_path() -> PathBuf {
//     let mut p = appdata();
//     p.push("dsc.exe");
//     p
// }

// kind of cache buster
fn cachelol(url: &str) -> String {
    let ts = Utc::now().timestamp();
    if url.contains('?') {
        format!("{}&_={}", url, ts)
    } else {
        format!("{}?_={}", url, ts)
    }
}

fn filesdownloader(url: &str, path: &PathBuf) {
    let mut r = get(url).unwrap();
    let mut f = File::create(path).unwrap();
    std::io::copy(&mut r, &mut f).unwrap();
}

// i don't really need to explain this one
fn unzip(path: &PathBuf, out: &PathBuf) {
    if out.exists() {
        fs::remove_dir_all(out).unwrap();
    }
    fs::create_dir_all(out).unwrap();
    let f = File::open(path).unwrap();
    let mut z = ZipArchive::new(f).unwrap();
    z.extract(out).unwrap();
    fs::remove_file(path).unwrap();
}

fn fuckoffprotocol() {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let _ = hkcu.delete_subkey_all(r"Software\Classes\epik17");
}

fn addprotocol() {
    let exe = lpath().to_str().unwrap().to_string();
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let k = hkcu.create_subkey(r"Software\Classes\epik17").unwrap().0;
    k.set_value("", &"URL:EPIK17").unwrap();
    k.set_value("URL Protocol", &"").unwrap();
    k.create_subkey("DefaultIcon").unwrap().0 // i have no idea what is this for
        .set_value("", &format!("\"{}\",0", exe)).unwrap();
    k.create_subkey(r"shell\open\command").unwrap().0
        .set_value("", &format!("\"{}\" \"%1\"", exe)).unwrap();
}

fn updatechecker() {
    let need = get(&cachelol(EPIKVERSION))
        .ok()
        .and_then(|r| r.text().ok())
        .map(|v| v.trim() != VERSION || !cexe().exists())
        .unwrap_or(true);

    if need {
        let zp = appdata().join("EPIKPlayerBeta.zip");
        filesdownloader(&cachelol(CZIPURL), &zp);
        unzip(&zp, &cdir());
        fuckoffprotocol();
        addprotocol();
    }

    if !sdir().exists() || !sdir().join("EPIKStudioBeta.exe").exists() {
        let zp = appdata().join("EPIKStudioBeta.zip");
        filesdownloader(&cachelol(SZIPURL), &zp);
        unzip(&zp, &sdir());
    }

    // discord rich presence client
    // let dp = dsc_path();
    // if !dp.exists() {
    //     filesdownloader(DSCURL, &dp);
    // }
}

fn epik17(url: &str) -> (String, HashMap<String, String>) {
    // we are gonna handle studio edits soon
    let u = url.replace("epik17:", "");
    let parts: Vec<&str> = u.split('+').collect();
    let mut map = HashMap::new();
    for p in &parts[1..] {
        if let Some(i) = p.find(':') {
            map.insert(p[..i].to_string(), decode(&p[i + 1..]).unwrap().to_string());
        }
    }
    (parts[0].to_string(), map)
}

fn lclient(p: &HashMap<String, String>) -> Child { // (Child, Child) {
    let ticket = p.get("ticket").cloned().unwrap_or_else(|| "whatyouwantthistobe".to_string());
    let gameid = p.get("gameid").cloned().unwrap_or_else(|| "1818".to_string());
    let base = "www.epik17.xyz";

    let game_exe = eepikexe();
    // let dsc_exe = dsc_path();

    // i need to see why this don't work...
    // i am tired of that dsc.exe..
    // let dsc_child = Command::new(&dsc_exe)
    //     .arg("-c").arg(DSC)
    //     .arg("-d").arg("Playing EPIK17")
    //     .arg("-s").arg("In Game")
    //     .arg("-N").arg("13b5bfbebee2d722a1f0d2af181ac561")
    //     .arg("-I").arg("EPIK17.xyz")
    //     .arg("-t")
    //     .spawn()
    //     .unwrap();

    // yeah well this works
    let game_child = Command::new(&game_exe)
        .arg("--play")
        .arg(format!("--authenticationUrl=https://{}/Login/Negotiate.ashx", base))
        .arg(format!("--authenticationTicket={}", ticket))
        .arg(format!(
            "--joinScriptUrl=https://{}/game/PlaceLauncher.ashx?placeId={}&t={}",
            base, gameid, ticket
        ))
        .spawn()
        .unwrap();

    game_child //(game_child, dsc_child)
}

fn gamelmao() {
    let _ = Command::new("cmd")
        .arg("/c")
        .arg("start")
        .arg("")
        .arg("https://www.epik17.xyz/games")
        .spawn();
}

fn main() {
    fs::create_dir_all(appdata()).unwrap();
    fs::create_dir_all(cdir()).unwrap();

    if !lpath().exists() || env::current_exe().unwrap() != lpath() {
        filesdownloader(&cachelol(EPIKLAUNCHER), &lpath());
        gamelmao();
    }

    updatechecker();
    addprotocol();

    if let Some(arg) = env::args().nth(1) {
        let (cmd, params) = epik17(&arg);
        if cmd == "play" {
            // even if playing, we ALWAYS need to check if theres a new update
            updatechecker();

            let game_child = lclient(&params); //let (mut game_child, mut dsc_child) = lclient(&params);

            unsafe {
                let h = GetConsoleWindow();
                if !h.is_null() {
                    ShowWindow(h, 0);
                }
            }

            let game_arc = Arc::new(Mutex::new(game_child));
            // let dsc_arc = Arc::new(Mutex::new(dsc_child));

            let game_clone = Arc::clone(&game_arc);
            // let dsc_clone = Arc::clone(&dsc_arc);

            ctrlc::set_handler(move || {
                let _ = game_clone.lock().unwrap().kill();
                // let _ = dsc_clone.lock().unwrap().kill();
                std::process::exit(0);
            }).unwrap();

            game_arc.lock().unwrap().wait().unwrap();
            // dsc_arc.lock().unwrap().kill().unwrap();
        }
    }
}
