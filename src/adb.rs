use std::{fs, io, path::PathBuf, process::Command};

use crate::utils;

// declaration of commands
const SHOW_INFO: &str = "showinfo";

/**
 * ADB basic commands
*/
const START_SERVER: &str = "start";
const STOP_SERVER: &str = "stop";
const LIST_DEVICES: &str = "ls";
const SHOW_IP: &str = "ip";
const CONNECT_WITH_WIFI: &str = "connect";
const LOGCAT: &str = "logcat";

/**
 * adb install and set-up commands
 */
const DOWNLOAD_TOOLS: &str = "download";
const SET_PATH: &str = "setpath";

/**
 * adb phone interaction related commands
*/
const SCREEN_CAPTURE: &str = "capture";
const SCREEN_RECORD: &str = "record";
const DUMP: &str = "dump";
const LIST_PACKAGE: &str = "listpackage";
const PULL_APK: &str = "pull";
const INSTALL_APK: &str = "install";

/**
 * adb exit commands
 */
const EXIT: &str = "exit";

fn show_info() {
    println!(
        "{:015}: to show the all commands and their used.",
        SHOW_INFO
    );
    println!();

    // description printing for basic adb commands
    println!(
        "{:015}: to start adb server. This is equivalent to adb start-server.",
        START_SERVER
    );
    println!(
        "{:015}: to stop adb server. This is equivalent to adb kill-server.",
        STOP_SERVER
    );
    println!(
        "{:015}: to show the list of connected devices. This is equivalent to adb devices.",
        LIST_DEVICES
    );
    println!("{:015}: to show ip of connected device.", SHOW_IP);
    println!("{:015}: to connect the device with wifi.\n\te.g. {} IP:PORT.\n\tYou can get the IP of phone in Settings > Wifi Settings > Advance > IP Address.\n\tOr Search IP address in Setting. Or Use ip command.\n\tMake sure phone is connect through USB.",CONNECT_WITH_WIFI, CONNECT_WITH_WIFI);
    println!("{:015}: to record logcat in file, after this command add file name where logcat should be stored.\n\teg. logcat demo.txt.", LOGCAT);
    println!();
    // description printing of install and setup commands
    println!(
        "{:015}: to download android command line tools and platform tools in home dir.",
        DOWNLOAD_TOOLS
    );
    println!(
        "{:015}: to set the downloaded tools enviromental path.",
        SET_PATH
    );
    println!();
    // description printing of phone interaction related commands
    println!(
        "{:015}: to capture the screen of connected devices.\n\teg. {} filename.png.",
        SCREEN_CAPTURE, SCREEN_CAPTURE
    );
    println!(
        "{:015}: to record the screen of connected devices.\n\teg. {} filename.mp4.",
        SCREEN_RECORD, SCREEN_RECORD
    );
    println!(
        "{:015}: to get dump of system or application.\n\teg. {} packagename.",
        DUMP, DUMP
    );
    println!(
        "{:015}: to print all installed applications packagename.",
        LIST_PACKAGE
    );
    println!(
        "{:015}: to pull the installed application from phone to computer.\n\teg. {} packagename.",
        PULL_APK, PULL_APK
    );
    println!(
        "{:015}: to install apk in connected phone.\n\teg. {} location/of/your/apk.",
        INSTALL_APK, INSTALL_APK
    );
    println!();
    // Exit command
    println!("{}: to close radb app.", EXIT);
    println!("\n");
}

fn get_cmd() -> Option<Vec<String>> {
    let mut input_string = String::new();
    let read = std::io::stdin().read_line(&mut input_string).ok();
    if let Some(_) = read {
        // removing spacing chars
        input_string = input_string.trim().to_owned();
        // removing new line \n chars
        if input_string.ends_with('\n') {
            input_string.pop();
        }
        // converting string into vec string
        let result: Vec<String> = input_string.split(" ").map(|p| p.to_owned()).collect();
        return Some(result);
    }
    return None;
}

pub fn run_adb() {
    println!("============radb started=============");
    println!();
    show_info();
    set_adb_path();
    loop {
        println!("Enter the command$");
        let cmd = get_cmd();
        match cmd {
            Some(cmd) => {
                if cmd.len() == 0 || cmd[0].len() == 0 {
                    println!("No command found. Enter the command.");
                    continue;
                }
                if cmd[0] == "exit" {
                    println!("==========Bye bye=========");
                    break;
                }
                exe_cmd(cmd);
            }
            None => {
                println!("Some error occured!!!, no command found.");
            }
        }
    }
}

fn exe_cmd(cmd: Vec<String>) {
    let c: &str = cmd[0].as_str();
    if c == DOWNLOAD_TOOLS {
        download_adb_tool();
    } else if c == SET_PATH {
        set_adb_path();
    } else if c == START_SERVER {
        utils::run_adb_cmd(vec!["start-server".to_owned()]);
    } else if c == STOP_SERVER {
        utils::run_adb_cmd(vec!["kill-server".to_owned()]);
    }
}

fn download_adb_tool() {
    // check if adb_cmdtools exits, if it exits then do nothing.
    let adb_path = check_or_create_tools_dir();
    if let Some(path) = adb_path {
        // changing current working dir
        let result = std::env::set_current_dir(&path);
        if let Err(err) = result {
            println!("{}", err);
            return;
        }
        // clone the zip files from github.
        let result = clone_adb_files();
        if let Err(err) = result {
            println!("{}", err);
            return;
        }
        // extract and unzip the files.
        let result = extracting_adb_files(&path);
        if let Err(err) = result {
            println!("{}", err);
            return;
        }
        // at last set the path
        set_adb_path();
    }
}

fn check_or_create_tools_dir() -> Option<PathBuf> {
    let home_path = dirs::home_dir();
    if let None = home_path {
        println!("Could not find home dir.");
        return None;
    }
    let mut home_path = home_path.unwrap();
    // create new adb_cmdtools dir.
    home_path = home_path.join("adb_cmdtools");
    if home_path.exists() {
        // in case if adb tools dir already exists, then ask user wheather to overwrite or not.
        // get input from console, if user enters y?Y then delete the existing dirs
        // rest proceeds same.
        println!("adb tools already exits. Do you want to overwrite?. Enter (y/Y).");
        let c = get_cmd();
        if let None = c {
            return None;
        }
        let op = c.unwrap();
        if op.len() > 0 || op[0].to_lowercase() == "y" {
            println!("deleting existing setup.");
            let result = std::fs::remove_dir_all(&home_path);
            if let Err(e) = result {
                println!("{}", e);
                return None;
            }
        } else {
            // return in else case
            return None;
        }
    }
    let result = std::fs::create_dir(&home_path);
    // if can not create path then print error and return
    if let Err(e) = result {
        println!("{}", e);
        return None;
    }
    return Some(home_path);
}

const ZIPPED_ADB_URL: &str = "https://github.com/digvijaysingh13/adb-cmdtools-linux.git";

fn clone_adb_files() -> Result<(), io::Error> {
    println!("Cloning the tools...");
    let mut cmd = Command::new("git");
    cmd.arg("clone");
    cmd.arg(ZIPPED_ADB_URL);
    cmd.output()?;
    return Ok(());
}

fn extracting_adb_files(path: &PathBuf) -> Result<(), io::Error> {
    println!("unzipping...");
    // moving internal files one level up
    let working_path = path.clone().into_os_string().into_string().unwrap();
    // moving platform tools to one level
    let platform_path = path
        .join("adb-cmdtools-linux")
        .join("platform-tools.zip")
        .into_os_string()
        .into_string()
        .unwrap();
    let mut platform_cmd = Command::new("mv");
    platform_cmd.arg(platform_path);
    platform_cmd.arg(&working_path);
    platform_cmd.output()?;
    // moving command line tools to one level
    let cmdtools_path = path
        .join("adb-cmdtools-linux")
        .join("cmdline-tools.zip")
        .into_os_string()
        .into_string()
        .unwrap();
    let mut cmd = Command::new("mv");
    cmd.arg(cmdtools_path);
    cmd.arg(&working_path);
    cmd.output()?;
    // remove the cloned dir with readme file
    fs::remove_dir_all(path.join("adb-cmdtools-linux"))?;
    // unzip the cmdline-tools and platform-tools
    // unziping cmdline-tools
    let mut cmd = Command::new("unzip");
    cmd.arg("cmdline-tools.zip");
    cmd.output()?;
    // unzipping platform-tools
    let mut cmd = Command::new("unzip");
    cmd.arg("platform-tools.zip");
    cmd.output()?;
    // remove the unzip files
    let mut cmd = Command::new("rm");
    cmd.arg("cmdline-tools.zip");
    cmd.arg("platform-tools.zip");
    cmd.output()?;
    return Ok(());
}

fn set_adb_path() {
    // check if adb_cmdtools folder exist
    let home = dirs::home_dir();
    if let Some(path) = home {
        let adbpath = path.join("adb_cmdtools");
        if !adbpath.exists() {
            println!("tools is download. use {} to download it.", DOWNLOAD_TOOLS);
            return;
        }
        let path_of_cmdline = path
            .join("cmdline-tools")
            .join("bin")
            .into_os_string()
            .into_string()
            .unwrap();
        let path_of_platform = path
            .join("platform-tools")
            .into_os_string()
            .into_string()
            .unwrap();
        let result = std::env::var("PATH");
        match result {
            Ok(path) => {
                let path = format!("{}:{}:{}", path, path_of_cmdline, path_of_platform);
                std::env::set_var("PATH", path);
            }
            Err(error) => {
                println!("{}", error);
            }
        }
    } else {
        println!("home dir not found!!!");
    }
}
